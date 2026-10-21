//! Signing in through an identity provider (v0.5.0, docs/adr/0012 §2).
//!
//! # What is spoken
//!
//! OpenID Connect's authorization-code flow, with PKCE, for Google, Microsoft
//! and any OpenID Connect provider an operator names by its issuer; and
//! GitHub's OAuth 2 flow, which is the same shape without the ID token, read
//! through GitHub's own user API instead.
//!
//! # What is checked, and what is deliberately not
//!
//! * **`state`** binds the provider's answer to the browser that asked: it is
//!   kept on the server and in a cookie, and both must agree. A callback that
//!   arrives with someone else's code is refused.
//! * **PKCE** binds the code to this server's request, so a code intercepted
//!   on its way back is worth nothing to anyone else.
//! * **`nonce`** binds the ID token to the same request.
//! * **The ID token's `iss`, `aud`, `exp` and `nonce`** are checked. Its
//!   *signature* is not: the token comes straight from the provider's token
//!   endpoint, over TLS, in answer to this server's own authenticated request,
//!   and OpenID Connect Core §3.1.3.7 says that is the case in which the TLS
//!   validation of the endpoint may stand in for the signature check. That is
//!   what keeps a JOSE stack and a key cache out of this binary.
//!
//! # Network
//!
//! The provider is reached from this process (discovery, and the code's
//! exchange), so the service needs outbound HTTPS. The systemd unit
//! bootstrap.sh writes denies every address but the local network; the
//! Connectivity page says so when a provider cannot be reached, and how to
//! allow it (docs/adr/0012 §6).

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use base64::Engine as _;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// How long a sign-in may take between leaving for the provider and coming
/// back. Ten minutes is a slow typist with a second factor.
pub const FLOW_TTL: Duration = Duration::from_secs(10 * 60);
/// How long a discovered provider's endpoints are trusted before asking again.
const DISCOVERY_TTL: Duration = Duration::from_secs(60 * 60);
/// The most flows held at once. A flood of `start` requests from one client
/// costs a map entry each; past this, the oldest are dropped.
const MAX_PENDING: usize = 512;
/// One budget for each exchange with a provider.
const TIMEOUT: Duration = Duration::from_secs(10);
/// The cookie carrying a flow's `state` back from the provider.
pub const FLOW_COOKIE: &str = "axgf_sso";
/// The cookie carrying a provider's user to the secret question.
pub const PENDING_COOKIE: &str = "axgf_reg";

/// The providers this build speaks to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Google,
    Microsoft,
    Github,
    /// Any OpenID Connect provider, by its issuer: Keycloak, Authentik,
    /// Okta, GitLab, a family's own.
    Oidc,
}

impl Kind {
    pub const ALL: [Kind; 4] = [Kind::Google, Kind::Microsoft, Kind::Github, Kind::Oidc];

    /// The wire form, in URLs, forms and the `.acl`.
    pub fn slug(self) -> &'static str {
        match self {
            Kind::Google => "google",
            Kind::Microsoft => "microsoft",
            Kind::Github => "github",
            Kind::Oidc => "oidc",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.slug() == s)
    }

    /// The provider's own name, which is a proper noun in every language.
    pub fn brand(self) -> &'static str {
        match self {
            Kind::Google => "Google",
            Kind::Microsoft => "Microsoft",
            Kind::Github => "GitHub",
            Kind::Oidc => "OpenID Connect",
        }
    }

    /// What a provider's button reads, for a generic one its configured label.
    pub fn label(self, provider: &crate::site::Provider) -> String {
        if self == Kind::Oidc && !provider.label.trim().is_empty() {
            provider.label.trim().to_string()
        } else {
            self.brand().to_string()
        }
    }

    /// Where the provider's own console registers an application, for the
    /// Connectivity page.
    pub fn console_url(self) -> Option<&'static str> {
        match self {
            Kind::Google => Some("https://console.cloud.google.com/apis/credentials"),
            Kind::Microsoft => Some(
                "https://entra.microsoft.com/#view/Microsoft_AAD_RegisteredApps/ApplicationsListBlade",
            ),
            Kind::Github => Some("https://github.com/settings/developers"),
            Kind::Oidc => None,
        }
    }

    /// The address a provider's discovery document is read from.
    pub fn discovery_url(self, provider: &crate::site::Provider) -> Option<String> {
        match self {
            Kind::Google => {
                Some("https://accounts.google.com/.well-known/openid-configuration".into())
            }
            Kind::Microsoft => {
                let tenant = provider.tenant.trim();
                let tenant = if tenant.is_empty() { "common" } else { tenant };
                if !tenant
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
                {
                    return None;
                }
                Some(format!(
                    "https://login.microsoftonline.com/{tenant}/v2.0/.well-known/openid-configuration"
                ))
            }
            Kind::Github => None,
            Kind::Oidc => {
                let issuer = provider.issuer.trim().trim_end_matches('/');
                let ok = issuer.starts_with("https://")
                    || (cfg!(debug_assertions) && is_loopback_http(issuer));
                ok.then(|| format!("{issuer}/.well-known/openid-configuration"))
            }
        }
    }

    /// What is asked of the provider.
    fn scopes(self) -> &'static str {
        match self {
            Kind::Github => "read:user user:email",
            _ => "openid email profile",
        }
    }
}

/// `http://127.0.0.1:…` or `http://localhost:…` — the one plain-HTTP issuer a
/// debug build accepts, so the test suite can run a provider of its own.
fn is_loopback_http(url: &str) -> bool {
    url.strip_prefix("http://").is_some_and(|rest| {
        let host = rest.split(['/', ':']).next().unwrap_or("");
        host == "127.0.0.1" || host == "localhost" || host == "[::1]"
    })
}

/// A provider's endpoints, from its discovery document or, for GitHub, fixed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoints {
    /// The issuer ID tokens must name. `None` for GitHub, which issues none.
    pub issuer: Option<String>,
    pub authorization: String,
    pub token: String,
}

/// Why a sign-in through a provider did not happen. Each maps to one sentence
/// on the page the reader lands on, and to one line in the journal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SsoError {
    /// The provider is not enabled, or lacks its client id or secret.
    NotConfigured,
    /// No public address is set, so there is nowhere to send the reader back.
    NoPublicUrl,
    /// The provider could not be reached or answered something unusable.
    Unreachable(String),
    /// The provider refused, or the reader cancelled at the provider.
    Refused(String),
    /// The answer did not belong to a flow this server started for this
    /// browser, or came too late.
    Stale,
    /// The provider's answer failed a check.
    Invalid(String),
}

impl SsoError {
    /// The catalogue key for the reader's page.
    pub fn key(&self) -> &'static str {
        match self {
            SsoError::NotConfigured => "sso-error-not-configured",
            SsoError::NoPublicUrl => "sso-error-no-public-url",
            SsoError::Unreachable(_) => "sso-error-unreachable",
            SsoError::Refused(_) => "sso-error-refused",
            SsoError::Stale => "sso-error-stale",
            SsoError::Invalid(_) => "sso-error-invalid",
        }
    }

    /// The detail, for the journal. Never shown to the reader: it can carry
    /// what a provider said, which is not theirs to read.
    pub fn detail(&self) -> String {
        match self {
            SsoError::Unreachable(s) | SsoError::Refused(s) | SsoError::Invalid(s) => s.clone(),
            other => format!("{other:?}"),
        }
    }
}

/// What the reader was doing when they left for the provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    SignIn,
    Register,
}

/// One sign-in in flight.
#[derive(Debug, Clone)]
pub struct Flow {
    pub kind: Kind,
    pub intent: Intent,
    /// Where to land afterwards: a same-site path.
    pub next: String,
    /// The visitor already gave the registration secret, on the way out.
    pub secret_ok: bool,
    nonce: String,
    verifier: String,
    redirect_uri: String,
    endpoints: Endpoints,
    created: Instant,
}

/// Who the provider says the reader is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    pub kind: Kind,
    /// The provider's stable identifier for this person: `sub`, or GitHub's
    /// numeric id.
    pub subject: String,
    /// The issuer that vouches for `subject`; with it, the pair is unique.
    pub issuer: String,
    pub email: Option<String>,
    /// Whether the provider says it verified `email`. Only a verified address
    /// links an existing account.
    pub email_verified: bool,
    pub name: Option<String>,
}

/// A provider's user with no account yet, waiting for the registration
/// secret.
#[derive(Debug, Clone)]
pub struct PendingAccount {
    pub claims: Claims,
    pub next: String,
    created: Instant,
}

/// Every flow in flight, and what was discovered about each provider.
#[derive(Default)]
pub struct Flows {
    flows: Mutex<HashMap<String, Flow>>,
    pending: Mutex<HashMap<String, PendingAccount>>,
    discovered: Mutex<HashMap<String, (Instant, Endpoints)>>,
}

impl Flows {
    pub fn new() -> Self {
        Self::default()
    }

    /// Hold a provider's user until they give the secret; the token is what
    /// the reader's cookie carries.
    pub fn hold(&self, claims: Claims, next: String) -> String {
        let token = random_token(32);
        let mut map = self.pending.lock().unwrap_or_else(|e| e.into_inner());
        map.retain(|_, p| p.created.elapsed() < FLOW_TTL);
        if map.len() >= MAX_PENDING {
            if let Some(oldest) = map
                .iter()
                .min_by_key(|(_, p)| p.created)
                .map(|(k, _)| k.clone())
            {
                map.remove(&oldest);
            }
        }
        map.insert(
            token.clone(),
            PendingAccount {
                claims,
                next,
                created: Instant::now(),
            },
        );
        token
    }

    /// The provider's user held under `token`, still fresh. Peeked, not
    /// taken: a wrong secret is answered with the same form.
    pub fn held(&self, token: &str) -> Option<PendingAccount> {
        let map = self.pending.lock().unwrap_or_else(|e| e.into_inner());
        map.get(token)
            .filter(|p| p.created.elapsed() < FLOW_TTL)
            .cloned()
    }

    /// Forget a held user: their account exists now, or they gave up.
    pub fn release(&self, token: &str) {
        let mut map = self.pending.lock().unwrap_or_else(|e| e.into_inner());
        map.remove(token);
    }

    fn insert(&self, state: String, flow: Flow) {
        let mut map = self.flows.lock().unwrap_or_else(|e| e.into_inner());
        map.retain(|_, f| f.created.elapsed() < FLOW_TTL);
        while map.len() >= MAX_PENDING {
            let Some(oldest) = map
                .iter()
                .min_by_key(|(_, f)| f.created)
                .map(|(k, _)| k.clone())
            else {
                break;
            };
            map.remove(&oldest);
        }
        map.insert(state, flow);
    }

    /// Take a flow by its `state`, once. A second callback with the same
    /// `state` — a reload, or a replay — finds nothing.
    fn take(&self, state: &str) -> Option<Flow> {
        let mut map = self.flows.lock().unwrap_or_else(|e| e.into_inner());
        map.remove(state).filter(|f| f.created.elapsed() < FLOW_TTL)
    }

    /// How many sign-ins are in flight right now, for the Connectivity page.
    pub fn in_flight(&self) -> usize {
        let map = self.flows.lock().unwrap_or_else(|e| e.into_inner());
        map.values()
            .filter(|f| f.created.elapsed() < FLOW_TTL)
            .count()
    }

    fn cached_endpoints(&self, url: &str) -> Option<Endpoints> {
        let map = self.discovered.lock().unwrap_or_else(|e| e.into_inner());
        map.get(url)
            .filter(|(at, _)| at.elapsed() < DISCOVERY_TTL)
            .map(|(_, e)| e.clone())
    }

    fn remember_endpoints(&self, url: &str, e: &Endpoints) {
        let mut map = self.discovered.lock().unwrap_or_else(|e| e.into_inner());
        map.insert(url.to_string(), (Instant::now(), e.clone()));
    }

    /// Forget what was discovered, after the operator changed a provider.
    pub fn forget_discovery(&self) {
        let mut map = self.discovered.lock().unwrap_or_else(|e| e.into_inner());
        map.clear();
    }
}

/// The agent every exchange with a provider goes through.
fn agent() -> ureq::Agent {
    ureq::Agent::new_with_config(
        ureq::Agent::config_builder()
            .timeout_global(Some(TIMEOUT))
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            // A provider's error is read, not turned into an exception: its
            // `error_description` is what the journal needs.
            .http_status_as_error(false)
            .build(),
    )
}

/// Read a provider's endpoints, from its discovery document.
pub async fn endpoints(
    flows: &Flows,
    kind: Kind,
    provider: &crate::site::Provider,
) -> Result<Endpoints, SsoError> {
    if kind == Kind::Github {
        return Ok(Endpoints {
            issuer: None,
            authorization: "https://github.com/login/oauth/authorize".into(),
            token: "https://github.com/login/oauth/access_token".into(),
        });
    }
    let url = kind
        .discovery_url(provider)
        .ok_or(SsoError::NotConfigured)?;
    if let Some(e) = flows.cached_endpoints(&url) {
        return Ok(e);
    }
    let fetched = url.clone();
    let doc = tokio::task::spawn_blocking(move || -> Result<Value, String> {
        let mut resp = agent().get(&fetched).call().map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("{} answered {}", fetched, resp.status()));
        }
        let body = resp
            .body_mut()
            .read_to_string()
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&body).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| SsoError::Unreachable(e.to_string()))?
    .map_err(SsoError::Unreachable)?;
    let field = |k: &str| doc.get(k).and_then(Value::as_str).map(str::to_string);
    let e = Endpoints {
        issuer: field("issuer"),
        authorization: field("authorization_endpoint")
            .ok_or_else(|| SsoError::Unreachable("no authorization_endpoint".into()))?,
        token: field("token_endpoint")
            .ok_or_else(|| SsoError::Unreachable("no token_endpoint".into()))?,
    };
    if e.issuer.is_none() {
        return Err(SsoError::Unreachable(
            "no issuer in the discovery document".into(),
        ));
    }
    flows.remember_endpoints(&url, &e);
    Ok(e)
}

/// Reach a provider the way a sign-in would, signing nobody in: its
/// discovery document, read afresh, or GitHub's authorization page. What the
/// Connectivity page's test button answers with — usually that this machine
/// may not reach the provider at all, which is worth knowing before a family
/// member clicks "Continue with Google".
pub async fn probe(
    flows: &Flows,
    kind: Kind,
    provider: &crate::site::Provider,
) -> Result<String, SsoError> {
    flows.forget_discovery();
    if kind == Kind::Github {
        let url = "https://github.com/login/oauth/authorize";
        let status = tokio::task::spawn_blocking(move || {
            agent()
                .get(url)
                .call()
                .map(|r| r.status().as_u16())
                .map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| SsoError::Unreachable(e.to_string()))?
        .map_err(SsoError::Unreachable)?;
        return if status < 500 {
            Ok(format!("github.com answered {status}"))
        } else {
            Err(SsoError::Unreachable(format!(
                "github.com answered {status}"
            )))
        };
    }
    let e = endpoints(flows, kind, provider).await?;
    Ok(e.issuer.unwrap_or_default())
}

/// Start a sign-in: remember the flow, and return the address to send the
/// reader to and the `state` their cookie carries.
pub async fn begin(
    flows: &Flows,
    site: &crate::site::Site,
    kind: Kind,
    intent: Intent,
    next: &str,
    secret_ok: bool,
) -> Result<(String, String), SsoError> {
    let provider = site.providers.get(kind).clone();
    if !provider.is_ready(kind) {
        return Err(SsoError::NotConfigured);
    }
    let base = site.public_base().ok_or(SsoError::NoPublicUrl)?;
    let endpoints = endpoints(flows, kind, &provider).await?;
    let redirect_uri = callback_url(&base, kind);
    let state = random_token(32);
    let nonce = random_token(32);
    let verifier = random_token(48);
    let challenge = pkce_challenge(&verifier);
    let mut url = format!(
        "{}{}response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        endpoints.authorization,
        if endpoints.authorization.contains('?') { '&' } else { '?' },
        enc(provider.client_id.trim()),
        enc(&redirect_uri),
        enc(kind.scopes()),
        enc(&state),
        enc(&challenge),
    );
    if kind != Kind::Github {
        url.push_str(&format!("&nonce={}", enc(&nonce)));
    }
    if kind == Kind::Google || kind == Kind::Microsoft {
        // Let a reader with two accounts at the provider choose, rather than
        // being signed in as whichever one the browser last used.
        url.push_str("&prompt=select_account");
    }
    flows.insert(
        state.clone(),
        Flow {
            kind,
            intent,
            next: crate::render::safe_back(next),
            secret_ok,
            nonce,
            verifier,
            redirect_uri,
            endpoints,
            created: Instant::now(),
        },
    );
    Ok((url, state))
}

/// Where a provider sends the reader back to.
pub fn callback_url(base: &str, kind: Kind) -> String {
    format!(
        "{}/auth/{}/callback",
        base.trim_end_matches('/'),
        kind.slug()
    )
}

/// Finish a sign-in: check the provider's answer against the flow it belongs
/// to, exchange the code, and say who the reader is.
pub async fn finish(
    flows: &Flows,
    site: &crate::site::Site,
    kind: Kind,
    state: &str,
    cookie_state: Option<&str>,
    code: &str,
) -> Result<(Claims, Flow), SsoError> {
    // The cookie first: a callback for a flow this browser did not start is
    // refused before the server-side flow is spent.
    match cookie_state {
        Some(c) if crate::session::constant_time_eq(c.as_bytes(), state.as_bytes()) => {}
        _ => return Err(SsoError::Stale),
    }
    let flow = flows.take(state).ok_or(SsoError::Stale)?;
    if flow.kind != kind {
        return Err(SsoError::Stale);
    }
    if code.trim().is_empty() {
        return Err(SsoError::Invalid("no code".into()));
    }
    let provider = site.providers.get(kind).clone();
    if !provider.is_ready(kind) {
        return Err(SsoError::NotConfigured);
    }
    let token = exchange(&flow, &provider, code).await?;
    let claims = match kind {
        Kind::Github => github_claims(token).await?,
        _ => {
            let id_token = token
                .get("id_token")
                .and_then(Value::as_str)
                .ok_or_else(|| SsoError::Invalid("no id_token".into()))?;
            let payload = jwt_payload(id_token)?;
            check_id_token(&payload, kind, &flow, &provider, now_unix())?
        }
    };
    Ok((claims, flow))
}

/// Exchange the code at the token endpoint.
async fn exchange(
    flow: &Flow,
    provider: &crate::site::Provider,
    code: &str,
) -> Result<Value, SsoError> {
    let form = [
        ("grant_type", "authorization_code".to_string()),
        ("code", code.to_string()),
        ("redirect_uri", flow.redirect_uri.clone()),
        ("client_id", provider.client_id.trim().to_string()),
        ("client_secret", provider.client_secret.trim().to_string()),
        ("code_verifier", flow.verifier.clone()),
    ];
    let url = flow.endpoints.token.clone();
    let (status, body) = tokio::task::spawn_blocking(move || -> Result<(u16, String), String> {
        let pairs: Vec<(&str, &str)> = form.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let mut resp = agent()
            .post(&url)
            .header("Accept", "application/json")
            .send_form(pairs)
            .map_err(|e| e.to_string())?;
        let status = resp.status().as_u16();
        let body = resp
            .body_mut()
            .read_to_string()
            .map_err(|e| e.to_string())?;
        Ok((status, body))
    })
    .await
    .map_err(|e| SsoError::Unreachable(e.to_string()))?
    .map_err(SsoError::Unreachable)?;
    let json: Value = serde_json::from_str(&body).map_err(|_| {
        SsoError::Invalid(format!("the token endpoint answered {status} with no JSON"))
    })?;
    if let Some(err) = json.get("error").and_then(Value::as_str) {
        let desc = json
            .get("error_description")
            .and_then(Value::as_str)
            .unwrap_or("");
        return Err(SsoError::Refused(format!("{err}: {desc}")));
    }
    if !(200..300).contains(&status) {
        return Err(SsoError::Refused(format!(
            "the token endpoint answered {status}"
        )));
    }
    Ok(json)
}

/// GitHub issues no ID token: who the reader is comes from its user API, and
/// their verified primary address from the addresses API.
async fn github_claims(token: Value) -> Result<Claims, SsoError> {
    let access = token
        .get("access_token")
        .and_then(Value::as_str)
        .ok_or_else(|| SsoError::Invalid("no access_token".into()))?
        .to_string();
    let (user, emails) = tokio::task::spawn_blocking(move || -> Result<(Value, Value), String> {
        let get = |url: &str| -> Result<Value, String> {
            let mut resp = agent()
                .get(url)
                .header("Authorization", &format!("Bearer {access}"))
                .header("Accept", "application/vnd.github+json")
                .call()
                .map_err(|e| e.to_string())?;
            if !resp.status().is_success() {
                return Err(format!("{url} answered {}", resp.status()));
            }
            let body = resp
                .body_mut()
                .read_to_string()
                .map_err(|e| e.to_string())?;
            serde_json::from_str(&body).map_err(|e| e.to_string())
        };
        Ok((
            get("https://api.github.com/user")?,
            get("https://api.github.com/user/emails").unwrap_or(Value::Null),
        ))
    })
    .await
    .map_err(|e| SsoError::Unreachable(e.to_string()))?
    .map_err(SsoError::Unreachable)?;
    let id = user
        .get("id")
        .and_then(|v| {
            v.as_u64()
                .map(|n| n.to_string())
                .or_else(|| v.as_str().map(str::to_string))
        })
        .filter(|s| !s.is_empty())
        .ok_or_else(|| SsoError::Invalid("no user id".into()))?;
    let primary = emails.as_array().and_then(|list| {
        list.iter().find(|e| {
            e.get("primary").and_then(Value::as_bool) == Some(true)
                && e.get("verified").and_then(Value::as_bool) == Some(true)
        })
    });
    let (email, verified) = match primary.and_then(|e| e.get("email")).and_then(Value::as_str) {
        Some(e) => (Some(e.to_string()), true),
        None => (
            user.get("email")
                .and_then(Value::as_str)
                .map(str::to_string),
            false,
        ),
    };
    let name = user
        .get("name")
        .and_then(Value::as_str)
        .or_else(|| user.get("login").and_then(Value::as_str))
        .map(str::to_string);
    Ok(Claims {
        kind: Kind::Github,
        subject: id,
        issuer: "https://github.com".into(),
        email,
        email_verified: verified,
        name,
    })
}

/// The JSON payload of a compact JWT, unverified (see the module notes).
pub fn jwt_payload(token: &str) -> Result<Value, SsoError> {
    let mut parts = token.split('.');
    let (Some(_), Some(payload), Some(_)) = (parts.next(), parts.next(), parts.next()) else {
        return Err(SsoError::Invalid("the ID token is not a JWT".into()));
    };
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload.trim_end_matches('='))
        .map_err(|_| SsoError::Invalid("the ID token's payload is not base64url".into()))?;
    serde_json::from_slice(&bytes)
        .map_err(|_| SsoError::Invalid("the ID token's payload is not JSON".into()))
}

/// Check an ID token's claims against the flow that asked for it.
pub fn check_id_token(
    payload: &Value,
    kind: Kind,
    flow: &Flow,
    provider: &crate::site::Provider,
    now: i64,
) -> Result<Claims, SsoError> {
    let s = |k: &str| payload.get(k).and_then(Value::as_str);
    let iss = s("iss").ok_or_else(|| SsoError::Invalid("no iss".into()))?;
    let expected = flow.endpoints.issuer.as_deref().unwrap_or("");
    let issuer_ok = if kind == Kind::Microsoft && expected.contains("{tenantid}") {
        // A multi-tenant directory's document names a template; the token
        // names its own tenant, which must be the one it says it is from.
        s("tid").is_some_and(|tid| expected.replace("{tenantid}", tid) == iss)
    } else if kind == Kind::Google {
        iss == "https://accounts.google.com" || iss == "accounts.google.com"
    } else {
        iss == expected
    };
    if !issuer_ok {
        return Err(SsoError::Invalid(format!("issuer {iss} is not {expected}")));
    }
    let client = provider.client_id.trim();
    let aud_ok = match payload.get("aud") {
        Some(Value::String(a)) => a == client,
        Some(Value::Array(list)) => {
            list.iter().any(|a| a.as_str() == Some(client))
                && (list.len() == 1 || s("azp") == Some(client))
        }
        _ => false,
    };
    if !aud_ok {
        return Err(SsoError::Invalid(
            "the ID token is for another client".into(),
        ));
    }
    let exp = payload.get("exp").and_then(Value::as_i64).unwrap_or(0);
    if exp + 60 < now {
        return Err(SsoError::Invalid("the ID token has expired".into()));
    }
    if let Some(iat) = payload.get("iat").and_then(Value::as_i64) {
        if iat > now + 300 {
            return Err(SsoError::Invalid("the ID token is from the future".into()));
        }
    }
    let nonce_ok = s("nonce")
        .is_some_and(|n| crate::session::constant_time_eq(n.as_bytes(), flow.nonce.as_bytes()));
    if !nonce_ok {
        return Err(SsoError::Invalid("the nonce does not match".into()));
    }
    let subject = s("sub")
        .filter(|v| !v.is_empty())
        .ok_or_else(|| SsoError::Invalid("no sub".into()))?;
    let email = s("email").map(str::to_string).filter(|e| e.contains('@'));
    // Only an explicit `true` is a verified address. Microsoft's `email` is
    // whatever the directory's user typed, and carries no such claim.
    let email_verified = email.is_some()
        && match payload.get("email_verified") {
            Some(Value::Bool(b)) => *b,
            Some(Value::String(v)) => v == "true",
            _ => false,
        };
    let name = s("name")
        .or_else(|| s("preferred_username"))
        .map(str::to_string);
    Ok(Claims {
        kind,
        subject: subject.to_string(),
        issuer: iss.to_string(),
        email,
        email_verified,
        name,
    })
}

/// PKCE's S256 challenge for a verifier.
pub fn pkce_challenge(verifier: &str) -> String {
    use sha2::Digest as _;
    let digest = sha2::Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}

/// `n` bytes from the operating system's random source, base64url.
pub fn random_token(n: usize) -> String {
    use rand::RngCore as _;
    let mut bytes = vec![0u8; n];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Percent-encode a query value.
fn enc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// A username for a provider's user: the address's local part, or their
/// name, in the characters a username may hold, made unique against `taken`.
pub fn username_for(claims: &Claims, taken: impl Fn(&str) -> bool) -> String {
    let from_email = claims
        .email
        .as_deref()
        .and_then(|e| e.split('@').next())
        .unwrap_or("");
    let raw = if from_email.is_empty() {
        claims.name.as_deref().unwrap_or("member")
    } else {
        from_email
    };
    let mut base: String = crate::text::fold(raw)
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '.'
            }
        })
        .collect();
    while base.contains("..") {
        base = base.replace("..", ".");
    }
    let mut base = base.trim_matches('.').to_string();
    base.truncate(40);
    if base.chars().count() < 2 {
        base = "member".into();
    }
    if !taken(&base) {
        return base;
    }
    for n in 2.. {
        let candidate = format!("{base}{n}");
        if !taken(&candidate) {
            return candidate;
        }
    }
    unreachable!("an unbounded range always finds a free name")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flow(kind: Kind, issuer: &str) -> Flow {
        Flow {
            kind,
            intent: Intent::SignIn,
            next: "/".into(),
            secret_ok: false,
            nonce: "n-123".into(),
            verifier: "v".into(),
            redirect_uri: "https://family.example/auth/google/callback".into(),
            endpoints: Endpoints {
                issuer: Some(issuer.into()),
                authorization: "https://idp/authorize".into(),
                token: "https://idp/token".into(),
            },
            created: Instant::now(),
        }
    }

    fn provider() -> crate::site::Provider {
        crate::site::Provider {
            enabled: true,
            client_id: "client-1".into(),
            client_secret: "s".into(),
            ..Default::default()
        }
    }

    #[test]
    fn a_good_id_token_says_who_the_reader_is() {
        let p = serde_json::json!({
            "iss": "https://accounts.google.com", "aud": "client-1", "sub": "1089",
            "exp": 2000, "iat": 1000, "nonce": "n-123",
            "email": "anna@example.org", "email_verified": true, "name": "Anna Nowak"
        });
        let c = check_id_token(
            &p,
            Kind::Google,
            &flow(Kind::Google, "https://accounts.google.com"),
            &provider(),
            1500,
        )
        .unwrap();
        assert_eq!(c.subject, "1089");
        assert!(c.email_verified);
        assert_eq!(c.name.as_deref(), Some("Anna Nowak"));
    }

    #[test]
    fn every_claim_that_binds_the_token_is_checked() {
        let f = flow(Kind::Oidc, "https://id.example");
        let base = serde_json::json!({
            "iss": "https://id.example", "aud": "client-1", "sub": "7",
            "exp": 2000, "nonce": "n-123"
        });
        assert!(check_id_token(&base, Kind::Oidc, &f, &provider(), 1500).is_ok());
        for (k, v) in [
            ("iss", serde_json::json!("https://evil.example")),
            ("aud", serde_json::json!("someone-else")),
            ("exp", serde_json::json!(1000)),
            ("nonce", serde_json::json!("replayed")),
            ("sub", serde_json::json!("")),
        ] {
            let mut p = base.clone();
            p[k] = v;
            assert!(
                check_id_token(&p, Kind::Oidc, &f, &provider(), 1500).is_err(),
                "{k} must be checked"
            );
        }
    }

    #[test]
    fn an_unverified_address_is_never_called_verified() {
        let f = flow(Kind::Oidc, "https://id.example");
        let p = serde_json::json!({
            "iss": "https://id.example", "aud": "client-1", "sub": "7",
            "exp": 2000, "nonce": "n-123", "email": "a@b.c"
        });
        assert!(
            !check_id_token(&p, Kind::Oidc, &f, &provider(), 1500)
                .unwrap()
                .email_verified
        );
    }

    #[test]
    fn a_multi_tenant_directory_checks_the_tenant_the_token_names() {
        let f = flow(
            Kind::Microsoft,
            "https://login.microsoftonline.com/{tenantid}/v2.0",
        );
        let p = serde_json::json!({
            "iss": "https://login.microsoftonline.com/abc/v2.0", "tid": "abc",
            "aud": "client-1", "sub": "9", "exp": 2000, "nonce": "n-123"
        });
        assert!(check_id_token(&p, Kind::Microsoft, &f, &provider(), 1500).is_ok());
        let mut forged = p.clone();
        forged["tid"] = serde_json::json!("other");
        assert!(check_id_token(&forged, Kind::Microsoft, &f, &provider(), 1500).is_err());
    }

    #[test]
    fn pkce_matches_the_rfc_7636_example() {
        assert_eq!(
            pkce_challenge("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"),
            "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
        );
    }

    #[test]
    fn a_username_is_made_from_the_address_and_made_unique() {
        let c = Claims {
            kind: Kind::Google,
            subject: "1".into(),
            issuer: "i".into(),
            email: Some("Zofia.Żak+tree@example.org".into()),
            email_verified: true,
            name: None,
        };
        assert_eq!(username_for(&c, |_| false), "zofia.zak.tree");
        assert_eq!(
            username_for(&c, |n| n == "zofia.zak.tree"),
            "zofia.zak.tree2"
        );
        let nameless = Claims {
            email: None,
            name: None,
            ..c
        };
        assert_eq!(username_for(&nameless, |_| false), "member");
    }

    #[test]
    fn a_debug_build_accepts_a_loopback_issuer_and_nothing_else_in_clear() {
        let mut p = provider();
        p.issuer = "http://127.0.0.1:9999".into();
        assert!(Kind::Oidc.discovery_url(&p).is_some());
        p.issuer = "http://idp.example".into();
        assert!(Kind::Oidc.discovery_url(&p).is_none());
        p.issuer = "https://idp.example/".into();
        assert_eq!(
            Kind::Oidc.discovery_url(&p).as_deref(),
            Some("https://idp.example/.well-known/openid-configuration")
        );
    }
}
