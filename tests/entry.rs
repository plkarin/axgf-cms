//! The ways in (v0.5.0, docs/adr/0012): registration under each policy, the
//! sign-in services against a provider of the suite's own on loopback, the
//! secret asked after a service, linking by a verified address, and the rule
//! that keeps every administrator from being locked out.

mod common;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use base64::Engine as _;
use common::*;
use serde_json::{json, Value};
use tower::ServiceExt;

fn sample() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("deploy/sample.axgf")
}

fn enc(v: &str) -> String {
    v.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

fn form(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{k}={}", enc(v)))
        .collect::<Vec<_>>()
        .join("&")
}

fn query_of(url: &str) -> HashMap<String, String> {
    let q = url.split_once('?').map(|(_, q)| q).unwrap_or("");
    q.split('&')
        .filter_map(|p| p.split_once('='))
        .map(|(k, v)| {
            let mut out = Vec::new();
            let b = v.as_bytes();
            let mut i = 0;
            while i < b.len() {
                if b[i] == b'%' && i + 2 < b.len() {
                    out.push(
                        u8::from_str_radix(std::str::from_utf8(&b[i + 1..i + 3]).unwrap(), 16)
                            .unwrap(),
                    );
                    i += 3;
                } else {
                    out.push(if b[i] == b'+' { b' ' } else { b[i] });
                    i += 1;
                }
            }
            (k.to_string(), String::from_utf8(out).unwrap())
        })
        .collect()
}

fn set_cookies(resp: &axum::response::Response) -> Vec<String> {
    resp.headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|v| v.to_str().unwrap().to_string())
        .collect()
}

fn cookie_named(resp: &axum::response::Response, name: &str) -> Option<String> {
    set_cookies(resp).into_iter().find_map(|c| {
        let first = c.split(';').next().unwrap().to_string();
        first
            .strip_prefix(&format!("{name}="))
            .filter(|v| !v.is_empty())
            .map(|_| first.clone())
    })
}

// ---------------------------------------------------------------------------
// A provider of the suite's own
// ---------------------------------------------------------------------------

/// What the provider will say about the next sign-in, and what it saw.
#[derive(Default)]
struct Provider {
    /// code → (claims to sign, PKCE challenge it was issued for)
    codes: HashMap<String, (Value, String)>,
}

struct Fake {
    base: String,
    shared: Arc<Mutex<Provider>>,
}

async fn fake_provider() -> Fake {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let shared = Arc::new(Mutex::new(Provider::default()));
    let doc = json!({
        "issuer": base,
        "authorization_endpoint": format!("{base}/authorize"),
        "token_endpoint": format!("{base}/token"),
    });
    let s2 = shared.clone();
    let app = axum::Router::new()
        .route(
            "/.well-known/openid-configuration",
            axum::routing::get(move || {
                let doc = doc.clone();
                async move { axum::Json(doc) }
            }),
        )
        .route(
            "/token",
            axum::routing::post(move |axum::Form(f): axum::Form<HashMap<String, String>>| {
                let s2 = s2.clone();
                async move {
                    let code = f.get("code").cloned().unwrap_or_default();
                    let entry = s2.lock().unwrap().codes.remove(&code);
                    let Some((claims, challenge)) = entry else {
                        return axum::Json(json!({ "error": "invalid_grant", "error_description": "unknown code" }));
                    };
                    let verifier = f.get("code_verifier").cloned().unwrap_or_default();
                    if axgf_cms::sso::pkce_challenge(&verifier) != challenge {
                        return axum::Json(json!({ "error": "invalid_grant", "error_description": "PKCE" }));
                    }
                    if f.get("client_secret").map(String::as_str) != Some("s3cret-value") {
                        return axum::Json(json!({ "error": "invalid_client" }));
                    }
                    let b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD;
                    let jwt = format!(
                        "{}.{}.{}",
                        b64.encode(r#"{"alg":"RS256","typ":"JWT"}"#),
                        b64.encode(claims.to_string()),
                        b64.encode("not-checked-here")
                    );
                    axum::Json(json!({ "access_token": "at", "token_type": "Bearer", "id_token": jwt }))
                }
            }),
        );
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    Fake { base, shared }
}

/// Set the site up for the fake provider, as an administrator would.
async fn configure(app: &axum::Router, admin: &str, fake: &Fake, registration: &str) {
    let r = post_form_as(
        app,
        admin,
        "/admin/connectivity/address",
        &form(&[("public_url", "https://family.example")]),
    )
    .await;
    assert_eq!(r.status(), StatusCode::OK);
    let r = post_form_as(
        app,
        admin,
        "/admin/connectivity/provider/oidc",
        &form(&[
            ("enabled", "on"),
            ("client_id", "axgf-test"),
            ("client_secret", "s3cret-value"),
            ("issuer", &fake.base),
            ("label", "Family sign-in"),
        ]),
    )
    .await;
    let status = r.status();
    assert_eq!(status, StatusCode::OK, "{}", body_string(r).await);
    let r = post_form_as(
        app,
        admin,
        "/admin/users/registration",
        &form(&[("mode", registration)]),
    )
    .await;
    assert!(r.status().is_redirection());
}

struct Started {
    flow_cookie: String,
    state: String,
    nonce: String,
    challenge: String,
}

async fn start(app: &axum::Router, intent: &str, secret: &str) -> Started {
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/auth/oidc/start")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(form(&[
                    ("intent", intent),
                    ("next", "/"),
                    ("secret", secret),
                ])))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(resp.status().is_redirection(), "{}", resp.status());
    let location = resp.headers()[header::LOCATION]
        .to_str()
        .unwrap()
        .to_string();
    let q = query_of(&location);
    assert_eq!(q["response_type"], "code");
    assert_eq!(q["client_id"], "axgf-test");
    assert_eq!(
        q["redirect_uri"],
        "https://family.example/auth/oidc/callback"
    );
    assert_eq!(q["code_challenge_method"], "S256");
    let flow_cookie = cookie_named(&resp, "axgf_sso").expect("a flow cookie");
    let raw = set_cookies(&resp).join("\n");
    assert!(
        raw.contains("SameSite=Lax") && raw.contains("HttpOnly"),
        "{raw}"
    );
    Started {
        flow_cookie,
        state: q["state"].clone(),
        nonce: q["nonce"].clone(),
        challenge: q["code_challenge"].clone(),
    }
}

fn claims(fake: &Fake, nonce: &str, sub: &str, email: Option<(&str, bool)>) -> Value {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let mut c = json!({
        "iss": fake.base, "aud": "axgf-test", "sub": sub, "nonce": nonce,
        "iat": now, "exp": now + 300, "name": "Ada Example",
    });
    if let Some((e, verified)) = email {
        c["email"] = json!(e);
        c["email_verified"] = json!(verified);
    }
    c
}

async fn come_back(
    app: &axum::Router,
    fake: &Fake,
    s: &Started,
    code: &str,
    claims: Value,
) -> axum::response::Response {
    fake.shared
        .lock()
        .unwrap()
        .codes
        .insert(code.to_string(), (claims, s.challenge.clone()));
    app.clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/auth/oidc/callback?code={code}&state={}",
                    enc(&s.state)
                ))
                .header(header::COOKIE, s.flow_cookie.clone())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
}

fn acl_of(bundle: &std::path::Path) -> Value {
    // No file until the first account or setting is saved.
    match std::fs::read_to_string(bundle.with_extension("acl")) {
        Ok(text) => serde_json::from_str(&text).unwrap(),
        Err(_) => json!({ "users": [] }),
    }
}

fn user_named<'a>(acl: &'a Value, name: &str) -> Option<&'a Value> {
    acl["users"]
        .as_array()
        .unwrap()
        .iter()
        .find(|u| u["username"] == name)
}

// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread")]
async fn a_visitor_opens_a_members_account_through_a_service_in_one_click() {
    let fake = fake_provider().await;
    let (app, bundle) = app_with_bundle("entry-open", &sample());
    let admin = admin_cookie(&app).await;
    configure(&app, &admin, &fake, "open").await;

    // The registration page offers the service first, then the form.
    let html = body_string(get_with_cookie(&app, "/register", "axgf_lang=en").await).await;
    assert!(html.contains("Continue with Family sign-in"), "{html}");
    assert!(html.contains(r#"formaction="/auth/oidc/start""#));
    assert!(html.contains(r#"action="/register""#));

    let s = start(&app, "register", "").await;
    let resp = come_back(
        &app,
        &fake,
        &s,
        "c1",
        claims(
            &fake,
            &s.nonce,
            "subject-1",
            Some(("ada@family.example", true)),
        ),
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "a page that moves on by itself, not a redirect"
    );
    let session = cookie_named(&resp, "axgf_session").expect("signed in");
    let html = body_string(resp).await;
    assert!(
        html.contains(r#"http-equiv="refresh" content="0;url=/""#),
        "{html}"
    );

    let acl = acl_of(&bundle);
    let ada = user_named(&acl, "ada").expect("an account named from the address");
    assert_eq!(ada["role"], "viewer", "a member's account");
    assert_eq!(ada["created_via"], "sso:oidc");
    assert_eq!(
        ada["email"], "ada@family.example",
        "a verified address is kept"
    );
    assert_eq!(ada["identities"][0]["subject"], "subject-1");
    assert_eq!(ada["password_hash"], "!no-password");

    // The session is real.
    let r = get_with_cookie(&app, "/settings", &session).await;
    assert_eq!(r.status(), StatusCode::OK);

    // The same person again: the same account, not a second one.
    let s = start(&app, "signin", "").await;
    let resp = come_back(
        &app,
        &fake,
        &s,
        "c2",
        claims(
            &fake,
            &s.nonce,
            "subject-1",
            Some(("ada@family.example", true)),
        ),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let acl = acl_of(&bundle);
    assert_eq!(
        acl["users"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|u| u["created_via"] == "sso:oidc")
            .count(),
        1
    );

    // The administrator sees how it signs in.
    let html =
        body_string(get_with_cookie(&app, "/admin/users", &format!("{admin}; axgf_lang=en")).await)
            .await;
    assert!(
        html.contains("single sign-on"),
        "the service is named on the account"
    );
    assert!(html.contains("opened through a service"));
}

#[tokio::test(flavor = "multi_thread")]
async fn an_answer_this_browser_did_not_ask_for_is_refused() {
    let fake = fake_provider().await;
    let (app, bundle) = app_with_bundle("entry-stale", &sample());
    let admin = admin_cookie(&app).await;
    configure(&app, &admin, &fake, "open").await;
    let s = start(&app, "register", "").await;

    // Another browser's cookie.
    let other = Started {
        flow_cookie: "axgf_sso=someone-elses".into(),
        ..s
    };
    let resp = come_back(
        &app,
        &fake,
        &other,
        "c1",
        claims(&fake, &other.nonce, "x", None),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert!(cookie_named(&resp, "axgf_session").is_none());

    // A token carrying another flow's nonce, or made for another client.
    let s = start(&app, "register", "").await;
    let resp = come_back(
        &app,
        &fake,
        &s,
        "c2",
        claims(&fake, "not-the-nonce", "x", None),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    let s = start(&app, "register", "").await;
    let mut c = claims(&fake, &s.nonce, "x", None);
    c["aud"] = json!("another-client");
    let resp = come_back(&app, &fake, &s, "c3", c).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // A flow is spent once.
    let s = start(&app, "register", "").await;
    let ok = come_back(&app, &fake, &s, "c4", claims(&fake, &s.nonce, "once", None)).await;
    assert_eq!(ok.status(), StatusCode::OK);
    let replay = come_back(&app, &fake, &s, "c5", claims(&fake, &s.nonce, "once", None)).await;
    assert_eq!(replay.status(), StatusCode::BAD_REQUEST);
    let acl = acl_of(&bundle);
    assert_eq!(
        acl["users"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|u| u["created_via"] == "sso:oidc")
            .count(),
        1
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn under_the_secret_a_services_user_is_asked_for_it_before_anything_is_created() {
    let fake = fake_provider().await;
    let (app, bundle) = app_with_bundle("entry-secret", &sample());
    let admin = admin_cookie(&app).await;
    configure(&app, &admin, &fake, "secret").await;
    let secret = acl_of(&bundle)["site"]["registration"]["secret"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(secret.len() >= 20, "a secret was drawn: {secret}");

    // A wrong secret on the way out stops it there.
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/auth/oidc/start")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, "axgf_lang=en")
                .body(Body::from(form(&[
                    ("intent", "register"),
                    ("secret", "guess"),
                ])))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // None given: the service vouches, then the secret is asked.
    let s = start(&app, "register", "").await;
    let resp = come_back(
        &app,
        &fake,
        &s,
        "c1",
        claims(
            &fake,
            &s.nonce,
            "subject-9",
            Some(("bo@family.example", true)),
        ),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(
        cookie_named(&resp, "axgf_session").is_none(),
        "nobody signed in yet"
    );
    let pending = cookie_named(&resp, "axgf_reg").expect("held for the secret");
    assert!(body_string(resp).await.contains("/register/finish"));
    assert!(
        user_named(&acl_of(&bundle), "bo").is_none(),
        "nothing created yet"
    );

    let cookie = format!("{pending}; axgf_lang=en");
    let html = body_string(get_with_cookie(&app, "/register/finish", &cookie).await).await;
    assert!(html.contains("bo@family.example"));
    let wrong = post_form_as(
        &app,
        &cookie,
        "/register/finish",
        &form(&[("secret", "nope")]),
    )
    .await;
    assert_eq!(wrong.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let right = post_form_as(
        &app,
        &cookie,
        "/register/finish",
        &form(&[("secret", &secret)]),
    )
    .await;
    assert_eq!(right.status(), StatusCode::OK);
    assert!(cookie_named(&right, "axgf_session").is_some());
    assert_eq!(
        user_named(&acl_of(&bundle), "bo").unwrap()["role"],
        "viewer"
    );
    // Spent.
    let again = post_form_as(
        &app,
        &cookie,
        "/register/finish",
        &form(&[("secret", &secret)]),
    )
    .await;
    assert_eq!(again.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_verified_address_links_the_account_an_administrator_made_and_nothing_else_does() {
    let fake = fake_provider().await;
    let (app, bundle) = app_with_bundle("entry-link", &sample());
    let admin = admin_cookie(&app).await;
    configure(&app, &admin, &fake, "closed").await;
    let r = post_form_as(
        &app,
        &admin,
        "/admin/users",
        &form(&[
            ("username", "grandma"),
            ("password", "correct-horse-battery"),
            ("email", "Gran@Family.example"),
            ("role", "contributor"),
        ]),
    )
    .await;
    assert_eq!(r.status(), StatusCode::OK);

    // An address the service did not verify links nothing, and a closed site
    // opens nothing.
    let s = start(&app, "signin", "").await;
    let resp = come_back(
        &app,
        &fake,
        &s,
        "c1",
        claims(&fake, &s.nonce, "g-1", Some(("gran@family.example", false))),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    assert!(user_named(&acl_of(&bundle), "grandma")
        .unwrap()
        .get("identities")
        .is_none());

    // A verified one does: her account, her role.
    let s = start(&app, "signin", "").await;
    let resp = come_back(
        &app,
        &fake,
        &s,
        "c2",
        claims(&fake, &s.nonce, "g-1", Some(("gran@family.example", true))),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(cookie_named(&resp, "axgf_session").is_some());
    let acl = acl_of(&bundle);
    let gran = user_named(&acl, "grandma").unwrap();
    assert_eq!(gran["identities"][0]["subject"], "g-1");
    assert_eq!(gran["role"], "contributor");
    // The emergency session that set this up is not an account: grandma is
    // the only one, and still the only one.
    assert_eq!(
        acl["users"].as_array().unwrap().len(),
        1,
        "no second account"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn the_registration_form_follows_the_policy_and_is_throttled() {
    let (app, bundle) = app_with_bundle("entry-form", &sample());
    let admin = admin_cookie(&app).await;
    let reg = |body: String| {
        let app = app.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .uri("/register")
                    .method("POST")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header(header::COOKIE, "axgf_lang=en")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap()
        }
    };

    // Closed by default: the page says so, and the form posts nothing.
    let r = get(&app, "/register").await;
    assert_eq!(r.status(), StatusCode::FORBIDDEN);
    let r = reg(form(&[
        ("username", "eve"),
        ("password", "long-enough-password"),
    ]))
    .await;
    assert_eq!(r.status(), StatusCode::FORBIDDEN);
    assert!(user_named(&acl_of(&bundle), "eve").is_none());

    // Open: a member, signed in at once.
    post_form_as(
        &app,
        &admin,
        "/admin/users/registration",
        &form(&[("mode", "open")]),
    )
    .await;
    let r = reg(form(&[
        ("username", "Cleo"),
        ("password", "long-enough-password"),
    ]))
    .await;
    assert_eq!(r.status(), StatusCode::SEE_OTHER);
    assert!(cookie_named(&r, "axgf_session").is_some());
    let cleo = user_named(&acl_of(&bundle), "cleo")
        .cloned()
        .expect("created, lower-cased");
    assert_eq!(cleo["role"], "viewer");
    assert_eq!(cleo["created_via"], "register");
    assert!(
        cleo.get("email").is_none(),
        "no address is taken from a form"
    );
    let r = reg(form(&[
        ("username", "cleo"),
        ("password", "long-enough-password"),
    ]))
    .await;
    assert_eq!(r.status(), StatusCode::UNPROCESSABLE_ENTITY, "taken");
    let r = reg(form(&[("username", "dan"), ("password", "short")])).await;
    assert_eq!(r.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // Secret: only with it.
    post_form_as(
        &app,
        &admin,
        "/admin/users/registration",
        &form(&[("mode", "secret")]),
    )
    .await;
    let secret = acl_of(&bundle)["site"]["registration"]["secret"]
        .as_str()
        .unwrap()
        .to_string();
    let r = reg(form(&[
        ("username", "dan"),
        ("password", "long-enough-password"),
        ("secret", "wrong"),
    ]))
    .await;
    assert_eq!(r.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(body_string(r)
        .await
        .contains("That is not the family&#x27;s secret."));
    let r = reg(form(&[
        ("username", "dan"),
        ("password", "long-enough-password"),
        ("secret", &secret),
    ]))
    .await;
    assert_eq!(r.status(), StatusCode::SEE_OTHER);

    // Rotated: the old one shuts out.
    post_form_as(
        &app,
        &admin,
        "/admin/users/registration",
        &form(&[("mode", "secret"), ("rotate", "1")]),
    )
    .await;
    let r = reg(form(&[
        ("username", "fay"),
        ("password", "long-enough-password"),
        ("secret", &secret),
    ]))
    .await;
    assert_eq!(r.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // Every attempt counts: the eighth is the last.
    let mut last = StatusCode::OK;
    for i in 0..10 {
        last = reg(form(&[
            ("username", &format!("g{i}x")),
            ("password", "long-enough-password"),
            ("secret", "x"),
        ]))
        .await
        .status();
    }
    assert_eq!(last, StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test(flavor = "multi_thread")]
async fn no_policy_may_leave_every_administrator_without_a_way_in() {
    let fake = fake_provider().await;
    let (app, _bundle) = app_with_bundle("entry-lockout", &sample());
    let admin = admin_cookie(&app).await;
    configure(&app, &admin, &fake, "closed").await;
    let admin_en = format!("{admin}; axgf_lang=en");

    // An administrator account with a password and an address.
    let r = post_form_as(
        &app,
        &admin,
        "/admin/users",
        &form(&[
            ("username", "root2"),
            ("password", "correct-horse-battery"),
            ("email", "root2@family.example"),
            ("role", "admin"),
        ]),
    )
    .await;
    assert_eq!(r.status(), StatusCode::OK);

    // No administrator has a linked service: refused.
    let r = post_form_as(
        &app,
        &admin_en,
        "/admin/connectivity/policy",
        &form(&[("password", "non-admins"), ("link_by_email", "on")]),
    )
    .await;
    assert_eq!(r.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(body_string(r)
        .await
        .contains("no active administrator could sign in"));

    // Once one has, it is accepted …
    let s = start(&app, "signin", "").await;
    let resp = come_back(
        &app,
        &fake,
        &s,
        "c1",
        claims(
            &fake,
            &s.nonce,
            "root-sub",
            Some(("root2@family.example", true)),
        ),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let r = post_form_as(
        &app,
        &admin_en,
        "/admin/connectivity/policy",
        &form(&[("password", "non-admins"), ("link_by_email", "on")]),
    )
    .await;
    assert_eq!(r.status(), StatusCode::OK);

    // … the administrator's password no longer opens anything …
    let login = |user: &'static str| {
        let app = app.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .uri("/admin/login")
                    .method("POST")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header(header::COOKIE, "axgf_lang=en")
                    .body(Body::from(format!(
                        "username={user}&password=correct-horse-battery"
                    )))
                    .unwrap(),
            )
            .await
            .unwrap()
        }
    };
    let r = login("root2").await;
    assert_eq!(r.status(), StatusCode::UNAUTHORIZED);
    assert!(body_string(r).await.contains("sign in through a service"));

    // … and disabling the only linked service is refused in turn.
    let r = post_form_as(
        &app,
        &admin_en,
        "/admin/connectivity/provider/oidc",
        &form(&[("client_id", "axgf-test"), ("issuer", &fake.base)]),
    )
    .await;
    assert_eq!(r.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test(flavor = "multi_thread")]
async fn the_connectivity_page_never_shows_a_providers_secret() {
    let fake = fake_provider().await;
    let (app, _bundle) = app_with_bundle("entry-page", &sample());
    let admin = admin_cookie(&app).await;
    configure(&app, &admin, &fake, "open").await;
    let html = body_string(
        get_with_cookie(
            &app,
            "/admin/connectivity",
            &format!("{admin}; axgf_lang=en"),
        )
        .await,
    )
    .await;
    assert!(html
        .contains(r#"<a class="admin-nav-item" href="/admin/connectivity" aria-current="page">"#));
    assert!(html.contains("A secret is set. Leave this empty to keep it."));
    assert!(!html.contains("s3cret-value"), "never shown again");
    // Escaped as every value in a page is: `/` is `&#x2f;`.
    assert!(
        html.contains("https:&#x2f;&#x2f;family.example&#x2f;auth&#x2f;oidc&#x2f;callback"),
        "the redirect address to register"
    );
    // Saving without a secret keeps the one set.
    let r = post_form_as(
        &app,
        &admin,
        "/admin/connectivity/provider/oidc",
        &form(&[
            ("enabled", "on"),
            ("client_id", "axgf-test"),
            ("issuer", &fake.base),
            ("label", "Family sign-in"),
        ]),
    )
    .await;
    assert_eq!(r.status(), StatusCode::OK);
    // The test button reaches the provider's discovery document.
    let r = post_form_as(
        &app,
        &format!("{admin}; axgf_lang=en"),
        "/admin/connectivity/test/oidc",
        "",
    )
    .await;
    let html = body_string(r).await;
    assert!(
        html.contains("Family sign-in answered"),
        "{}",
        &html[..html.len().min(600)]
    );
    // And the sign-in page offers it.
    let html = body_string(get_with_cookie(&app, "/admin/login", "axgf_lang=en").await).await;
    assert!(html.contains("Continue with Family sign-in"));
    assert!(html.contains(r#"href="/register""#));
}
