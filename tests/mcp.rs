//! The MCP (docs/adr/0045 §3–4): closed until an administrator opens it;
//! then an account's token, made on its own page and shown once, lets an AI
//! assistant elsewhere read and change the archive as that account, over
//! JSON-RPC posted to `/mcp` — with the token in `Authorization`, or in the
//! address — through the same operations as the AFG's commands.

mod common;

use std::path::{Path, PathBuf};

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::{json, Value};
use tower::ServiceExt;

const PASSWORD: &str = "correct-horse-battery-staple";

fn sample() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("deploy/sample.axgf")
}

/// An app over the sample with a contributor (vera) and a viewer (paul);
/// the administrator's cookie, and theirs.
async fn app(tag: &str) -> (axum::Router, Scratch, String, String, String) {
    use axgf_cms::acl::{new_user, Acl, Role};
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    std::fs::copy(sample(), &path).unwrap();
    let mut acl = Acl::default();
    for (name, role) in [("vera", Role::Contributor), ("paul", Role::Viewer)] {
        acl.users.push(new_user(name, PASSWORD, role).unwrap());
    }
    acl.save(&Acl::path_for(&path)).unwrap();
    let app = build_app(&path).unwrap();
    let admin = admin_cookie(&app).await;
    let mut cookies = Vec::new();
    for name in ["vera", "paul"] {
        let login = post_form(
            &app,
            "/admin/login",
            &format!("username={name}&password={PASSWORD}"),
            false,
        )
        .await;
        cookies.push(
            login.headers()[header::SET_COOKIE]
                .to_str()
                .unwrap()
                .split(';')
                .next()
                .unwrap()
                .to_string(),
        );
    }
    let paul = cookies.pop().unwrap();
    let vera = cookies.pop().unwrap();
    (app, dir.pointing_at(path), admin, vera, paul)
}

/// A token made on the account's page, as the page shows it once.
async fn token(app: &axum::Router, cookie: &str, label: &str, write: bool) -> String {
    let body = format!("label={label}{}", if write { "&write=1" } else { "" });
    let resp = post_form_as(app, cookie, "/account/tokens", &body).await;
    assert_eq!(resp.status(), StatusCode::OK, "a token is made");
    let html = body_string(resp).await;
    let at = html.find("axgf_").expect("the token, shown once");
    html[at..at + html[at..].find('"').unwrap()].to_string()
}

async fn rpc(
    app: &axum::Router,
    auth: Option<&str>,
    path: &str,
    body: Value,
) -> (StatusCode, Value) {
    let mut req = Request::builder()
        .uri(path)
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "application/json, text/event-stream");
    if let Some(t) = auth {
        req = req.header(header::AUTHORIZATION, format!("Bearer {t}"));
    }
    let resp = app
        .clone()
        .oneshot(req.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let text = body_string(resp).await;
    (status, serde_json::from_str(&text).unwrap_or(Value::Null))
}

async fn call(app: &axum::Router, t: &str, tool: &str, args: Value) -> Value {
    let (status, v) = rpc(
        app,
        Some(t),
        "/mcp",
        json!({"jsonrpc": "2.0", "id": 7, "method": "tools/call",
               "params": {"name": tool, "arguments": args}}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{v}");
    v["result"].clone()
}

fn text(result: &Value) -> String {
    result["content"][0]["text"]
        .as_str()
        .unwrap_or_default()
        .to_string()
}

#[tokio::test]
async fn the_mcp_is_closed_until_an_administrator_opens_it() {
    let (app, _dir, admin, vera, _) = app("mcp-closed").await;
    let (status, _) = rpc(
        &app,
        None,
        "/mcp",
        json!({"jsonrpc": "2.0", "id": 1, "method": "ping"}),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let page = body_string(get_with_cookie(&app, "/account", &vera).await).await;
    assert!(page.contains(r#"id="account-tokens""#));
    assert!(page.contains("The MCP of this site is closed"));
    let resp = post_form_as(&app, &vera, "/account/tokens", "label=ChatGPT").await;
    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "no token while it is closed"
    );
    let resp = post_form_as(&app, &vera, "/admin/afg/mcp", "open=1").await;
    assert_ne!(
        resp.status(),
        StatusCode::OK,
        "only an administrator opens it"
    );
    let resp = post_form_as(&app, &admin, "/admin/afg/mcp", "open=1").await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(body_string(resp).await.contains("The MCP is open"));
    let (status, v) = rpc(
        &app,
        None,
        "/mcp",
        json!({"jsonrpc": "2.0", "id": 1, "method": "ping"}),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "{v}");
    assert_eq!(
        get(&app, "/mcp").await.status(),
        StatusCode::METHOD_NOT_ALLOWED,
        "no stream is offered"
    );
}

#[tokio::test]
async fn a_token_reads_and_writes_as_its_account() {
    let (app, dir, admin, vera, _) = app("mcp-write").await;
    post_form_as(&app, &admin, "/admin/afg/mcp", "open=1").await;
    let t = token(&app, &vera, "Claude+portable", true).await;

    let (status, v) = rpc(
        &app,
        Some(&t),
        "/mcp",
        json!({"jsonrpc": "2.0", "id": 1, "method": "initialize",
               "params": {"protocolVersion": "2025-06-18", "capabilities": {},
                          "clientInfo": {"name": "test", "version": "1"}}}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(v["result"]["protocolVersion"], "2025-06-18");
    assert_eq!(v["result"]["serverInfo"]["name"], "ax-genealogy");
    assert!(v["result"]["capabilities"]["tools"].is_object());
    // A notification is accepted, and answered with nothing.
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/mcp")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {t}"))
                .body(Body::from(
                    json!({"jsonrpc": "2.0", "method": "notifications/initialized"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::ACCEPTED);

    let (_, v) = rpc(
        &app,
        Some(&t),
        "/mcp",
        json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"}),
    )
    .await;
    let names: Vec<&str> = v["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    for tool in [
        "search_people",
        "get_person",
        "create_person",
        "apply_operations",
        "guide",
    ] {
        assert!(names.contains(&tool), "{tool} in {names:?}");
    }
    assert!(!names.contains(&"delete_person"), "an administrator's only");

    // A person, written as the account, journalled as the token's.
    let made = call(
        &app,
        &t,
        "create_person",
        json!({"given": "Mcpa", "family": "Fictive", "gender": "F", "living": false,
               "birth": {"date": "1902", "circa": true}}),
    )
    .await;
    assert_eq!(made["isError"], false, "{made}");
    assert!(
        text(&made).starts_with("Applied: 1 record(s) written."),
        "{made}"
    );
    let found = call(&app, &t, "search_people", json!({"query": "mcpa fictive"})).await;
    let v: Value = serde_json::from_str(&text(&found)).unwrap();
    assert_eq!(v["total"], 1, "{v}");
    let id = v["people"][0]["id"].as_str().unwrap().to_string();
    let person = call(&app, &t, "get_person", json!({"person": id})).await;
    let p: Value = serde_json::from_str(&text(&person)).unwrap();
    assert_eq!(p["birth"]["date"], "1902");
    assert_eq!(p["birth"]["circa"], true);

    // A batch, whole or not at all: its second operation fails, nothing is
    // written.
    let refused = call(
        &app,
        &t,
        "apply_operations",
        json!({"operations": [
            {"op": "set_death", "person": id, "date": "1980"},
            {"op": "set_birth", "person": id, "date": "the spring"}
        ]}),
    )
    .await;
    assert_eq!(refused["isError"], true);
    assert!(text(&refused).contains("Operation 2:"), "{refused}");
    let person = call(&app, &t, "get_person", json!({"person": id})).await;
    let p: Value = serde_json::from_str(&text(&person)).unwrap();
    assert!(p["death"].is_null(), "nothing of the batch was written");

    // The same, through the address with the token in it.
    let (status, v) = rpc(
        &app,
        None,
        &format!("/mcp/t/{t}"),
        json!({"jsonrpc": "2.0", "id": 3, "method": "tools/call",
               "params": {"name": "set_death", "arguments": {"person": id, "date": "1980-04-02"}}}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(v["result"]["isError"], false, "{v}");

    let journal = axgf_cms::journal::Journal::new(axgf_cms::journal::Journal::path_for(
        &dir.dir().join("family.axgf"),
    ));
    let whos: Vec<String> = journal.read_all().iter().map(|e| e.who.clone()).collect();
    assert!(
        whos.iter().any(|w| w == "vera · MCP · Claude portable"),
        "{whos:?}"
    );

    // The account's page lists it, used today; revoked, it opens nothing.
    let page = body_string(get_with_cookie(&app, "/account", &vera).await).await;
    assert!(page.contains("Claude portable") && page.contains("reads and writes"));
    assert!(!page.contains(&t), "the token is never shown again");
    let id = &t[5..13];
    let resp = post_form_as(&app, &vera, &format!("/account/tokens/{id}/revoke"), "").await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let (status, _) = rpc(
        &app,
        Some(&t),
        "/mcp",
        json!({"jsonrpc": "2.0", "id": 4, "method": "ping"}),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn a_reading_token_and_a_viewers_token_write_nothing() {
    let (app, _dir, admin, vera, paul) = app("mcp-read").await;
    post_form_as(&app, &admin, "/admin/afg/mcp", "open=1").await;
    let reading = token(&app, &vera, "Lecture", false).await;
    let viewers = token(&app, &paul, "Paul", true).await;
    for t in [&reading, &viewers] {
        let (_, v) = rpc(
            &app,
            Some(t),
            "/mcp",
            json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
        )
        .await;
        let names: Vec<&str> = v["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert!(names.contains(&"search_people") && !names.contains(&"create_person"));
        let r = call(&app, t, "create_person", json!({"given": "Nope"})).await;
        assert_eq!(r["isError"], true);
        let r = call(&app, t, "search_people", json!({"query": "nope"})).await;
        assert_eq!(r["isError"], false, "it reads");
    }
    let (status, v) = rpc(
        &app,
        Some("axgf_00000000_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
        "/mcp",
        json!({"jsonrpc": "2.0", "id": 1, "method": "ping"}),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "{v}");
}

#[tokio::test]
async fn an_unknown_method_and_a_foreign_page_are_refused() {
    let (app, _dir, admin, vera, _) = app("mcp-refusals").await;
    post_form_as(&app, &admin, "/admin/afg/mcp", "open=1").await;
    let t = token(&app, &vera, "X", true).await;
    let (_, v) = rpc(
        &app,
        Some(&t),
        "/mcp",
        json!({"jsonrpc": "2.0", "id": 9, "method": "sampling/createMessage"}),
    )
    .await;
    assert_eq!(v["error"]["code"], -32601);
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/mcp")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::ORIGIN, "https://evil.example")
                .header(header::AUTHORIZATION, format!("Bearer {t}"))
                .body(Body::from(
                    json!({"jsonrpc": "2.0", "id": 1, "method": "ping"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "a page elsewhere is refused"
    );
}

#[tokio::test]
async fn the_community_connector_is_served_packed_and_agrees_with_the_mcp() {
    let (app, _dir, admin, vera, _) = app("mcp-connector").await;
    let resp = get(&app, "/mcp/ax-genealogy.mcpb").await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.headers()[header::CONTENT_TYPE], "application/zip");
    let bytes = body_bytes(resp).await;
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("a ZIP");
    let mut names: Vec<String> = (0..zip.len())
        .map(|i| zip.by_index(i).unwrap().name().unwrap().to_string())
        .collect();
    names.sort();
    assert_eq!(
        names,
        ["README.md", "icon.png", "manifest.json", "server.mjs"]
    );
    let manifest: Value = {
        let mut s = String::new();
        std::io::Read::read_to_string(&mut zip.by_name("manifest.json").unwrap(), &mut s).unwrap();
        serde_json::from_str(&s).unwrap()
    };
    assert_eq!(
        manifest["version"],
        env!("CARGO_PKG_VERSION"),
        "this release's"
    );
    assert_eq!(
        manifest["server"]["mcp_config"]["env"]["AXGF_TOKEN"],
        "${user_config.token}"
    );
    assert_eq!(manifest["user_config"]["token"]["sensitive"], true);

    // The tools it announces are the MCP's.
    post_form_as(&app, &admin, "/admin/afg/mcp", "open=1").await;
    let t = token(&app, &vera, "Desktop", true).await;
    let (_, v) = rpc(
        &app,
        Some(&t),
        "/mcp",
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
    )
    .await;
    let mut served: Vec<String> = v["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap().to_string())
        .collect();
    served.push("delete_person".into()); // an administrator's
    served.sort();
    let mut announced: Vec<String> = manifest["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap().to_string())
        .collect();
    announced.sort();
    assert_eq!(announced, served);
}
