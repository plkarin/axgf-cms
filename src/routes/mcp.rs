//! The MCP (docs/adr/0045 §4): the archive's operations offered to AI
//! assistants elsewhere — ChatGPT, Claude, any client of the Model Context
//! Protocol — over its Streamable HTTP transport: JSON-RPC 2.0 posted to
//! `/mcp`, one JSON answer for each. No stream and no session: every request
//! carries its token and stands alone.
//!
//! The token is an account's (src/tokens.rs), in `Authorization: Bearer`, or
//! in the address itself, `/mcp/t/<token>`, for the clients that can be given
//! an address and nothing else. Either way the request acts as that account,
//! through the same operations as the AFG's commands (src/ops.rs): the same
//! role, the same branch, the same lens, the same journal, with `MCP` and the
//! token's name beside the account's name. The session cookie is never read
//! here, so a page elsewhere has nothing to borrow.
//!
//! Messages are in English: they are read by an assistant, which says them to
//! its user in their own words.

use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde_json::{json, Value};

use crate::access::Viewer;
use crate::ops::{self, Operation};
use crate::routes::Shared;

/// The protocol versions this server speaks, the newest first.
const VERSIONS: &[&str] = &["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"];

/// The language of the MCP's messages.
const LANG: &str = "en";

/// The most a request may weigh.
pub const MOST_BYTES: usize = 512 * 1024;

/// Who is calling, once their token is found.
#[derive(Clone)]
struct Caller {
    viewer: Viewer,
    /// The token may write, and so may its account.
    write: bool,
    /// What the journal says the write came through.
    via: String,
    /// The records' door lets the account in (docs/adr/0046): what every
    /// tool but the guide reads and writes.
    records: bool,
}

/// `POST /mcp`
pub async fn post(State(state): State<Shared>, headers: HeaderMap, body: Bytes) -> Response {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| {
            let (scheme, rest) = v.trim().split_once(' ')?;
            scheme
                .eq_ignore_ascii_case("bearer")
                .then(|| rest.trim().to_string())
        });
    answer(state, token, body).await
}

/// `POST /mcp/t/:token`
pub async fn post_with(
    State(state): State<Shared>,
    Path(token): Path<String>,
    body: Bytes,
) -> Response {
    answer(state, Some(token), body).await
}

/// The community connector's files (docs/adr/0045 §5), as the repository
/// holds them in `mcp/ax-genealogy/`.
const CONNECTOR_MANIFEST: &str = include_str!("../../mcp/ax-genealogy/manifest.json");
const CONNECTOR_SERVER: &str = include_str!("../../mcp/ax-genealogy/server.mjs");
const CONNECTOR_README: &str = include_str!("../../mcp/ax-genealogy/README.md");

/// The community connector, packed as Claude Desktop installs it: an MCP
/// bundle (a ZIP) of its manifest at this release's version, its bridge, its
/// guide and its icon. Nothing in it is a site's: the address and the token
/// are asked for when it is installed.
pub fn connector() -> std::io::Result<Vec<u8>> {
    use std::io::Write as _;
    let mut manifest: Value =
        serde_json::from_str(CONNECTOR_MANIFEST).map_err(std::io::Error::other)?;
    manifest["version"] = json!(env!("CARGO_PKG_VERSION"));
    let manifest = serde_json::to_string_pretty(&manifest).map_err(std::io::Error::other)?;
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let deflated = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in [
        ("manifest.json", manifest.as_bytes()),
        ("server.mjs", CONNECTOR_SERVER.as_bytes()),
        ("README.md", CONNECTOR_README.as_bytes()),
        ("icon.png", crate::render::ICON_192_PNG),
    ] {
        zip.start_file(name, deflated)
            .map_err(std::io::Error::other)?;
        zip.write_all(bytes)?;
    }
    Ok(zip.finish().map_err(std::io::Error::other)?.into_inner())
}

/// `GET /mcp/ax-genealogy.mcpb` — the community connector, to install in
/// Claude Desktop.
pub async fn connector_file() -> Response {
    match connector() {
        Ok(bytes) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "application/zip"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"ax-genealogy.mcpb\"",
                ),
            ],
            bytes,
        )
            .into_response(),
        Err(e) => {
            tracing::error!(error = %e, "the community connector could not be packed");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// `GET /mcp` — no stream is offered, which the transport says is answered so.
pub async fn get() -> Response {
    (StatusCode::METHOD_NOT_ALLOWED, [(header::ALLOW, "POST")]).into_response()
}

fn http_error(status: StatusCode, code: i64, message: &str) -> Response {
    let mut resp = (
        status,
        [(header::CONTENT_TYPE, "application/json")],
        json!({"jsonrpc": "2.0", "id": null, "error": {"code": code, "message": message}})
            .to_string(),
    )
        .into_response();
    if status == StatusCode::UNAUTHORIZED {
        resp.headers_mut().insert(
            header::WWW_AUTHENTICATE,
            header::HeaderValue::from_static("Bearer realm=\"ax-genealogy\""),
        );
    }
    resp
}

#[allow(clippy::result_large_err)]
fn caller(state: &Shared, token: Option<&str>) -> Result<Caller, Response> {
    if !state.site().mcp {
        return Err(http_error(
            StatusCode::FORBIDDEN,
            -32001,
            "The MCP of this site is closed. An administrator opens it on the AFG page of the administration.",
        ));
    }
    let Some(token) = token.filter(|t| !t.trim().is_empty()) else {
        return Err(http_error(
            StatusCode::UNAUTHORIZED,
            -32001,
            "A token is needed: Authorization: Bearer axgf_…, made on the account's page of the site.",
        ));
    };
    let found =
        state.acl_read(|acl| crate::tokens::find(acl, token).map(|(u, t)| (u.clone(), t.clone())));
    let Some((user, kept)) = found else {
        return Err(http_error(
            StatusCode::UNAUTHORIZED,
            -32001,
            "This token is not known here: mistyped, revoked, or its account disabled.",
        ));
    };
    // The day it was last used, written once a day at most.
    let today = crate::view::now_iso8601()
        .get(..10)
        .unwrap_or_default()
        .to_string();
    if kept.last_used.as_deref() != Some(today.as_str()) {
        let (uid, tid) = (user.id.clone(), kept.id.clone());
        let written = state.acl_mutate(|acl| {
            if let Some(t) = acl
                .users
                .iter_mut()
                .find(|u| u.id == uid)
                .and_then(|u| u.api_tokens.iter_mut().find(|t| t.id == tid))
            {
                t.last_used = Some(today.clone());
            }
        });
        if let Err(e) = written {
            tracing::warn!(error = %e, "could not note the day a token was used");
        }
    }
    let viewer = Viewer {
        user: Some(user),
        emergency: false,
    };
    let records = state.acl_read(|acl| acl.site.doors.records).admits(
        true,
        viewer.may_write(),
        viewer.is_admin(),
    );
    Ok(Caller {
        write: kept.write && viewer.may_write(),
        via: format!("MCP · {}", kept.label),
        records,
        viewer,
    })
}

async fn answer(state: Shared, token: Option<String>, body: Bytes) -> Response {
    let caller = match caller(&state, token.as_deref()) {
        Ok(c) => c,
        Err(r) => return r,
    };
    if body.len() > MOST_BYTES {
        return http_error(
            StatusCode::PAYLOAD_TOO_LARGE,
            -32600,
            "The request is too large.",
        );
    }
    let message: Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => return http_error(StatusCode::BAD_REQUEST, -32700, "Parse error"),
    };
    let reply = match message {
        Value::Array(items) => {
            let mut out = Vec::new();
            for m in &items {
                if let Some(r) = one(&state, &caller, m).await {
                    out.push(r);
                }
            }
            (!out.is_empty()).then_some(Value::Array(out))
        }
        m => one(&state, &caller, &m).await,
    };
    match reply {
        // A notification, or a client's answer: nothing to say back.
        None => StatusCode::ACCEPTED.into_response(),
        Some(v) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            v.to_string(),
        )
            .into_response(),
    }
}

async fn one(state: &Shared, caller: &Caller, m: &Value) -> Option<Value> {
    let id = m.get("id").filter(|v| !v.is_null()).cloned()?;
    let Some(method) = m.get("method").and_then(Value::as_str) else {
        return Some(
            json!({"jsonrpc": "2.0", "id": id, "error": {"code": -32600, "message": "Invalid Request"}}),
        );
    };
    let params = m.get("params").cloned().unwrap_or_else(|| json!({}));
    let result = match method {
        "initialize" => Ok(initialize(&params)),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({"tools": tools(caller)})),
        "tools/call" => call(state, caller, &params).await,
        "resources/list" => Ok(json!({"resources": []})),
        "resources/templates/list" => Ok(json!({"resourceTemplates": []})),
        "prompts/list" => Ok(json!({"prompts": []})),
        _ => Err((-32601, format!("Method not found: {method}"))),
    };
    Some(match result {
        Ok(r) => json!({"jsonrpc": "2.0", "id": id, "result": r}),
        Err((code, message)) => {
            json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
        }
    })
}

fn initialize(params: &Value) -> Value {
    let asked = params
        .get("protocolVersion")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let version = VERSIONS
        .iter()
        .find(|v| **v == asked)
        .copied()
        .unwrap_or(VERSIONS[0]);
    json!({
        "protocolVersion": version,
        "capabilities": {"tools": {"listChanged": false}},
        "serverInfo": {
            "name": "ax-genealogy",
            "title": "AX Genealogy",
            "version": env!("CARGO_PKG_VERSION"),
        },
        "instructions": format!(
            "A family's genealogy archive (AXGF), read and edited as one of its accounts: what it may read, and, if its token may write, what it may change. Find people with search_people before changing them, and name them by the ids it gives. Several changes that belong together go in one apply_operations call: all of them are applied, or none. Ask the user before deleting anything.\n\n{}",
            ops::CATALOGUE
        ),
    })
}

/// A tool: its name, the operation it is, whether it writes, and what it
/// takes.
struct Tool {
    name: &'static str,
    op: Option<&'static str>,
    writes: bool,
    destructive: bool,
    admin: bool,
    description: &'static str,
    input: fn() -> Value,
}

fn person_ref() -> Value {
    json!({"type": "string", "description": "A person: their id (best, from search_people), or a name exactly one person bears. Within apply_operations, also a ref such as new:1."})
}

fn date() -> Value {
    json!({"type": "string", "description": "1890, 1890-06 or 1890-06-14. An empty string removes it."})
}

fn place() -> Value {
    json!({"type": "string", "description": "A place: its id, or its name. A name no place has makes a new place. An empty string removes it."})
}

fn fact() -> Value {
    json!({"type": "object", "properties": {
        "date": date(), "circa": {"type": "boolean", "description": "The date is approximate."}, "place": place()
    }})
}

fn people() -> Value {
    json!({"type": "array", "items": person_ref()})
}

fn object(properties: Value, required: &[&str]) -> Value {
    json!({"type": "object", "properties": properties, "required": required})
}

fn union_kinds() -> Value {
    json!({"type": "string", "enum": crate::person_new::UNION_TYPES})
}

fn genders() -> Value {
    json!({"type": "string", "enum": ["M", "F", "NB", "U"]})
}

const TOOLS: &[Tool] = &[
    Tool {
        name: "search_people",
        op: Some("search"),
        writes: false,
        destructive: false,
        admin: false,
        description: "Find people whose name holds every word of the query; an empty query lists everyone. Gives each one's id, name and years.",
        input: || object(json!({
            "query": {"type": "string", "description": "Words of the name."},
            "limit": {"type": "integer", "minimum": 1, "maximum": 100, "description": "20 by default."}
        }), &["query"]),
    },
    Tool {
        name: "get_person",
        op: Some("get_person"),
        writes: false,
        destructive: false,
        admin: false,
        description: "A person's record: names, gender, birth, death, biography, parents, siblings, unions with their children, and events, each with its id.",
        input: || object(json!({"person": person_ref()}), &["person"]),
    },
    Tool {
        name: "get_family",
        op: Some("get_family"),
        writes: false,
        destructive: false,
        admin: false,
        description: "A family (a union): its kind, its partners and its children.",
        input: || object(json!({"family": {"type": "string", "description": "The family's id."}}), &["family"]),
    },
    Tool {
        name: "create_person",
        op: Some("create_person"),
        writes: true,
        destructive: false,
        admin: false,
        description: "Add a person, with their birth, death, parents, siblings and unions. A parent, sibling, partner or child named by a name nobody bears is created too.",
        input: || object(json!({
            "ref": {"type": "string", "description": "Within apply_operations: a name for this person in later operations, such as new:1."},
            "given": {"type": "string"}, "family": {"type": "string"},
            "nicknames": {"type": "array", "items": {"type": "string"}},
            "gender": genders(),
            "living": {"type": "boolean", "description": "Left out: living unless they died or were born more than 110 years ago."},
            "birth": fact(), "death": fact(),
            "bio": {"type": "string", "description": "Markdown."},
            "father": person_ref(), "mother": person_ref(), "siblings": people(),
            "unions": {"type": "array", "items": object(json!({
                "partner": person_ref(), "kind": union_kinds(), "children": people()
            }), &[])}
        }), &[]),
    },
    Tool {
        name: "update_person",
        op: Some("update_person"),
        writes: true,
        destructive: true,
        admin: false,
        description: "Change a person's names, gender, living state, visibility or biography. A field left out is kept; an empty string removes it.",
        input: || object(json!({
            "person": person_ref(), "given": {"type": "string"}, "family": {"type": "string"},
            "nicknames": {"type": "array", "items": {"type": "string"}}, "gender": genders(),
            "living": {"type": "boolean"},
            "visibility": {"type": "string", "enum": ["public", "members", "contributors", "private"]},
            "bio": {"type": "string", "description": "Markdown."}
        }), &["person"]),
    },
    Tool {
        name: "set_birth",
        op: Some("set_birth"),
        writes: true,
        destructive: true,
        admin: false,
        description: "Set or change a person's birth: date, approximation, place. clear: true removes it.",
        input: || object(json!({
            "person": person_ref(), "date": date(), "circa": {"type": "boolean"}, "place": place(),
            "clear": {"type": "boolean"}
        }), &["person"]),
    },
    Tool {
        name: "set_death",
        op: Some("set_death"),
        writes: true,
        destructive: true,
        admin: false,
        description: "Set or change a person's death: date, approximation, place; the person is then not living. clear: true removes it.",
        input: || object(json!({
            "person": person_ref(), "date": date(), "circa": {"type": "boolean"}, "place": place(),
            "clear": {"type": "boolean"}
        }), &["person"]),
    },
    Tool {
        name: "add_event",
        op: Some("add_event"),
        writes: true,
        destructive: false,
        admin: false,
        description: "Add an event to a person: a baptism, a burial, a migration, a census, military service… with others in it if need be.",
        input: || object(json!({
            "person": person_ref(),
            "category": {"type": "string", "enum": ops::EVENT_CATEGORIES},
            "subcategory": {"type": "string", "description": "baptism, christening, confirmation, burial, cremation, residence, emigration, immigration, graduation, retirement, will, probate…"},
            "date": date(), "circa": {"type": "boolean"}, "place": place(),
            "role": {"type": "string", "enum": ops::ROLES, "description": "The person's role; subject by default."},
            "description": {"type": "string"},
            "with": {"type": "array", "items": object(json!({
                "person": person_ref(), "role": {"type": "string", "enum": ops::ROLES}
            }), &["person"])}
        }), &["person", "category"]),
    },
    Tool {
        name: "update_event",
        op: Some("update_event"),
        writes: true,
        destructive: true,
        admin: false,
        description: "Change an event's category, date, place or description. A field left out is kept; an empty string removes it.",
        input: || object(json!({
            "event": {"type": "string", "description": "The event's id, from get_person."},
            "category": {"type": "string", "enum": ops::EVENT_CATEGORIES},
            "subcategory": {"type": "string"}, "date": date(), "circa": {"type": "boolean"},
            "place": place(), "description": {"type": "string"}
        }), &["event"]),
    },
    Tool {
        name: "delete_event",
        op: Some("delete_event"),
        writes: true,
        destructive: true,
        admin: false,
        description: "Delete an event.",
        input: || object(json!({"event": {"type": "string", "description": "The event's id."}}), &["event"]),
    },
    Tool {
        name: "add_union",
        op: Some("add_union"),
        writes: true,
        destructive: false,
        admin: false,
        description: "Unite one or two people, with the union's kind, children, and its marriage's date and place (an event of its own). A union these partners already have is brought up to date instead.",
        input: || object(json!({
            "ref": {"type": "string", "description": "Within apply_operations: a name for this family, such as new:2."},
            "partners": {"type": "array", "items": person_ref(), "minItems": 1, "maxItems": 2},
            "kind": union_kinds(), "date": date(), "circa": {"type": "boolean"}, "place": place(),
            "children": people()
        }), &["partners"]),
    },
    Tool {
        name: "add_child",
        op: Some("add_child"),
        writes: true,
        destructive: false,
        admin: false,
        description: "Make a person the child of one or two parents (their union is made if they have none), or of a family by its id.",
        input: || object(json!({
            "child": person_ref(), "parents": people(),
            "family": {"type": "string", "description": "A family's id, instead of the parents."}
        }), &["child"]),
    },
    Tool {
        name: "remove_child",
        op: Some("remove_child"),
        writes: true,
        destructive: true,
        admin: false,
        description: "Take a child out of a family.",
        input: || object(json!({
            "child": person_ref(), "family": {"type": "string", "description": "The family's id."}
        }), &["child", "family"]),
    },
    Tool {
        name: "delete_union",
        op: Some("delete_union"),
        writes: true,
        destructive: true,
        admin: false,
        description: "Undo a union: the family goes; its children stay, without these parents. Its marriage event, if any, is deleted with delete_event.",
        input: || object(json!({"family": {"type": "string", "description": "The family's id."}}), &["family"]),
    },
    Tool {
        name: "delete_person",
        op: Some("delete_person"),
        writes: true,
        destructive: true,
        admin: true,
        description: "Delete a person: they leave every family and event, then go. Administrators only. Ask the user first.",
        input: || object(json!({"person": person_ref()}), &["person"]),
    },
    Tool {
        name: "check_operations",
        op: None,
        writes: false,
        destructive: false,
        admin: false,
        description: "Check a batch of operations without writing anything: what it would do, or why it cannot be applied.",
        input: operations_input,
    },
    Tool {
        name: "apply_operations",
        op: None,
        writes: true,
        destructive: true,
        admin: false,
        description: "Apply a batch of operations, all of them or none. Each is an object with \"op\" (create_person, update_person, set_birth, set_death, add_event, update_event, delete_event, add_union, add_child, remove_child, delete_union, delete_person) and that operation's fields; \"ref\": \"new:1\" names what one creates for the next ones. The guide tool describes them all.",
        input: operations_input,
    },
    Tool {
        name: "guide",
        op: None,
        writes: false,
        destructive: false,
        admin: false,
        description: "Every operation, what it takes, and the rules they keep.",
        input: || object(json!({}), &[]),
    },
];

fn operations_input() -> Value {
    object(
        json!({"operations": {"type": "array", "maxItems": ops::MOST,
            "items": {"type": "object", "properties": {"op": {"type": "string"}}, "required": ["op"]}}}),
        &["operations"],
    )
}

fn tools(caller: &Caller) -> Vec<Value> {
    TOOLS
        .iter()
        .filter(|t| caller.records || t.name == "guide")
        .filter(|t| !t.writes || caller.write)
        .filter(|t| !t.admin || caller.viewer.is_admin())
        .map(|t| {
            json!({
                "name": t.name,
                "description": t.description,
                "inputSchema": (t.input)(),
                "annotations": {
                    "readOnlyHint": !t.writes,
                    "destructiveHint": t.destructive,
                    "idempotentHint": !t.writes,
                    "openWorldHint": false,
                },
            })
        })
        .collect()
}

fn text(text: String, error: bool) -> Value {
    json!({"content": [{"type": "text", "text": text}], "isError": error})
}

fn refused(r: &ops::Refusal) -> Value {
    text(r.text(LANG), true)
}

fn lines_text(lines: &[ops::Line]) -> String {
    lines
        .iter()
        .map(|l| format!("- {}", l.text(LANG)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn refs_text(refs: &std::collections::BTreeMap<String, String>) -> String {
    if refs.is_empty() {
        return String::new();
    }
    let pairs: Vec<String> = refs.iter().map(|(r, id)| format!("{r} = {id}")).collect();
    format!("\nIds: {}", pairs.join(", "))
}

async fn call(state: &Shared, caller: &Caller, params: &Value) -> Result<Value, (i64, String)> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or((-32602, "tools/call needs the tool's name".to_string()))?;
    let args = params
        .get("arguments")
        .cloned()
        .filter(Value::is_object)
        .unwrap_or_else(|| json!({}));
    let Some(tool) = TOOLS.iter().find(|t| t.name == name) else {
        return Err((-32602, format!("Unknown tool: {name}")));
    };
    if !caller.records && name != "guide" {
        return Ok(text(
            "The records of this site are not open to this account: an administrator keeps them for other roles.".into(),
            true,
        ));
    }
    if tool.writes && !caller.write {
        return Ok(text(
            "This token reads only, or its account may not write: nothing was changed.".into(),
            true,
        ));
    }
    if name == "guide" {
        return Ok(text(ops::CATALOGUE.to_string(), false));
    }
    let batch: Vec<Operation> = match tool.op {
        Some(op) => {
            let mut fields = args.as_object().cloned().unwrap_or_default();
            fields.insert("op".into(), json!(op));
            match serde_json::from_value(Value::Object(fields)) {
                Ok(o) => vec![o],
                Err(e) => {
                    return Ok(text(
                        format!("These arguments do not fit {name}: {e}"),
                        true,
                    ))
                }
            }
        }
        None => match ops::parse(
            &json!({"operations": args.get("operations").cloned().unwrap_or(Value::Null)})
                .to_string(),
        ) {
            Ok(b) => b,
            Err(e) => return Ok(text(format!("These operations cannot be read: {e}"), true)),
        },
    };
    if let [only] = batch.as_slice() {
        if only.reads() {
            let viewer = caller.viewer.clone();
            let found = state.read(|flat| ops::read(flat, &viewer, only));
            return Ok(match found {
                Ok(v) => text(serde_json::to_string_pretty(&v).unwrap_or_default(), false),
                Err(r) => refused(&r),
            });
        }
    }
    let lang = caller
        .viewer
        .language()
        .unwrap_or(crate::i18n::DEFAULT)
        .to_string();
    if name == "check_operations" {
        let quote = move |n: &str| ops::quoted(&lang, n);
        return Ok(match ops::check(state, &caller.viewer, &batch, &quote) {
            Ok(lines) => text(
                format!("Would apply, all together:\n{}", lines_text(&lines)),
                false,
            ),
            Err(r) => refused(&r),
        });
    }
    let (state, viewer, via) = (state.clone(), caller.viewer.clone(), caller.via.clone());
    let done = tokio::task::spawn_blocking(move || {
        let quote = move |n: &str| ops::quoted(&lang, n);
        ops::apply(&state, &viewer, &via, &batch, &quote)
    })
    .await
    .map_err(|e| (-32603, format!("The write stopped: {e}")))?;
    Ok(match done {
        Ok(d) => text(
            format!(
                "Applied: {} record(s) written.\n{}{}",
                d.records,
                lines_text(&d.lines),
                refs_text(&d.refs)
            ),
            false,
        ),
        Err(r) => refused(&r),
    })
}
