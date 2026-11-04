//! Canva's AI as the AFG's picture service (the operator's request of
//! 4 November 2026, docs/adr/0030): the account connected once by an
//! administrator, and a photograph restored through Canva's MCP server,
//! brought back at its full size.
//!
//! # The connection
//!
//! Canva's MCP server is an OAuth 2.1 resource. This site registers itself
//! as a public client the first time (`/register`, RFC 7591), then sends the
//! administrator to Canva with a PKCE challenge. Canva sends them back to an
//! address on `localhost`: when their browser reaches this site there, the
//! connection ends by itself; otherwise the page does not load, and they
//! paste its address into the room. The tokens are kept with the AFG's
//! settings in `family.acl`, and no page shows them.
//!
//! # A photograph restored
//!
//! 1. `create-upload-url`, and the photograph posted to it.
//! 2. `generate-image`, the photograph as its reference and the restoration
//!    as its prompt; then `get-generate-image-job` until it is done.
//! 3. Canva gives back a thumbnail of what it made, not the picture: the
//!    picture at its full size comes out of a design. `get-assets` says its
//!    size; one editing transaction empties the page of the base design (one
//!    design, made once) and lays the picture on it; `export-design` gives
//!    the page back at the scale that gives the picture its own size, and
//!    the picture is cut out of it here.
//! 4. The picture and the base design are filed in the account's folder
//!    `ax-genealogy`, for the family to clear when it likes.
//!
//! Only `generate-image`, and `create-design` once, spend the account's AI
//! credits: Canva holds them back for a while when they come too fast
//! (`quota_cooldown`), so the base design is made before the picture is
//! asked for, never after.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

use crate::afg::{self, CanvaLink, Failure, ImageConfig};

/// What the AFG asks of the account.
pub const SCOPES: &str = "profile:read design:meta:read design:content:read \
design:content:write folder:read folder:write asset:read asset:write";

/// The folder of the account the AFG files its work in.
pub const FOLDER: &str = "ax-genealogy";

/// What every call says it is for, as the server asks of its clients.
const INTENT: &str = "ax-genealogy: restore an old family photograph (colours, \
sharpness) and bring it back at its full size for the family's archive";

/// The page the base design is made from: anything will do, as every copy
/// is emptied before the picture fills it.
const BASE_BRIEF: &str =
    "Une page vierge : fond blanc uni, sans aucun texte, image, forme ni décoration.";
const BASE_FORMAT: &str = "Instagram Post (Portrait)";

/// How long an authorization started in the room waits for its return.
const PENDING_TTL: Duration = Duration::from_secs(20 * 60);

/// The most an answer of the server, or a picture it exports, may weigh.
const ANSWER_MAX: u64 = 64 * 1024 * 1024;

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn fail(what: impl Into<String>) -> Failure {
    Failure::new(format!("Canva: {}", what.into()))
}

/// The address Canva sends the administrator back to: this site's own, on
/// `localhost` and the port they reach it at, which Canva accepts without
/// HTTPS.
pub fn redirect_uri_for(host: &str) -> String {
    let port = host
        .rsplit_once(':')
        .and_then(|(_, p)| p.parse::<u16>().ok())
        .unwrap_or(80);
    format!("http://localhost:{port}/admin/afg/canva/callback")
}

// ---------------------------------------------------------------------------
// the authorization server
// ---------------------------------------------------------------------------

struct Endpoints {
    authorize: String,
    token: String,
    register: String,
}

fn endpoints(agent: &ureq::Agent, base: &str) -> Result<Endpoints, Failure> {
    let url = format!("{base}/.well-known/oauth-authorization-server");
    let v = get_json(agent, &url)?;
    let field = |k: &str| {
        v.get(k)
            .and_then(Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| fail(format!("{url} names no {k}")))
    };
    Ok(Endpoints {
        authorize: field("authorization_endpoint")?,
        token: field("token_endpoint")?,
        register: field("registration_endpoint")?,
    })
}

fn get_json(agent: &ureq::Agent, url: &str) -> Result<Value, Failure> {
    let mut resp = agent
        .get(url)
        .header("Accept", "application/json")
        .call()
        .map_err(|e| fail(format!("{url}: {e}")))?;
    let (status, text) = read_text(&mut resp)?;
    if !(200..300).contains(&status) {
        return Err(fail(afg::http_error(status, &text)));
    }
    serde_json::from_str(&text).map_err(|e| fail(format!("{url}: not JSON: {e}")))
}

fn read_text(resp: &mut ureq::http::Response<ureq::Body>) -> Result<(u16, String), Failure> {
    let status = resp.status().as_u16();
    let text = resp
        .body_mut()
        .with_config()
        .limit(ANSWER_MAX)
        .read_to_string()
        .map_err(|e| fail(format!("reading the answer: {e}")))?;
    Ok((status, text))
}

/// A token endpoint's answer, folded into the link.
fn take_tokens(link: &mut CanvaLink, v: &Value) -> Result<(), Failure> {
    if let Some(e) = v.get("error").and_then(Value::as_str) {
        let d = v
            .get("error_description")
            .and_then(Value::as_str)
            .unwrap_or_default();
        return Err(fail(format!("{e}: {d}")));
    }
    let access = v
        .get("access_token")
        .and_then(Value::as_str)
        .filter(|t| !t.is_empty())
        .ok_or_else(|| fail("the token endpoint gave no access token"))?;
    link.access_token = access.to_string();
    if let Some(r) = v
        .get("refresh_token")
        .and_then(Value::as_str)
        .filter(|t| !t.is_empty())
    {
        link.refresh_token = r.to_string();
    }
    let life = v.get("expires_in").and_then(Value::as_u64).unwrap_or(3600);
    link.expires_at = now() + life;
    Ok(())
}

fn post_token(agent: &ureq::Agent, url: &str, form: &[(&str, &str)]) -> Result<Value, Failure> {
    let mut resp = agent
        .post(url)
        .header("Accept", "application/json")
        .send_form(form.iter().copied())
        .map_err(|e| fail(format!("{url}: {e}")))?;
    let (status, text) = read_text(&mut resp)?;
    let v: Value = serde_json::from_str(&text).map_err(|_| fail(afg::http_error(status, &text)))?;
    if !(200..300).contains(&status) && v.get("error").is_none() {
        return Err(fail(afg::http_error(status, &text)));
    }
    Ok(v)
}

/// Exchange the refresh token for a new access token.
fn refresh(agent: &ureq::Agent, base: &str, link: &mut CanvaLink) -> Result<(), Failure> {
    let ep = endpoints(agent, base)?;
    let resource = format!("{base}/mcp");
    let refresh_token = link.refresh_token.clone();
    let client_id = link.client_id.clone();
    let v = post_token(
        agent,
        &ep.token,
        &[
            ("grant_type", "refresh_token"),
            ("refresh_token", &refresh_token),
            ("client_id", &client_id),
            ("resource", &resource),
        ],
    )?;
    take_tokens(link, &v)
}

// ---------------------------------------------------------------------------
// connecting the account
// ---------------------------------------------------------------------------

struct Pending {
    verifier: String,
    client_id: String,
    redirect_uri: String,
    made: Instant,
}

fn pending() -> &'static Mutex<HashMap<String, Pending>> {
    static PENDING: OnceLock<Mutex<HashMap<String, Pending>>> = OnceLock::new();
    PENDING.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Begin connecting an account: register this site with Canva when it has
/// not, or not for this address, then give the address to send the
/// administrator to. The link comes back with the registration in it.
pub async fn start(
    cfg: ImageConfig,
    redirect_uri: String,
    timeout: Duration,
) -> Result<(String, CanvaLink), Failure> {
    tokio::task::spawn_blocking(move || {
        let agent = afg::http_agent(timeout);
        let base = cfg.base_url();
        let ep = endpoints(&agent, &base)?;
        let mut link = cfg.canva.clone();
        if link.client_id.is_empty() || link.redirect_uri != redirect_uri {
            let body = json!({
                "client_name": "ax-genealogy",
                "redirect_uris": [redirect_uri],
                "grant_types": ["authorization_code", "refresh_token"],
                "response_types": ["code"],
                "token_endpoint_auth_method": "none",
            });
            let mut resp = agent
                .post(&ep.register)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .send(body.to_string())
                .map_err(|e| fail(format!("{}: {e}", ep.register)))?;
            let (status, text) = read_text(&mut resp)?;
            if !(200..300).contains(&status) {
                return Err(fail(afg::http_error(status, &text)));
            }
            let v: Value = serde_json::from_str(&text)
                .map_err(|e| fail(format!("the registration is not JSON: {e}")))?;
            link.client_id = v
                .get("client_id")
                .and_then(Value::as_str)
                .filter(|c| !c.is_empty())
                .ok_or_else(|| fail("the registration gave no client_id"))?
                .to_string();
            link.redirect_uri = redirect_uri.clone();
        }
        let verifier = crate::sso::random_token(48);
        let state = crate::sso::random_token(18);
        let q = crate::view::query_value;
        let url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&code_challenge={}\
             &code_challenge_method=S256&state={}&scope={}&resource={}",
            ep.authorize,
            q(&link.client_id),
            q(&link.redirect_uri),
            q(&crate::sso::pkce_challenge(&verifier)),
            q(&state),
            q(SCOPES),
            q(&format!("{base}/mcp")),
        );
        let mut map = pending().lock().unwrap_or_else(|e| e.into_inner());
        map.retain(|_, p| p.made.elapsed() < PENDING_TTL);
        map.insert(
            state,
            Pending {
                verifier,
                client_id: link.client_id.clone(),
                redirect_uri: link.redirect_uri.clone(),
                made: Instant::now(),
            },
        );
        Ok((url, link))
    })
    .await
    .map_err(|e| fail(e.to_string()))?
}

/// A parameter of a query string, decoded.
fn param(query: &str, name: &str) -> Option<String> {
    query.split('&').find_map(|pair| {
        let (k, v) = pair.split_once('=')?;
        (k == name).then(|| percent_decode(v))
    })
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok();
                match hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                    Some(b) => {
                        out.push(b);
                        i += 3;
                    }
                    None => {
                        out.push(b'%');
                        i += 1;
                    }
                }
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// The query of what the administrator pasted: the whole address Canva sent
/// them back to, or its query alone.
fn returned_query(returned: &str) -> &str {
    let r = returned.trim();
    let r = r.split_once('#').map_or(r, |(q, _)| q);
    r.split_once('?').map_or(r, |(_, q)| q)
}

/// Finish connecting the account: the code Canva sent back, exchanged for
/// the tokens, and the account's name asked for the room.
pub async fn finish(
    cfg: ImageConfig,
    returned: String,
    timeout: Duration,
) -> Result<CanvaLink, Failure> {
    tokio::task::spawn_blocking(move || {
        let query = returned_query(&returned);
        if let Some(e) = param(query, "error") {
            let d = param(query, "error_description").unwrap_or_default();
            return Err(fail(format!("{e} {d}").trim().to_string()));
        }
        let code = param(query, "code").ok_or_else(|| fail("no code in the address"))?;
        let state = param(query, "state").ok_or_else(|| fail("no state in the address"))?;
        let p = {
            let mut map = pending().lock().unwrap_or_else(|e| e.into_inner());
            map.remove(&state)
        }
        .filter(|p| p.made.elapsed() < PENDING_TTL)
        .ok_or_else(|| fail("this connection was not started here, or it is too old"))?;
        let agent = afg::http_agent(timeout);
        let base = cfg.base_url();
        let ep = endpoints(&agent, &base)?;
        let resource = format!("{base}/mcp");
        let v = post_token(
            &agent,
            &ep.token,
            &[
                ("grant_type", "authorization_code"),
                ("code", &code),
                ("redirect_uri", &p.redirect_uri),
                ("client_id", &p.client_id),
                ("code_verifier", &p.verifier),
                ("resource", &resource),
            ],
        )?;
        let mut link = CanvaLink {
            client_id: p.client_id,
            redirect_uri: p.redirect_uri,
            ..CanvaLink::default()
        };
        take_tokens(&mut link, &v)?;
        let mut mcp = Mcp::new(agent, &base, link);
        if mcp.begin().is_err() {
            return Ok(mcp.link);
        }
        if let Ok(who) = mcp.call("get-logged-in-account-details", json!({})) {
            mcp.link.account = who
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
        }
        Ok(mcp.link)
    })
    .await
    .map_err(|e| fail(e.to_string()))?
}

// ---------------------------------------------------------------------------
// the MCP server
// ---------------------------------------------------------------------------

/// One conversation with the MCP server, in the account's name.
struct Mcp {
    agent: ureq::Agent,
    base: String,
    endpoint: String,
    link: CanvaLink,
    next: u64,
    protocol: Option<String>,
    session: Option<String>,
}

enum Answer {
    Ok(Value),
    Unauthorized,
}

impl Mcp {
    /// A conversation not yet begun: nothing sent.
    fn new(agent: ureq::Agent, base: &str, link: CanvaLink) -> Mcp {
        Mcp {
            agent,
            base: base.to_string(),
            endpoint: format!("{base}/mcp"),
            link,
            next: 0,
            protocol: None,
            session: None,
        }
    }

    /// Begin it: the token renewed when it is about to expire, and the
    /// protocol's handshake. A token renewed stays in `link` whatever
    /// follows, so the caller keeps it.
    fn begin(&mut self) -> Result<(), Failure> {
        if self.link.expires_at <= now() + 60 {
            let base = self.base.clone();
            refresh(&self.agent, &base, &mut self.link)?;
        }
        self.handshake()
    }

    fn handshake(&mut self) -> Result<(), Failure> {
        let init = self.request(
            "initialize",
            json!({
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": {"name": "axgf-cms", "version": env!("CARGO_PKG_VERSION")},
            }),
            true,
        )?;
        self.protocol = init
            .get("protocolVersion")
            .and_then(Value::as_str)
            .map(str::to_string);
        self.post(json!({"jsonrpc": "2.0", "method": "notifications/initialized"}))?;
        Ok(())
    }

    fn post(&mut self, payload: Value) -> Result<(u16, Option<String>, String), Failure> {
        let mut req = self
            .agent
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/event-stream")
            .header(
                "Authorization",
                format!("Bearer {}", self.link.access_token),
            );
        if let Some(s) = &self.session {
            req = req.header("Mcp-Session-Id", s.as_str());
        }
        if let Some(p) = &self.protocol {
            req = req.header("MCP-Protocol-Version", p.as_str());
        }
        let mut resp = req
            .send(payload.to_string())
            .map_err(|e| fail(format!("{}: {e}", self.endpoint)))?;
        if let Some(s) = resp
            .headers()
            .get("Mcp-Session-Id")
            .and_then(|v| v.to_str().ok())
        {
            self.session = Some(s.to_string());
        }
        let ctype = resp
            .headers()
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);
        let (status, text) = read_text(&mut resp)?;
        Ok((status, ctype, text))
    }

    /// One JSON-RPC request: its result; a refused token renewed once.
    fn request(&mut self, method: &str, params: Value, retry: bool) -> Result<Value, Failure> {
        match self.attempt(method, &params)? {
            Answer::Ok(v) => Ok(v),
            Answer::Unauthorized if retry => {
                refresh(&self.agent, &self.base.clone(), &mut self.link)?;
                match self.attempt(method, &params)? {
                    Answer::Ok(v) => Ok(v),
                    Answer::Unauthorized => Err(fail("the account's connection was refused")),
                }
            }
            Answer::Unauthorized => Err(fail("the account's connection was refused")),
        }
    }

    fn attempt(&mut self, method: &str, params: &Value) -> Result<Answer, Failure> {
        self.next += 1;
        let id = self.next;
        let (status, ctype, text) = self.post(json!({
            "jsonrpc": "2.0", "id": id, "method": method, "params": params,
        }))?;
        if status == 401 {
            return Ok(Answer::Unauthorized);
        }
        if !(200..300).contains(&status) {
            return Err(fail(afg::http_error(status, &text)));
        }
        let msg = rpc_message(ctype.as_deref(), &text, id)
            .ok_or_else(|| fail(format!("{method}: no answer to the request")))?;
        if let Some(e) = msg.get("error") {
            let m = e.get("message").and_then(Value::as_str).unwrap_or("error");
            return Err(fail(format!("{method}: {m}")));
        }
        Ok(Answer::Ok(
            msg.get("result").cloned().unwrap_or(Value::Null),
        ))
    }

    /// Call a tool: what it answered, structured or as the JSON of its text.
    fn call(&mut self, tool: &str, mut args: Value) -> Result<Value, Failure> {
        if let Some(o) = args.as_object_mut() {
            o.entry("user_intent")
                .or_insert_with(|| Value::from(INTENT));
        }
        let result = self.request("tools/call", json!({"name": tool, "arguments": args}), true)?;
        let out = tool_output(&result);
        if result.get("isError").and_then(Value::as_bool) == Some(true) {
            return Err(fail(format!(
                "{tool}: {}",
                afg::tail(&text_of(&result), 300)
            )));
        }
        Ok(out)
    }
}

/// The JSON-RPC message answering request `id`, from a JSON body or an
/// event stream.
fn rpc_message(ctype: Option<&str>, text: &str, id: u64) -> Option<Value> {
    if ctype.is_some_and(|c| c.contains("text/event-stream")) {
        return text.split("\n\n").find_map(|event| {
            let data: String = event
                .lines()
                .filter_map(|l| l.strip_prefix("data:"))
                .map(str::trim)
                .collect();
            let v: Value = serde_json::from_str(&data).ok()?;
            (v.get("id").and_then(Value::as_u64) == Some(id)).then_some(v)
        });
    }
    serde_json::from_str(text).ok()
}

fn text_of(result: &Value) -> String {
    result
        .get("content")
        .and_then(Value::as_array)
        .map(|blocks| {
            blocks
                .iter()
                .filter_map(|b| b.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}

/// A tool's answer: its structured content, else the first of its text
/// blocks that is JSON, else its text.
fn tool_output(result: &Value) -> Value {
    if let Some(s) = result.get("structuredContent").filter(|s| !s.is_null()) {
        return s.clone();
    }
    let blocks = result
        .get("content")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    blocks
        .iter()
        .filter_map(|b| b.get("text").and_then(Value::as_str))
        .find_map(|t| serde_json::from_str::<Value>(t).ok())
        .unwrap_or_else(|| json!({"text": text_of(result)}))
}

// ---------------------------------------------------------------------------
// a photograph restored
// ---------------------------------------------------------------------------

/// One picture at a time on the base design, which every picture is placed
/// on in turn.
fn base_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// Restore a photograph with Canva: the picture at its full size, as a
/// JPEG; and, whether it came or not, the link as it now stands (its token
/// renewed, its folder and base design made), for the caller to keep: a
/// token renewed may have made the one before it worthless.
pub async fn restore(
    cfg: ImageConfig,
    timeout: Duration,
    prompt: &'static str,
    bytes: Vec<u8>,
) -> (Result<Vec<u8>, Failure>, Option<CanvaLink>) {
    let run = tokio::task::spawn_blocking(move || {
        let _one = base_lock().lock().unwrap_or_else(|e| e.into_inner());
        let deadline = Instant::now() + timeout;
        let agent = afg::http_agent(timeout);
        let base = cfg.base_url();
        let mut mcp = Mcp::new(agent, &base, cfg.canva.clone());
        let picture = mcp
            .begin()
            .and_then(|()| restore_with(&mut mcp, deadline, prompt, &bytes));
        (picture, Some(mcp.link))
    })
    .await;
    run.unwrap_or_else(|e| (Err(fail(e.to_string())), None))
}

fn restore_with(
    mcp: &mut Mcp,
    deadline: Instant,
    prompt: &str,
    bytes: &[u8],
) -> Result<Vec<u8>, Failure> {
    // 0. The page every picture is placed on, made first: a design Canva
    //    cannot make now must not cost the picture's credit.
    if mcp.link.base_design_id.is_empty() {
        mcp.link.base_design_id = base_design(mcp, deadline)?;
        file_away(mcp, &[mcp.link.base_design_id.clone().as_str()]);
    }

    // 1. The photograph, into the account.
    let up = mcp.call("create-upload-url", json!({}))?;
    let upload_url = up
        .get("uploadUrl")
        .or_else(|| up.get("upload_url"))
        .and_then(Value::as_str)
        .ok_or_else(|| fail("create-upload-url gave no address"))?
        .to_string();
    let mut resp = mcp
        .agent
        .post(&upload_url)
        .header("Content-Type", "application/octet-stream")
        .send(bytes)
        .map_err(|e| fail(format!("the upload: {e}")))?;
    let (status, text) = read_text(&mut resp)?;
    if !(200..300).contains(&status) {
        return Err(fail(format!(
            "the upload: {}",
            afg::http_error(status, &text)
        )));
    }
    let uploaded: Value = serde_json::from_str(&text)
        .map_err(|e| fail(format!("the upload's answer is not JSON: {e}")))?;
    let media = uploaded
        .get("mediaId")
        .or_else(|| uploaded.get("media_id"))
        .or_else(|| uploaded.get("id"))
        .and_then(Value::as_str)
        .ok_or_else(|| fail("the upload gave no media id"))?
        .to_string();

    // 2. Restored by Canva's AI.
    let job = mcp.call(
        "generate-image",
        json!({"prompt": prompt, "imageReferences": [{"type": "MEDIA", "id": media}]}),
    )?;
    refused(&job)?;
    let job_id = job
        .get("jobId")
        .or_else(|| job.get("job_id"))
        .and_then(Value::as_str)
        .ok_or_else(|| fail("generate-image gave no job"))?
        .to_string();
    let restored = loop {
        if Instant::now() >= deadline {
            return Err(fail("no picture in the time allowed"));
        }
        let st = mcp.call("get-generate-image-job", json!({"jobId": job_id}))?;
        match st.get("status").and_then(Value::as_str) {
            Some("SUCCESS") => {
                let id = st
                    .pointer("/result/results/0/id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| fail("the job gave no picture"))?;
                break id.to_string();
            }
            Some("FAILURE") => {
                let t = st.get("failureType").and_then(Value::as_str).unwrap_or("");
                let m = st
                    .get("failureMessage")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                return Err(fail(format!("{t} {m}").trim().to_string()));
            }
            _ => std::thread::sleep(Duration::from_secs(3)),
        }
    };

    // 3. Its size. Canva gives back a thumbnail of what it made, not the
    //    picture: the picture comes out of the base design's page.
    let assets = mcp.call("get-assets", json!({"asset_ids": [restored]}))?;
    let meta = assets
        .pointer("/asset/metadata")
        .or_else(|| assets.pointer("/assets/0/metadata"))
        .cloned()
        .unwrap_or(Value::Null);
    let (w, h) = match (
        meta.get("width").and_then(Value::as_u64),
        meta.get("height").and_then(Value::as_u64),
    ) {
        (Some(w), Some(h)) if w > 0 && h > 0 => (w as f64, h as f64),
        _ => return Err(fail("get-assets gave no size for the picture")),
    };
    let design = mcp.link.base_design_id.clone();
    let tx = match mcp.call("start-editing-transaction", json!({"design_id": design})) {
        Ok(tx) => tx,
        Err(f) => {
            // The base design gone from the account: made again next time.
            mcp.link.base_design_id.clear();
            return Err(f);
        }
    };
    let tid = tx
        .pointer("/transaction/transaction_id")
        .or_else(|| tx.pointer("/transaction/id"))
        .and_then(Value::as_str)
        .ok_or_else(|| fail("start-editing-transaction gave no transaction"))?
        .to_string();
    let pages = tx.get("pages").cloned().unwrap_or_else(|| json!([]));
    let page_id = pages
        .pointer("/0/page_id")
        .and_then(Value::as_str)
        .ok_or_else(|| fail("the base design has no page"))?
        .to_string();
    let pw = pages
        .pointer("/0/dimension/width")
        .and_then(Value::as_f64)
        .unwrap_or(1080.0);
    let ph = pages
        .pointer("/0/dimension/height")
        .and_then(Value::as_f64)
        .unwrap_or(1440.0);
    let placed = placement(pw, ph, w, h);
    let ops = place_operations(&tx, &page_id, &restored, placed);
    mcp.call(
        "perform-editing-operations",
        json!({"transaction_id": tid, "operations": ops, "page_index": 1, "pages": pages}),
    )?;
    mcp.call("commit-editing-transaction", json!({"transaction_id": tid}))?;

    // 4. The page, exported at the scale that gives the picture its own
    //    size, then cut down to the picture.
    let export_width = (pw * w / placed.0).round().clamp(40.0, 25_000.0) as u64;
    let export = mcp.call(
        "export-design",
        json!({"design_id": design,
               "format": {"type": "png", "width": export_width, "lossless": true}}),
    )?;
    let job = export.get("job").unwrap_or(&export);
    if job.get("status").and_then(Value::as_str) != Some("success") {
        return Err(fail(format!(
            "export-design: {}",
            afg::tail(&export.to_string(), 300)
        )));
    }
    let url = job
        .pointer("/urls/0")
        .and_then(Value::as_str)
        .ok_or_else(|| fail("export-design gave no address"))?
        .to_string();
    let mut resp = mcp
        .agent
        .get(&url)
        .call()
        .map_err(|e| fail(format!("the export: {e}")))?;
    let status = resp.status().as_u16();
    let page = resp
        .body_mut()
        .with_config()
        .limit(ANSWER_MAX)
        .read_to_vec()
        .map_err(|e| fail(format!("the export: {e}")))?;
    if !(200..300).contains(&status) {
        return Err(fail(format!("the export: HTTP {status}")));
    }
    let picture = cut_out(&page, pw, ph, placed)?;

    // 5. Filed, for the family to clear when it likes.
    file_away(mcp, &[restored.as_str()]);
    Ok(picture)
}

/// A failure a tool reported in its answer rather than as an error:
/// `{"status": "failed", "failure": {"code", "message"}}`.
fn refused(v: &Value) -> Result<(), Failure> {
    if v.get("status").and_then(Value::as_str) == Some("failed") {
        let code = v
            .pointer("/failure/code")
            .and_then(Value::as_str)
            .unwrap_or("failed");
        let msg = v
            .pointer("/failure/message")
            .and_then(Value::as_str)
            .unwrap_or("");
        return Err(fail(format!("{code}: {msg}")));
    }
    Ok(())
}

/// Where the picture goes on the page: its top left corner, as large as the
/// page allows without changing its proportions. Its width and height there.
fn placement(pw: f64, ph: f64, w: f64, h: f64) -> (f64, f64) {
    let s = (pw / w).min(ph / h);
    (w * s, h * s)
}

/// Make the base design: the first page Canva makes, whatever it holds, as
/// every picture empties it before taking its place.
fn base_design(mcp: &mut Mcp, deadline: Instant) -> Result<String, Failure> {
    let job = mcp.call(
        "create-design",
        json!({"brief": BASE_BRIEF, "format": BASE_FORMAT}),
    )?;
    refused(&job)?;
    let job_id = job
        .get("job_id")
        .and_then(Value::as_str)
        .ok_or_else(|| fail("create-design gave no job"))?
        .to_string();
    let mut token = job
        .get("continuation_token")
        .cloned()
        .unwrap_or(Value::Null);
    let mut wait = job
        .pointer("/polling_policy/wait_seconds")
        .and_then(Value::as_u64)
        .unwrap_or(15);
    loop {
        if Instant::now() + Duration::from_secs(wait) >= deadline {
            return Err(fail("no base design in the time allowed"));
        }
        std::thread::sleep(Duration::from_secs(wait.clamp(1, 60)));
        let st = mcp.call(
            "get-create-design-async-job",
            json!({"job_id": job_id, "continuation_token": token}),
        )?;
        if let Some(t) = st.get("continuation_token") {
            token = t.clone();
        }
        wait = st
            .pointer("/polling_policy/wait_seconds")
            .and_then(Value::as_u64)
            .unwrap_or(5);
        match st.get("status").and_then(Value::as_str) {
            Some("completed") | Some("success") => {
                return st
                    .pointer("/design/id")
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .ok_or_else(|| fail("create-design gave no design"));
            }
            Some("failed") | Some("error") => {
                refused(&st)?;
                return Err(fail(format!(
                    "create-design: {}",
                    afg::tail(&st.to_string(), 300)
                )));
            }
            _ => {}
        }
    }
}

/// The operations that empty the page and lay the picture on it: every
/// text and every image laid on it before deleted (the page's own
/// background stays, under it and cut away after), and the picture at the
/// top left corner, `placed` wide and high.
fn place_operations(tx: &Value, page_id: &str, picture: &str, placed: (f64, f64)) -> Vec<Value> {
    let list = |k: &str| {
        tx.get(k)
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
    };
    let mut ops: Vec<Value> = list("richtexts")
        .iter()
        .chain(list("fills").iter())
        .filter_map(|el| el.get("element_id").and_then(Value::as_str))
        .filter(|id| *id != page_id)
        .map(|id| json!({"type": "delete_element", "element_id": id}))
        .collect();
    ops.push(json!({
        "type": "insert_fill", "page_id": page_id, "asset_type": "image",
        "asset_id": picture, "alt_text": "Photographie restaurée (ax-genealogy)",
        "top": 0, "left": 0, "width": placed.0, "height": placed.1,
    }));
    ops
}

/// The picture cut out of the exported page: the region `placed` covers,
/// at the scale the page was exported at; as a JPEG.
fn cut_out(page: &[u8], pw: f64, ph: f64, placed: (f64, f64)) -> Result<Vec<u8>, Failure> {
    let img = image::load_from_memory(page)
        .map_err(|e| fail(format!("the export is not a picture: {e}")))?;
    let sx = img.width() as f64 / pw;
    let sy = img.height() as f64 / ph;
    let cw = ((placed.0 * sx).round() as u32).clamp(1, img.width());
    let ch = ((placed.1 * sy).round() as u32).clamp(1, img.height());
    let cut = img.crop_imm(0, 0, cw, ch).to_rgb8();
    let mut out = std::io::Cursor::new(Vec::new());
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 92)
        .encode_image(&image::DynamicImage::ImageRgb8(cut))
        .map_err(|e| fail(format!("the picture could not be encoded: {e}")))?;
    Ok(out.into_inner())
}

/// File the work in the account's folder, made when there is none. Never
/// a reason to fail: a folder gone is made again the next time.
fn file_away(mcp: &mut Mcp, items: &[&str]) {
    if mcp.link.folder_id.is_empty() {
        let made = mcp.call(
            "create-folder",
            json!({"name": FOLDER, "parent_folder_id": "root"}),
        );
        match made.ok().and_then(|f| {
            f.pointer("/folder/id")
                .and_then(Value::as_str)
                .map(str::to_string)
        }) {
            Some(id) => mcp.link.folder_id = id,
            None => return,
        }
    }
    for item in items {
        let moved = mcp.call(
            "move-item-to-folder",
            json!({"item_id": item, "to_folder_id": mcp.link.folder_id}),
        );
        if moved.is_err() && item.starts_with('D') {
            mcp.link.folder_id.clear();
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_address_canva_sends_back_to_is_this_sites_on_localhost() {
        assert_eq!(
            redirect_uri_for("192.168.1.105:8089"),
            "http://localhost:8089/admin/afg/canva/callback"
        );
        assert_eq!(
            redirect_uri_for("genealogie.example"),
            "http://localhost:80/admin/afg/canva/callback"
        );
    }

    #[test]
    fn what_was_pasted_gives_its_code_and_state() {
        let pasted = "http://localhost:8765/callback?code=oUZ%3A3CQ%3AMxn2&state=AI5-x_Z";
        let q = returned_query(pasted);
        assert_eq!(param(q, "code").as_deref(), Some("oUZ:3CQ:Mxn2"));
        assert_eq!(param(q, "state").as_deref(), Some("AI5-x_Z"));
        assert_eq!(returned_query("code=a&state=b"), "code=a&state=b");
        assert_eq!(
            param("error=access_denied", "error").as_deref(),
            Some("access_denied")
        );
        assert_eq!(percent_decode("a%2"), "a%2");
    }

    #[test]
    fn a_tools_answer_is_read_structured_or_from_its_text() {
        let structured = json!({"structuredContent": {"jobId": "j1"}, "content": []});
        assert_eq!(tool_output(&structured)["jobId"], "j1");
        let text = json!({"content": [{"type": "text", "text": "{\"asset\": {\"metadata\": {\"width\": 720}}}"},
                                      {"type": "image", "data": "…"}]});
        assert_eq!(
            tool_output(&text).pointer("/asset/metadata/width"),
            Some(&json!(720))
        );
        let sse = "event: message\ndata: {\"jsonrpc\":\"2.0\",\"id\":7,\"result\":{}}\n\n";
        assert!(rpc_message(Some("text/event-stream"), sse, 7).is_some());
        assert!(rpc_message(Some("text/event-stream"), sse, 8).is_none());
    }

    #[test]
    fn the_page_is_emptied_and_the_picture_laid_on_it_then_cut_out() {
        let tx = json!({
            "richtexts": [{"element_id": "P1-T1"}],
            "fills": [{"element_id": "P1"}, {"element_id": "P1-I2", "containerElement": {}}],
        });
        let placed = placement(1080.0, 1440.0, 720.0, 912.0);
        assert_eq!(placed, (1080.0, 1368.0));
        let ops = place_operations(&tx, "P1", "M9", placed);
        assert_eq!(
            ops.len(),
            3,
            "two deleted, the page's background kept, one laid"
        );
        assert_eq!(ops[2]["type"], "insert_fill");
        assert_eq!(ops[2]["asset_id"], "M9");
        assert_eq!(ops[2]["height"], 1368.0);

        // The page exported 720 wide: the picture is its top 720 × 912.
        let mut page = image::RgbImage::from_pixel(720, 960, image::Rgb([255, 255, 255]));
        for y in 0..912 {
            for x in 0..720 {
                page.put_pixel(x, y, image::Rgb([200, 30, 30]));
            }
        }
        let mut png = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgb8(page)
            .write_to(&mut png, image::ImageFormat::Png)
            .unwrap();
        let jpeg = cut_out(png.get_ref(), 1080.0, 1440.0, placed).unwrap();
        let back = image::load_from_memory(&jpeg).unwrap();
        assert_eq!((back.width(), back.height()), (720, 912));
        let px = back.to_rgb8().get_pixel(700, 900).0;
        assert!(px[0] > 150 && px[1] < 90, "no white band: {px:?}");
    }

    #[test]
    fn a_refusal_in_an_answer_is_a_failure() {
        let v = json!({"status": "failed", "failure": {"code": "quota_cooldown", "message": "Credit quota cooldown is active"}});
        let f = refused(&v).unwrap_err();
        assert!(f.message.contains("quota_cooldown"));
        assert!(refused(&json!({"jobId": "j"})).is_ok());
    }
}
