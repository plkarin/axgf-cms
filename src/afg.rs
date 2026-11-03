//! The Artificial family genealogist, the AFG: the agent that reads the
//! family's archive and answers about it (the operator's request of
//! 2 November 2026, docs/adr/0023). This first part is where its computing
//! comes from, and a test.
//!
//! # Where the computing comes from
//!
//! The operator named five ways, and each is spoken to in its own protocol:
//!
//! * **An address**: Ollama, `POST /api/chat`. The model is optional: the
//!   first one the server has installed is taken.
//! * **An address, an API key and a model**:
//!   * OpenAI, Mistral and vLLM speak OpenAI's `POST /chat/completions`;
//!   * Claude speaks Anthropic's `POST /v1/messages`;
//!   * Gemini speaks Google's `POST /v1beta/models/{model}:generateContent`.
//!
//!   Each takes an effort too, in the setting its protocol names for it
//!   (Ollama's `think` among them; docs/adr/0025).
//! * **A coding agent already signed in on this machine**, with a model and an
//!   effort, run as the command line it is:
//!   * Claude Code: `claude -p`, with no tool and no MCP server at all;
//!   * Codex: `codex exec -` with no shell and no web search, in a read-only
//!     sandbox, without the user's `config.toml`;
//!   * OpenCode: `opencode run` with the archive attached and every tool
//!     denied but reading its private working directory.
//!
//! None of them is given a shell: a program is run with its arguments, and
//! the archive goes through its standard input or a file in a private working
//! directory, never through a command line, which is too short for it. None
//! inherits this site's own settings (`AXGF_*`, among them the emergency
//! token).
//!
//! # What is sent
//!
//! The archive as an `.axgf` holds it, every JSON file of it, without the
//! photographs and the scans: the files [`crate::state::export_to_bytes`]
//! writes, read back out of the ZIP. The sensitive classes (health,
//! biometrics, genomics, legal, behaviour) are left out unless the
//! administrator chose otherwise, exactly as a shared export leaves them out
//! ([`crate::state::export_copy`]).
//!
//! # Where the key is kept
//!
//! With the sign-in services' secrets, in the accounts file (`family.acl`,
//! mode 600; [`crate::site`]): never in the bundle, always in a backup, never
//! on a page and never in the journal.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Where the computing comes from.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// No AFG.
    #[default]
    Off,
    Ollama,
    Openai,
    Mistral,
    Vllm,
    Anthropic,
    Gemini,
    ClaudeCode,
    Codex,
    Opencode,
}

/// How a source is spoken to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    None,
    Ollama,
    OpenAi,
    Anthropic,
    Gemini,
    ClaudeCode,
    Codex,
    OpenCode,
}

impl Kind {
    pub const ALL: [Kind; 10] = [
        Kind::Off,
        Kind::Ollama,
        Kind::Openai,
        Kind::Mistral,
        Kind::Vllm,
        Kind::Anthropic,
        Kind::Gemini,
        Kind::ClaudeCode,
        Kind::Codex,
        Kind::Opencode,
    ];

    pub fn slug(self) -> &'static str {
        match self {
            Kind::Off => "off",
            Kind::Ollama => "ollama",
            Kind::Openai => "openai",
            Kind::Mistral => "mistral",
            Kind::Vllm => "vllm",
            Kind::Anthropic => "anthropic",
            Kind::Gemini => "gemini",
            Kind::ClaudeCode => "claude-code",
            Kind::Codex => "codex",
            Kind::Opencode => "opencode",
        }
    }

    pub fn parse(s: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.slug() == s.trim())
    }

    pub fn protocol(self) -> Protocol {
        match self {
            Kind::Off => Protocol::None,
            Kind::Ollama => Protocol::Ollama,
            Kind::Openai | Kind::Mistral | Kind::Vllm => Protocol::OpenAi,
            Kind::Anthropic => Protocol::Anthropic,
            Kind::Gemini => Protocol::Gemini,
            Kind::ClaudeCode => Protocol::ClaudeCode,
            Kind::Codex => Protocol::Codex,
            Kind::Opencode => Protocol::OpenCode,
        }
    }

    /// The operator's three ways: an address, a service (an address, a key
    /// and a model), an agent on this machine.
    pub fn group(self) -> &'static str {
        match self {
            Kind::Off => "off",
            Kind::Ollama => "address",
            Kind::Openai | Kind::Mistral | Kind::Vllm | Kind::Anthropic | Kind::Gemini => "service",
            Kind::ClaudeCode | Kind::Codex | Kind::Opencode => "agent",
        }
    }

    pub fn is_agent(self) -> bool {
        self.group() == "agent"
    }

    pub fn uses_url(self) -> bool {
        matches!(self.group(), "address" | "service")
    }

    /// A key it cannot work without. vLLM takes one when its server was
    /// started with one, and works without otherwise.
    pub fn needs_key(self) -> bool {
        matches!(
            self,
            Kind::Openai | Kind::Mistral | Kind::Anthropic | Kind::Gemini
        )
    }

    pub fn uses_key(self) -> bool {
        self.needs_key() || self == Kind::Vllm
    }

    /// A model it cannot work without. Ollama takes the first it has, and an
    /// agent its own default.
    pub fn needs_model(self) -> bool {
        self.group() == "service"
    }

    /// What follows the address in the call it is spoken to with; Gemini
    /// names the model there.
    pub fn path(self) -> &'static str {
        match self.protocol() {
            Protocol::Ollama => "/api/chat",
            Protocol::OpenAi => "/chat/completions",
            Protocol::Anthropic => "/v1/messages",
            Protocol::Gemini => "/v1beta/models/{model}:generateContent",
            _ => "",
        }
    }

    /// The fields that mean something to it, for the page's script.
    pub fn uses(self) -> &'static str {
        match self.group() {
            "address" => "url model effort",
            "service" => "url key model effort",
            "agent" => "model effort program",
            _ => "",
        }
    }

    pub fn default_url(self) -> &'static str {
        match self {
            Kind::Ollama => "http://127.0.0.1:11434",
            Kind::Openai => "https://api.openai.com/v1",
            Kind::Mistral => "https://api.mistral.ai/v1",
            Kind::Vllm => "http://127.0.0.1:8000/v1",
            Kind::Anthropic => "https://api.anthropic.com",
            Kind::Gemini => "https://generativelanguage.googleapis.com",
            _ => "",
        }
    }

    /// The agent's program.
    pub fn program(self) -> &'static str {
        match self {
            Kind::ClaudeCode => "claude",
            Kind::Codex => "codex",
            Kind::Opencode => "opencode",
            _ => "",
        }
    }

    /// The models of the moment the room suggests for it, each with the
    /// efforts it takes ([`crate::afg_models`]).
    pub fn suggestions(self) -> &'static [crate::afg_models::Suggestion] {
        crate::afg_models::of(self)
    }

    /// Every effort its suggestions take, the lightest first. Suggestions
    /// only: any effort written plainly is sent as it is.
    pub fn efforts(self) -> Vec<&'static str> {
        crate::afg_models::efforts_of(self)
    }
}

/// A plain name: a model, an effort. Letters, digits and the punctuation
/// model names use (`llama3.1:8b`, `sonnet[1m]`, `openai/gpt-5`), nothing a
/// program could read as anything else.
pub fn plain(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 120
        && !s.starts_with('-')
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || "._:/@[]+-".contains(c))
}

fn plausible_url(u: &str) -> bool {
    let rest = u
        .strip_prefix("https://")
        .or_else(|| u.strip_prefix("http://"));
    rest.is_some_and(|r| !r.is_empty() && !r.starts_with('/')) && !u.contains(char::is_whitespace)
}

/// The AFG's configuration, kept in the accounts file with the rules of
/// entry (`site.afg`). Its `Debug` leaves the key out.
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub kind: Kind,
    /// The service's address; empty means the kind's usual one.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub url: String,
    /// The service's API key. Never on a page: shown as set or not, and a
    /// form that leaves it blank keeps it.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_key: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub model: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    /// The agent's program, when it is not on the usual paths.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub program: String,
    /// Whether the sensitive classes go with the archive.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub sensitive: bool,
    /// Seconds an answer may take; 0 is ten minutes.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub timeout_secs: u64,
}

fn is_zero(n: &u64) -> bool {
    *n == 0
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("kind", &self.kind)
            .field("url", &self.url)
            .field("api_key", &self.key_hint())
            .field("model", &self.model)
            .field("effort", &self.effort)
            .field("program", &self.program)
            .field("sensitive", &self.sensitive)
            .field("timeout_secs", &self.timeout_secs)
            .finish()
    }
}

impl Config {
    pub fn is_default(&self) -> bool {
        self == &Config::default()
    }

    pub fn timeout(&self) -> Duration {
        Duration::from_secs(match self.timeout_secs {
            0 => 600,
            n => n.clamp(30, 3600),
        })
    }

    /// The address, or the kind's usual one, without a trailing slash.
    pub fn base_url(&self) -> String {
        let u = match self.url.trim() {
            "" => self.kind.default_url(),
            u => u,
        };
        u.trim_end_matches('/').to_string()
    }

    /// What is called, for the page.
    pub fn endpoint(&self) -> Option<String> {
        self.call_url(&self.model)
    }

    /// The address a call goes to, with the model it asks for.
    fn call_url(&self, model: &str) -> Option<String> {
        match self.kind.path() {
            "" => None,
            path => Some(format!(
                "{}{}",
                self.base_url(),
                path.replace("{model}", gemini_model(model))
            )),
        }
    }

    /// The end of the key, so an administrator can tell which one is set.
    pub fn key_hint(&self) -> Option<String> {
        let k = self.api_key.trim();
        match k.chars().count() {
            0 => None,
            n if n >= 12 => Some(format!("…{}", k.chars().skip(n - 4).collect::<String>())),
            _ => Some("…".to_string()),
        }
    }

    /// What is wrong with it, as a catalogue key; `None` when it may run.
    pub fn problem(&self) -> Option<&'static str> {
        let k = self.kind;
        if k == Kind::Off {
            return None;
        }
        if k.uses_url() && !plausible_url(&self.base_url()) {
            return Some("afg-error-url");
        }
        if k.needs_key() && self.api_key.trim().is_empty() {
            return Some("afg-error-key");
        }
        let model = self.model.trim();
        if k.needs_model() && model.is_empty() {
            return Some("afg-error-model");
        }
        if !model.is_empty() && !plain(model) {
            return Some("afg-error-model");
        }
        // Any effort, as any model: the room suggests, it does not decide
        // (docs/adr/0025). Written plainly, it is never read as an option
        // nor breaks out of the TOML string Codex is given it in.
        let effort = self.effort.trim();
        if !effort.is_empty() && !plain(effort) {
            return Some("afg-error-effort");
        }
        let program = self.program.trim();
        if k.is_agent() && !program.is_empty() {
            // Only the agent's own program, by name: this field never runs
            // anything else.
            let name = Path::new(program).file_name().and_then(|n| n.to_str());
            if name != Some(k.program()) || !Path::new(program).is_absolute() {
                return Some("afg-error-program");
            }
        }
        None
    }
}

/// A Gemini model as its URL names it.
pub(crate) fn gemini_model(m: &str) -> &str {
    let m = m.trim();
    m.strip_prefix("models/").unwrap_or(m)
}

// ---------------------------------------------------------------------------
// what is sent
// ---------------------------------------------------------------------------

/// The archive's JSON files, as an `.axgf` holds them, without its binaries.
#[derive(Debug, Clone, Serialize)]
pub struct Archive {
    #[serde(skip)]
    pub files: Vec<(String, String)>,
    pub count: usize,
    pub bytes: usize,
    /// A rough estimate, a token for every three bytes of JSON.
    pub tokens: usize,
    pub persons: usize,
    pub families: usize,
    pub sensitive: bool,
}

impl Archive {
    /// No person and no family: what the AFG must answer "Bundle vide" to.
    pub fn is_empty(&self) -> bool {
        self.persons == 0 && self.families == 0
    }

    /// The counts without the files: what a test keeps of what it sent.
    pub fn summary(&self) -> Archive {
        Archive {
            files: Vec::new(),
            count: self.count,
            bytes: self.bytes,
            tokens: self.tokens,
            persons: self.persons,
            families: self.families,
            sensitive: self.sensitive,
        }
    }
}

/// Read the archive out the way an export writes it, keeping only its JSON
/// files, each on one line.
pub fn archive(flat: &Value, sensitive: bool) -> anyhow::Result<Archive> {
    use std::io::Read;
    let include = if sensitive {
        crate::sensitive::Scopes::EVERY
    } else {
        crate::sensitive::Scopes::NONE
    };
    let mut copy = crate::state::export_copy(flat, include);
    // No media goes: a photograph the site keeps on disk beside the JSON
    // (`external_payloads`) would make the export refuse the whole archive
    // (axgf-rs `PAYLOADS_EXTERNAL`), and one held in the JSON itself would
    // only be compressed to be read back and left out.
    if let Some(obj) = copy.as_object_mut() {
        obj.remove("external_payloads");
        if let Some(att) = obj.get_mut("attachments").and_then(Value::as_object_mut) {
            att.retain(|path, _| path.ends_with(".json"));
        }
    }
    let bytes = crate::state::export_to_bytes(&copy.to_string())?;
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
    let mut files = Vec::new();
    for i in 0..zip.len() {
        let mut f = zip.by_index(i)?;
        let name = f.name()?.to_string();
        if f.is_dir() || !name.ends_with(".json") {
            continue;
        }
        let mut text = String::new();
        f.read_to_string(&mut text)?;
        let compact = serde_json::from_str::<Value>(&text)
            .map(|v| v.to_string())
            .unwrap_or(text);
        files.push((name, compact));
    }
    files.sort_by(|a, b| (a.0 != "manifest.json", &a.0).cmp(&(b.0 != "manifest.json", &b.0)));
    let count_of = |k: &str| {
        flat.get(k)
            .and_then(Value::as_object)
            .map_or(0, |m| m.len())
    };
    let bytes: usize = files.iter().map(|(n, c)| n.len() + c.len() + 10).sum();
    Ok(Archive {
        count: files.len(),
        bytes,
        tokens: bytes / 3,
        persons: count_of("persons"),
        families: count_of("families"),
        sensitive,
        files,
    })
}

/// What the AFG is told: who it is, the archive, the question.
#[derive(Debug, Clone)]
pub struct Prompt {
    pub system: String,
    pub bundle: String,
    /// The asker's brain (docs/adr/0024), said after the archive; empty when
    /// nobody's is given.
    pub brain: String,
    pub question: String,
}

impl Prompt {
    /// The archive, then the asker's brain: what the AFG reads before the
    /// question.
    pub fn context(&self) -> String {
        if self.brain.is_empty() {
            self.bundle.clone()
        } else {
            format!("{}\n\n{}", self.bundle, self.brain)
        }
    }

    /// The archive, the brain, then the question.
    pub fn user(&self) -> String {
        format!("{}\n\nQuestion: {}", self.context(), self.question)
    }

    /// All of it, for an agent with no separate system prompt.
    pub fn whole(&self) -> String {
        format!("{}\n\n{}", self.system, self.user())
    }
}

/// Who asks (docs/adr/0024): their account's name, the person in the tree
/// they say they are, and what the AFG remembers of them.
#[derive(Debug, Clone, Copy)]
pub struct Asker<'a> {
    pub name: &'a str,
    /// The person's id and name, when the account says who it is.
    pub person: Option<(&'a str, &'a str)>,
    pub recall: &'a crate::brain::Recall,
}

/// The brain as the AFG reads it, after the archive.
fn brain_section(a: &Asker) -> String {
    let mut out = format!("=== brain: {} ===\n", a.name);
    match a.person {
        Some((id, name)) => out.push_str(&format!(
            "You are talking to {}, who says they are {name} (persons/{id}.json) in the \
             archive.\n",
            a.name
        )),
        None => out.push_str(&format!("You are talking to {}.\n", a.name)),
    }
    if a.recall.memory.is_empty() && a.recall.recent.is_empty() {
        out.push_str("You have no memory of them yet: this is your first exchange.\n");
        return out;
    }
    out.push_str("What you noted about them:\n");
    if a.recall.memory.is_empty() {
        out.push_str("(nothing yet)\n");
    } else {
        out.push_str(&a.recall.memory);
        out.push('\n');
    }
    if !a.recall.recent.is_empty() {
        out.push_str("Your last exchanges with them, oldest first:\n");
        for e in &a.recall.recent {
            out.push_str(&format!(
                "--- {}, {}\nQuestion: {}\nAnswer: {}\n",
                e.at, e.context, e.question, e.answer
            ));
        }
    }
    out
}

pub fn prompt(
    archive: &Archive,
    question: &str,
    empty_answer: &str,
    asker: Option<&Asker>,
) -> Prompt {
    let mut system = String::from(
        "You are the Artificial family genealogist (AFG) of an ax·genealogy archive. \
         What you know of the family is its archive: the AXGF bundle given below, every \
         JSON file of it (the manifest, the persons, the families, the events, the links, \
         the sources, the places, the occupations and the documents' descriptions), \
         without the photographs. ",
    );
    if asker.is_some() {
        system.push_str(
            "What you remember of the person asking is their brain, given after the \
             bundle: what you chose to note about them and your last exchanges with them. \
             Each account has a brain of its own, and you are only ever given the asker's. ",
        );
    }
    system.push_str(&format!(
        "Answer from the bundle only, in the language of the question, and name the \
         records your answer rests on. If the bundle holds no person and no family, answer \
         exactly: {empty_answer}"
    ));
    if asker.is_some() {
        system.push_str(
            "\n\nWhen this exchange taught you something about the person asking that will \
             help you next time (who they are in the family, what they are looking for, how \
             they like to be answered), end your answer with a <memory> block, one new fact \
             per line, and nothing after it. It is taken out of your answer and kept in their \
             brain. Never put in it what the bundle already says.",
        );
    }
    let mut bundle = format!(
        "The AXGF bundle: {} JSON files, {} persons, {} families.\n",
        archive.count, archive.persons, archive.families
    );
    for (name, content) in &archive.files {
        bundle.push_str("\n=== ");
        bundle.push_str(name);
        bundle.push_str(" ===\n");
        bundle.push_str(content);
        bundle.push('\n');
    }
    Prompt {
        system,
        bundle,
        brain: asker.map(brain_section).unwrap_or_default(),
        question: question.to_string(),
    }
}

// ---------------------------------------------------------------------------
// the services: one HTTP request each
// ---------------------------------------------------------------------------

/// One request to a service, before it is sent.
#[derive(Debug, Clone)]
pub struct HttpCall {
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Value,
}

/// Ollama reads only `num_ctx` tokens of what it is given and drops the rest
/// without a word: room for the whole archive, within what models offer.
pub(crate) fn num_ctx(tokens: usize) -> usize {
    (tokens + tokens / 5 + 4096).clamp(8192, 131_072)
}

/// What Anthropic's models may write: the answer, and the thinking each
/// current model does first, which counts within it and grows with the
/// effort.
pub(crate) const ANTHROPIC_MAX_TOKENS: u32 = 32_000;

pub fn http_call(cfg: &Config, model: &str, p: &Prompt, tokens: usize) -> Option<HttpCall> {
    let url = cfg.call_url(model)?;
    let key = cfg.api_key.trim();
    let effort = cfg.effort.trim();
    let messages = json!([
        {"role": "system", "content": p.system},
        {"role": "user", "content": p.user()},
    ]);
    let mut call = match cfg.kind.protocol() {
        Protocol::Ollama => HttpCall {
            url,
            headers: Vec::new(),
            body: json!({"model": model, "stream": false, "messages": messages,
                         "options": {"num_ctx": num_ctx(tokens)}}),
        },
        Protocol::OpenAi => HttpCall {
            url,
            headers: if key.is_empty() {
                Vec::new()
            } else {
                vec![("Authorization".into(), format!("Bearer {key}"))]
            },
            body: json!({"model": model, "messages": messages}),
        },
        Protocol::Anthropic => HttpCall {
            url,
            headers: vec![
                ("x-api-key".into(), key.to_string()),
                ("anthropic-version".into(), "2023-06-01".into()),
            ],
            body: json!({"model": model, "max_tokens": ANTHROPIC_MAX_TOKENS, "system": p.system,
                         "messages": [{"role": "user", "content": p.user()}]}),
        },
        Protocol::Gemini => HttpCall {
            url,
            headers: vec![("x-goog-api-key".into(), key.to_string())],
            body: json!({"systemInstruction": {"parts": [{"text": p.system}]},
                         "contents": [{"role": "user", "parts": [{"text": p.user()}]}]}),
        },
        _ => return None,
    };
    apply_effort(cfg.kind.protocol(), &mut call.body, effort);
    Some(call)
}

/// The effort, in the setting each protocol names for it; without one, the
/// model's own default.
pub(crate) fn apply_effort(protocol: Protocol, body: &mut Value, effort: &str) {
    if effort.is_empty() {
        return;
    }
    match protocol {
        Protocol::Ollama => {
            body["think"] = match effort {
                "true" => json!(true),
                "false" => json!(false),
                level => json!(level),
            }
        }
        Protocol::OpenAi => body["reasoning_effort"] = json!(effort),
        Protocol::Anthropic => body["output_config"] = json!({"effort": effort}),
        Protocol::Gemini => {
            body["generationConfig"] = json!({"thinkingConfig": {"thinkingLevel": effort}})
        }
        _ => {}
    }
}

/// What came back.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Answer {
    pub text: String,
    pub model: Option<String>,
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub cost_usd: Option<f64>,
}

/// A failure, said for the administrator: what happened, and what the
/// program or the service said about it.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Failure {
    pub message: String,
    pub detail: Option<String>,
}

impl Failure {
    pub fn new(message: impl Into<String>) -> Self {
        Failure {
            message: message.into(),
            detail: None,
        }
    }
}

/// The last `n` characters of a text, for a failure's detail.
pub(crate) fn tail(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if count <= n {
        return s.trim().to_string();
    }
    format!("…{}", s.chars().skip(count - n).collect::<String>().trim())
}

/// A service's own words for what went wrong.
pub(crate) fn error_message(v: &Value) -> Option<String> {
    v.pointer("/error/message")
        .or_else(|| v.get("error").filter(|e| e.is_string()))
        .or_else(|| v.get("message"))
        .and_then(Value::as_str)
        .map(str::to_string)
}

/// A refusal, in the service's own words when it gave any.
pub(crate) fn http_error(status: u16, body: &str) -> String {
    let said = serde_json::from_str::<Value>(body)
        .ok()
        .as_ref()
        .and_then(error_message)
        .unwrap_or_else(|| tail(body, 400));
    format!("HTTP {status}: {said}")
}

pub fn parse_http(protocol: Protocol, status: u16, body: &str) -> Result<Answer, String> {
    if !(200..300).contains(&status) {
        return Err(http_error(status, body));
    }
    let v: Value = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(_) => return Err(format!("HTTP {status}: {}", tail(body, 400))),
    };
    let n = |p: &str| v.pointer(p).and_then(Value::as_u64);
    let model = |p: &str| v.pointer(p).and_then(Value::as_str).map(str::to_string);
    let joined = |parts: Option<&Value>, key: &str| -> String {
        parts
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|x| x.get(key).and_then(Value::as_str))
            .collect::<Vec<_>>()
            .join("")
    };
    let answer = match protocol {
        Protocol::Ollama => Answer {
            text: v
                .pointer("/message/content")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            model: model("/model"),
            input_tokens: n("/prompt_eval_count"),
            output_tokens: n("/eval_count"),
            cost_usd: None,
        },
        Protocol::OpenAi => {
            let content = v.pointer("/choices/0/message/content");
            Answer {
                text: match content {
                    Some(Value::String(s)) => s.clone(),
                    other => joined(other, "text"),
                },
                model: model("/model"),
                input_tokens: n("/usage/prompt_tokens"),
                output_tokens: n("/usage/completion_tokens"),
                cost_usd: None,
            }
        }
        Protocol::Anthropic => Answer {
            text: v
                .get("content")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter(|b| b.get("type").and_then(Value::as_str) == Some("text"))
                .filter_map(|b| b.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join(""),
            model: model("/model"),
            input_tokens: n("/usage/input_tokens"),
            output_tokens: n("/usage/output_tokens"),
            cost_usd: None,
        },
        Protocol::Gemini => Answer {
            text: joined(v.pointer("/candidates/0/content/parts"), "text"),
            model: model("/modelVersion"),
            input_tokens: n("/usageMetadata/promptTokenCount"),
            output_tokens: n("/usageMetadata/candidatesTokenCount"),
            cost_usd: None,
        },
        _ => return Err("not a service".into()),
    };
    if answer.text.trim().is_empty() {
        let why = v
            .pointer("/candidates/0/finishReason")
            .or_else(|| v.pointer("/choices/0/finish_reason"))
            .or_else(|| v.get("stop_reason"))
            .and_then(Value::as_str)
            .unwrap_or("no text");
        return Err(format!("HTTP {status}: an answer with no text ({why})"));
    }
    Ok(answer)
}

pub(crate) fn http_agent(timeout: Duration) -> ureq::Agent {
    ureq::Agent::new_with_config(
        ureq::Agent::config_builder()
            .timeout_global(Some(timeout))
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .http_status_as_error(false)
            .build(),
    )
}

/// The first model an Ollama server has installed.
pub(crate) fn ollama_model(agent: &ureq::Agent, base: &str) -> Result<String, Failure> {
    let url = format!("{base}/api/tags");
    let mut resp = agent
        .get(&url)
        .call()
        .map_err(|e| Failure::new(format!("{url}: {e}")))?;
    let status = resp.status().as_u16();
    let body = resp
        .body_mut()
        .read_to_string()
        .map_err(|e| Failure::new(format!("{url}: {e}")))?;
    let v: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
    v.pointer("/models/0/name")
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| Failure::new(format!("{url} answered {status} and lists no model")))
}

fn ask_http(cfg: &Config, p: &Prompt, tokens: usize) -> Result<Answer, Failure> {
    let agent = http_agent(cfg.timeout());
    let model = match cfg.model.trim() {
        "" if cfg.kind == Kind::Ollama => ollama_model(&agent, &cfg.base_url())?,
        m => m.to_string(),
    };
    let call = http_call(cfg, &model, p, tokens).ok_or_else(|| Failure::new("not a service"))?;
    let mut req = agent
        .post(&call.url)
        .header("Content-Type", "application/json");
    for (k, v) in &call.headers {
        req = req.header(k.as_str(), v.as_str());
    }
    let mut resp = req
        .send(call.body.to_string())
        .map_err(|e| Failure::new(format!("{}: {e}", call.url)))?;
    let status = resp.status().as_u16();
    let body = resp
        .body_mut()
        .read_to_string()
        .map_err(|e| Failure::new(format!("{}: {e}", call.url)))?;
    parse_http(cfg.kind.protocol(), status, &body)
        .map(|mut a| {
            a.model.get_or_insert(model);
            a
        })
        .map_err(Failure::new)
}

// ---------------------------------------------------------------------------
// the agents on this machine: one command line each
// ---------------------------------------------------------------------------

/// One run of an agent's program, before it starts.
#[derive(Debug, Clone, Default)]
pub struct CliCall {
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub env: Vec<(String, String)>,
    /// Files written into the working directory first.
    pub files: Vec<(String, String)>,
    /// Where the agent writes its answer, when it does not print it.
    pub answer_file: Option<String>,
}

/// OpenCode's permissions while it reads the archive: every tool denied but
/// reading, which its attachment goes through, and reading outside its
/// private working directory (`external_directory`) is denied with the rest.
pub(crate) const OPENCODE_DENY: &str = r#"{"*":"deny","read":"allow"}"#;

/// The file OpenCode is given the archive in.
const OPENCODE_FILE: &str = "axgf-bundle.txt";

/// The file Codex writes its last message to.
const CODEX_ANSWER: &str = "answer.md";

/// Whether a program's own help names an option. Some options the AFG passes
/// came with later versions of an agent: those are passed only when the
/// program in hand names them. A version without one of the others says so
/// itself, in the test's failure.
pub(crate) fn knows(help: &str, option: &str) -> bool {
    help.split(|c: char| c.is_whitespace() || matches!(c, ',' | '=' | '[' | ']'))
        .any(|w| w == option)
}

/// What keeps the account's own setup out of Claude Code, each passed when
/// its help names it: no transcript of the archive kept; none of the
/// account's CLAUDE.md, skills, hooks or plugins (`--safe-mode`, which 2.1.131
/// did not know yet); no MCP server; no skill; nobody to ask for a
/// permission.
pub(crate) const CLAUDE_KEEP_OUT: &[&[&str]] = &[
    &["--no-session-persistence"],
    &["--safe-mode"],
    &["--strict-mcp-config"],
    &["--disable-slash-commands"],
    &["--permission-prompts", "none"],
];

/// Codex's, likewise: not the account's `config.toml` (its MCP servers), and
/// nothing kept.
pub(crate) const CODEX_KEEP_OUT: &[&str] = &["--ignore-user-config", "--ephemeral"];

/// `help` is what the agent's own help printed (`claude --help`,
/// `codex exec --help`), for the options only some versions know.
pub fn cli_call(cfg: &Config, p: &Prompt, work: &Path, help: &str) -> Option<CliCall> {
    let model = cfg.model.trim();
    let effort = cfg.effort.trim();
    let s = |x: &str| x.to_string();
    let mut args: Vec<String>;
    let mut call = CliCall::default();
    match cfg.kind.protocol() {
        Protocol::ClaudeCode => {
            // Print mode, the answer as JSON, no tool at all: the archive is
            // read from the prompt, not from this machine. Then whatever of
            // the account's own setup this version knows how to keep out.
            args = ["-p", "--output-format", "json", "--tools", ""]
                .map(s)
                .to_vec();
            for option in CLAUDE_KEEP_OUT {
                if knows(help, option[0]) {
                    args.extend(option.iter().map(|o| s(o)));
                }
            }
            args.extend([s("--system-prompt"), p.system.clone()]);
            if !model.is_empty() {
                args.extend([s("--model"), s(model)]);
            }
            if !effort.is_empty() {
                args.extend([s("--effort"), s(effort)]);
            }
            call.stdin = Some(p.user());
        }
        Protocol::Codex => {
            // No shell and no web search, so nothing on this machine or
            // beyond is read, and a read-only sandbox besides; the last
            // message to a file, the prompt from standard input (`-`). Not
            // the user's config.toml (its MCP servers) and nothing kept, when
            // this version knows how; its sign-in still comes from
            // CODEX_HOME.
            let answer = work.join(CODEX_ANSWER);
            args = vec![s("exec")];
            args.extend(
                CODEX_KEEP_OUT
                    .iter()
                    .filter(|o| knows(help, o))
                    .map(|o| s(o)),
            );
            args.extend(
                [
                    "-c",
                    "features.shell_tool=false",
                    "-c",
                    "web_search=\"disabled\"",
                    "--skip-git-repo-check",
                    "--sandbox",
                    "read-only",
                    "--color",
                    "never",
                    "--output-last-message",
                ]
                .map(s),
            );
            args.push(answer.to_string_lossy().into_owned());
            if !model.is_empty() {
                args.extend([s("--model"), s(model)]);
            }
            if !effort.is_empty() {
                args.extend([s("-c"), format!("model_reasoning_effort=\"{effort}\"")]);
            }
            args.push(s("-"));
            call.stdin = Some(p.whole());
            call.answer_file = Some(s(CODEX_ANSWER));
        }
        Protocol::OpenCode => {
            // The message first and the attachment last: `--file` takes
            // every word after it as another file.
            let message = format!(
                "{}\n\nThe archive is the attached file {OPENCODE_FILE}.\n\nQuestion: {}",
                p.system, p.question
            );
            args = vec![s("run"), message];
            if !model.is_empty() {
                args.extend([s("--model"), s(model)]);
            }
            if !effort.is_empty() {
                args.extend([s("--variant"), s(effort)]);
            }
            args.extend([
                s("--file"),
                work.join(OPENCODE_FILE).to_string_lossy().into_owned(),
            ]);
            call.files.push((s(OPENCODE_FILE), p.context()));
            call.env.push((s("OPENCODE_PERMISSION"), s(OPENCODE_DENY)));
            // Nobody's plugins, project files, Claude skills or updates.
            for k in [
                "OPENCODE_PURE",
                "OPENCODE_DISABLE_PROJECT_CONFIG",
                "OPENCODE_DISABLE_CLAUDE_CODE",
                "OPENCODE_DISABLE_EXTERNAL_SKILLS",
                "OPENCODE_DISABLE_AUTOUPDATE",
                "OPENCODE_DISABLE_LSP_DOWNLOAD",
            ] {
                call.env.push((s(k), s("1")));
            }
        }
        _ => return None,
    }
    call.args = args;
    Some(call)
}

/// Terminal colours, which an agent's plain output may still carry.
pub(crate) fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                chars.next();
                for d in chars.by_ref() {
                    if d.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
            continue;
        }
        out.push(c);
    }
    out
}

pub fn parse_cli(
    protocol: Protocol,
    stdout: &str,
    answer_file: Option<String>,
) -> Result<Answer, String> {
    let text = match protocol {
        Protocol::ClaudeCode => {
            let trimmed = stdout.trim();
            let v: Value = serde_json::from_str(trimmed)
                .or_else(|_| serde_json::from_str(trimmed.lines().last().unwrap_or_default()))
                .map_err(|_| format!("not the JSON claude -p prints: {}", tail(trimmed, 400)))?;
            let result = v.get("result").and_then(Value::as_str).unwrap_or_default();
            if v.get("is_error").and_then(Value::as_bool) == Some(true) {
                let subtype = v.get("subtype").and_then(Value::as_str).unwrap_or("error");
                return Err(if result.is_empty() {
                    subtype.to_string()
                } else {
                    format!("{subtype}: {result}")
                });
            }
            return Ok(Answer {
                text: result.to_string(),
                model: None,
                input_tokens: v
                    .pointer("/usage/input_tokens")
                    .and_then(Value::as_u64)
                    .map(|n| {
                        n + v
                            .pointer("/usage/cache_read_input_tokens")
                            .and_then(Value::as_u64)
                            .unwrap_or(0)
                            + v.pointer("/usage/cache_creation_input_tokens")
                                .and_then(Value::as_u64)
                                .unwrap_or(0)
                    }),
                output_tokens: v.pointer("/usage/output_tokens").and_then(Value::as_u64),
                cost_usd: v.get("total_cost_usd").and_then(Value::as_f64),
            })
            .and_then(|a| {
                if a.text.trim().is_empty() {
                    Err("an answer with no text".to_string())
                } else {
                    Ok(a)
                }
            });
        }
        Protocol::Codex => answer_file.unwrap_or_else(|| stdout.to_string()),
        Protocol::OpenCode => strip_ansi(stdout),
        _ => return Err("not an agent".into()),
    };
    let text = text.trim().to_string();
    if text.is_empty() {
        return Err("an answer with no text".into());
    }
    Ok(Answer {
        text,
        ..Answer::default()
    })
}

fn is_executable(p: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(p).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}

/// The agent's program: the one configured, else the first on the search
/// path and the places its installers put it.
pub fn find_program(kind: Kind, configured: &str) -> Option<PathBuf> {
    let name = kind.program();
    if name.is_empty() {
        return None;
    }
    let configured = configured.trim();
    if !configured.is_empty() {
        let p = PathBuf::from(configured);
        return is_executable(&p).then_some(p);
    }
    let mut dirs: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        for d in [
            ".local/bin",
            ".npm-global/bin",
            ".bun/bin",
            ".opencode/bin",
            ".cargo/bin",
        ] {
            dirs.push(home.join(d));
        }
    }
    dirs.extend(["/usr/local/bin", "/usr/bin", "/opt/homebrew/bin"].map(PathBuf::from));
    dirs.into_iter()
        .map(|d| d.join(name))
        .find(|p| is_executable(p))
}

/// An agent's command, run in its working directory, without this site's own
/// settings in its environment: the service's environment file holds the
/// emergency token.
pub(crate) fn agent_command(program: &Path, work: &Path) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new(program);
    cmd.current_dir(work).kill_on_drop(true);
    for (k, _) in std::env::vars_os() {
        if k.to_string_lossy().starts_with("AXGF_") {
            cmd.env_remove(&k);
        }
    }
    cmd
}

/// What an agent's help says, for the options its version knows (`knows`).
pub(crate) async fn help_of(program: &Path, args: &[&str], work: &Path) -> String {
    let mut cmd = agent_command(program, work);
    cmd.args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    match tokio::time::timeout(Duration::from_secs(30), cmd.output()).await {
        Ok(Ok(out)) => format!(
            "{}\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ),
        _ => String::new(),
    }
}

async fn ask_cli(cfg: &Config, p: &Prompt, base: &Path) -> Result<Answer, Failure> {
    use tokio::io::AsyncWriteExt;
    let protocol = cfg.kind.protocol();
    let program = find_program(cfg.kind, &cfg.program).ok_or_else(|| {
        Failure::new(format!(
            "{}: not found on this machine",
            match cfg.program.trim() {
                "" => cfg.kind.program(),
                p => p,
            }
        ))
    })?;
    // One test at a time (`Runs`): another working directory here is one a
    // process stopped mid-test left behind, with a copy of the archive in it.
    if let Ok(entries) = std::fs::read_dir(base) {
        for e in entries.flatten() {
            let name = e.file_name();
            let leftover = name
                .to_str()
                .and_then(|n| n.strip_prefix(".afg-"))
                .is_some_and(|id| uuid::Uuid::parse_str(id).is_ok());
            if leftover {
                let _ = std::fs::remove_dir_all(e.path());
            }
        }
    }
    // A private working directory beside the bundle, gone afterwards.
    let work = base.join(format!(".afg-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&work).map_err(|e| Failure::new(format!("{}: {e}", work.display())))?;
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&work, std::fs::Permissions::from_mode(0o700));
    }
    let outcome = async {
        let help = match protocol {
            Protocol::ClaudeCode => help_of(&program, &["--help"], &work).await,
            Protocol::Codex => help_of(&program, &["exec", "--help"], &work).await,
            _ => String::new(),
        };
        let call = cli_call(cfg, p, &work, &help).ok_or_else(|| Failure::new("not an agent"))?;
        for (name, content) in &call.files {
            std::fs::write(work.join(name), content)
                .map_err(|e| Failure::new(format!("{name}: {e}")))?;
        }
        let mut cmd = agent_command(&program, &work);
        cmd.args(&call.args)
            .stdin(if call.stdin.is_some() {
                std::process::Stdio::piped()
            } else {
                std::process::Stdio::null()
            })
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        for (k, v) in &call.env {
            cmd.env(k, v);
        }
        let mut child = cmd
            .spawn()
            .map_err(|e| Failure::new(format!("{}: {e}", program.display())))?;
        // Written while the output is read, so that neither pipe can fill
        // and stop the other.
        let writer = match (call.stdin.clone(), child.stdin.take()) {
            (Some(input), Some(mut pipe)) => Some(tokio::spawn(async move {
                let _ = pipe.write_all(input.as_bytes()).await;
                let _ = pipe.shutdown().await;
            })),
            _ => None,
        };
        let waited = tokio::time::timeout(cfg.timeout(), child.wait_with_output()).await;
        if let Some(w) = writer {
            w.abort();
        }
        let out = match waited {
            Err(_) => {
                return Err(Failure::new(format!(
                    "{}: no answer after {} s",
                    cfg.kind.program(),
                    cfg.timeout().as_secs()
                )))
            }
            Ok(Err(e)) => return Err(Failure::new(format!("{}: {e}", program.display()))),
            Ok(Ok(o)) => o,
        };
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        let answer_file = call
            .answer_file
            .as_ref()
            .and_then(|f| std::fs::read_to_string(work.join(f)).ok())
            .filter(|t| !t.trim().is_empty());
        let parsed = parse_cli(protocol, &stdout, answer_file);
        match (out.status.success(), parsed) {
            (true, Ok(a)) => Ok(a),
            (true, Err(m)) => Err(Failure {
                message: m,
                detail: Some(tail(&stderr, 2000)).filter(|d| !d.is_empty()),
            }),
            (false, parsed) => Err(Failure {
                message: match parsed {
                    // claude -p says its own error in its JSON.
                    Err(m)
                        if protocol == Protocol::ClaudeCode && !m.starts_with("not the JSON") =>
                    {
                        m
                    }
                    _ => format!("{} exited with {}", cfg.kind.program(), out.status),
                },
                detail: Some(tail(&format!("{stderr}\n{stdout}"), 2000)).filter(|d| !d.is_empty()),
            }),
        }
    }
    .await;
    let _ = std::fs::remove_dir_all(&work);
    outcome
}

/// Ask the AFG: the archive and a question, to wherever its computing comes
/// from.
pub async fn ask(cfg: Config, p: Prompt, tokens: usize, base: PathBuf) -> Result<Answer, Failure> {
    match cfg.kind.protocol() {
        Protocol::None => Err(Failure::new("the AFG is off")),
        Protocol::Ollama | Protocol::OpenAi | Protocol::Anthropic | Protocol::Gemini => {
            tokio::task::spawn_blocking(move || ask_http(&cfg, &p, tokens))
                .await
                .map_err(|e| Failure::new(e.to_string()))?
        }
        Protocol::ClaudeCode | Protocol::Codex | Protocol::OpenCode => {
            ask_cli(&cfg, &p, &base).await
        }
    }
}

// ---------------------------------------------------------------------------
// test runs
// ---------------------------------------------------------------------------

/// One test, running or finished.
#[derive(Debug, Clone, Serialize)]
pub struct Run {
    pub id: String,
    pub kind: &'static str,
    pub model: String,
    pub effort: String,
    pub endpoint: Option<String>,
    pub started: String,
    #[serde(skip)]
    since: Instant,
    /// `running`, `done` or `failed`.
    pub state: &'static str,
    pub seconds: u64,
    pub question: String,
    pub archive: Archive,
    pub answer: Option<Answer>,
    pub failure: Option<Failure>,
    /// Whose brain the test went through (docs/adr/0024): the asking
    /// account's name; none for a session with no account.
    pub brain: Option<String>,
    /// What the AFG kept in that brain from its answer.
    pub remembered: Vec<String>,
}

impl Run {
    pub fn new(cfg: &Config, archive: &Archive, question: &str, brain: Option<&str>) -> Self {
        Run {
            id: uuid::Uuid::new_v4().to_string(),
            kind: cfg.kind.slug(),
            model: cfg.model.trim().to_string(),
            effort: cfg.effort.trim().to_string(),
            endpoint: cfg.endpoint(),
            started: crate::view::now_iso8601(),
            since: Instant::now(),
            state: "running",
            seconds: 0,
            question: question.to_string(),
            archive: archive.summary(),
            answer: None,
            failure: None,
            brain: brain.map(str::to_string),
            remembered: Vec::new(),
        }
    }
}

/// The tests of this process's life, the last few; one at a time.
#[derive(Debug, Default)]
pub struct Runs {
    runs: Mutex<Vec<Run>>,
}

const KEPT: usize = 10;

impl Runs {
    /// Start a run, unless one is running: then that one.
    pub fn begin(&self, run: Run) -> Result<(), Box<Run>> {
        let mut runs = self.runs.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(r) = runs.iter().find(|r| r.state == "running") {
            return Err(Box::new(r.clone()));
        }
        runs.push(run);
        let n = runs.len();
        if n > KEPT {
            runs.drain(..n - KEPT);
        }
        Ok(())
    }

    /// A test answered or failed, and what its brain kept.
    pub fn end(&self, id: &str, outcome: Result<Answer, Failure>, remembered: Vec<String>) {
        let mut runs = self.runs.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(r) = runs.iter_mut().find(|r| r.id == id) {
            r.seconds = r.since.elapsed().as_secs();
            match outcome {
                Ok(a) => {
                    r.state = "done";
                    r.answer = Some(a);
                    r.remembered = remembered;
                }
                Err(f) => {
                    r.state = "failed";
                    r.failure = Some(f);
                }
            }
        }
    }

    pub fn get(&self, id: &str) -> Option<Run> {
        let runs = self.runs.lock().unwrap_or_else(|e| e.into_inner());
        runs.iter().find(|r| r.id == id).cloned().map(Self::now)
    }

    pub fn latest(&self) -> Option<Run> {
        let runs = self.runs.lock().unwrap_or_else(|e| e.into_inner());
        runs.last().cloned().map(Self::now)
    }

    /// A running test's seconds so far.
    fn now(mut r: Run) -> Run {
        if r.state == "running" {
            r.seconds = r.since.elapsed().as_secs();
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(kind: Kind) -> Config {
        Config {
            kind,
            api_key: "sk-test-0123456789abcd".into(),
            model: "m1".into(),
            ..Config::default()
        }
    }

    fn sample() -> Prompt {
        Prompt {
            system: "SYS".into(),
            bundle: "BUNDLE".into(),
            brain: String::new(),
            question: "Q?".into(),
        }
    }

    #[test]
    fn every_kind_reads_back_from_its_slug() {
        for k in Kind::ALL {
            assert_eq!(Kind::parse(k.slug()), Some(k));
            let v = serde_json::to_value(k).unwrap();
            assert_eq!(v, json!(k.slug()), "stored as its slug");
        }
        assert_eq!(Kind::parse("bash"), None);
    }

    #[test]
    fn a_configuration_says_what_is_missing() {
        assert_eq!(Config::default().problem(), None, "off needs nothing");
        let mut c = cfg(Kind::Openai);
        assert_eq!(c.problem(), None);
        c.api_key.clear();
        assert_eq!(c.problem(), Some("afg-error-key"));
        let mut c = cfg(Kind::Gemini);
        c.model.clear();
        assert_eq!(c.problem(), Some("afg-error-model"));
        c.model = "gemini; rm -rf /".into();
        assert_eq!(c.problem(), Some("afg-error-model"));
        let mut c = cfg(Kind::Anthropic);
        c.url = "ftp://example.org".into();
        assert_eq!(c.problem(), Some("afg-error-url"));
        // Ollama needs neither key nor model; vLLM no key.
        assert_eq!(
            Config {
                kind: Kind::Ollama,
                ..Config::default()
            }
            .problem(),
            None
        );
        assert_eq!(
            Config {
                kind: Kind::Vllm,
                model: "q".into(),
                ..Config::default()
            }
            .problem(),
            None
        );
        // Any effort written plainly, suggested or not (docs/adr/0025); never
        // one read as an option. An agent's program is its own.
        let mut c = Config {
            kind: Kind::ClaudeCode,
            effort: "high".into(),
            ..Config::default()
        };
        assert_eq!(c.problem(), None);
        c.effort = "turbo".into();
        assert_eq!(c.problem(), None, "not suggested, sent as it is");
        for wrong in ["--max", "high\" model=\"x", "a b"] {
            c.effort = wrong.into();
            assert_eq!(c.problem(), Some("afg-error-effort"), "{wrong}");
        }
        c.effort = "max".into();
        c.program = "/bin/sh".into();
        assert_eq!(c.problem(), Some("afg-error-program"));
        c.program = "/usr/local/bin/claude".into();
        assert_eq!(c.problem(), None);
        c.program = "claude".into();
        assert_eq!(c.problem(), Some("afg-error-program"), "a full path");
        let c = Config {
            kind: Kind::Opencode,
            effort: "high-thinking".into(),
            ..Config::default()
        };
        assert_eq!(c.problem(), None, "OpenCode's variants are its providers'");
        let mut c = cfg(Kind::Mistral);
        c.effort = "none".into();
        assert_eq!(c.problem(), None, "a service takes an effort too");
        assert!(!plain("--model"), "never read as an option");
    }

    #[test]
    fn each_service_is_spoken_to_in_its_own_protocol() {
        let p = sample();
        let c = Config {
            kind: Kind::Ollama,
            ..Config::default()
        };
        let call = http_call(&c, "llama3.1:8b", &p, 100_000).unwrap();
        assert_eq!(call.url, "http://127.0.0.1:11434/api/chat");
        assert_eq!(call.body["stream"], false);
        assert_eq!(call.body["messages"][0]["content"], "SYS");
        assert!(call.body["options"]["num_ctx"].as_u64().unwrap() > 100_000);

        let c = cfg(Kind::Mistral);
        let call = http_call(&c, "m1", &p, 10).unwrap();
        assert_eq!(call.url, "https://api.mistral.ai/v1/chat/completions");
        assert_eq!(
            call.headers,
            vec![(
                "Authorization".to_string(),
                "Bearer sk-test-0123456789abcd".to_string()
            )]
        );

        let mut c = cfg(Kind::Vllm);
        c.api_key.clear();
        c.url = "http://gpu.lan:8000/v1/".into();
        let call = http_call(&c, "m1", &p, 10).unwrap();
        assert_eq!(call.url, "http://gpu.lan:8000/v1/chat/completions");
        assert!(call.headers.is_empty(), "no key, no header");

        let call = http_call(&cfg(Kind::Anthropic), "m1", &p, 10).unwrap();
        assert_eq!(call.url, "https://api.anthropic.com/v1/messages");
        assert!(call
            .headers
            .iter()
            .any(|(k, v)| k == "anthropic-version" && v == "2023-06-01"));
        assert_eq!(call.body["system"], "SYS");
        assert_eq!(
            call.body["messages"][0]["content"],
            "BUNDLE\n\nQuestion: Q?"
        );

        let call = http_call(&cfg(Kind::Gemini), "models/gemini-2.5-pro", &p, 10).unwrap();
        assert_eq!(
            call.url,
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent"
        );
        assert_eq!(call.headers[0].0, "x-goog-api-key");
        assert_eq!(call.body["systemInstruction"]["parts"][0]["text"], "SYS");
    }

    #[test]
    fn each_service_is_given_the_effort_in_its_own_setting() {
        let p = sample();
        let with = |kind, effort: &str| {
            let mut c = cfg(kind);
            c.effort = effort.into();
            http_call(&c, "m1", &p, 10).unwrap().body
        };
        // Without one, nothing is said and the model's own default holds.
        for kind in [Kind::Ollama, Kind::Openai, Kind::Anthropic, Kind::Gemini] {
            let body = with(kind, "");
            for field in ["think", "reasoning_effort", "output_config"] {
                assert!(body.get(field).is_none(), "{kind:?} {field}");
            }
            assert!(body.get("generationConfig").is_none(), "{kind:?}");
        }
        // Ollama's `think`: a switch, or a level the model names.
        assert_eq!(with(Kind::Ollama, "false")["think"], json!(false));
        assert_eq!(with(Kind::Ollama, "true")["think"], json!(true));
        assert_eq!(with(Kind::Ollama, "high")["think"], json!("high"));
        for kind in [Kind::Openai, Kind::Mistral, Kind::Vllm] {
            assert_eq!(with(kind, "xhigh")["reasoning_effort"], "xhigh", "{kind:?}");
        }
        let body = with(Kind::Anthropic, "max");
        assert_eq!(body["output_config"], json!({"effort": "max"}));
        assert_eq!(body["max_tokens"], 32_000, "room for its thinking");
        assert_eq!(
            with(Kind::Gemini, "minimal")["generationConfig"],
            json!({"thinkingConfig": {"thinkingLevel": "minimal"}})
        );
    }

    #[test]
    fn each_answer_is_read_in_its_own_shape() {
        let ok = |p, body: &str| parse_http(p, 200, body).unwrap();
        let a = ok(
            Protocol::Ollama,
            r#"{"model":"llama3","message":{"role":"assistant","content":"Les Delaunay."},"prompt_eval_count":120,"eval_count":9}"#,
        );
        assert_eq!(
            (a.text.as_str(), a.input_tokens, a.output_tokens),
            ("Les Delaunay.", Some(120), Some(9))
        );
        let a = ok(
            Protocol::OpenAi,
            r#"{"model":"gpt","choices":[{"message":{"content":"Un."}}],"usage":{"prompt_tokens":5,"completion_tokens":2}}"#,
        );
        assert_eq!(a.text, "Un.");
        let a = ok(
            Protocol::Anthropic,
            r#"{"model":"claude","content":[{"type":"thinking","thinking":"…"},{"type":"text","text":"Deux."}],"usage":{"input_tokens":7,"output_tokens":3}}"#,
        );
        assert_eq!(a.text, "Deux.");
        let a = ok(
            Protocol::Gemini,
            r#"{"candidates":[{"content":{"parts":[{"text":"Trois"},{"text":"."}]}}],"usageMetadata":{"promptTokenCount":4,"candidatesTokenCount":1},"modelVersion":"gemini-2.5-pro"}"#,
        );
        assert_eq!(
            (a.text.as_str(), a.model.as_deref()),
            ("Trois.", Some("gemini-2.5-pro"))
        );
        // A service's own words for its refusal.
        let e = parse_http(
            Protocol::OpenAi,
            401,
            r#"{"error":{"message":"Incorrect API key provided."}}"#,
        )
        .unwrap_err();
        assert_eq!(e, "HTTP 401: Incorrect API key provided.");
        let e = parse_http(Protocol::Anthropic, 400, r#"{"type":"error","error":{"type":"invalid_request_error","message":"prompt is too long: 431000 tokens > 200000 maximum"}}"#).unwrap_err();
        assert!(e.contains("prompt is too long"));
        let e = parse_http(
            Protocol::Gemini,
            200,
            r#"{"candidates":[{"finishReason":"SAFETY"}]}"#,
        )
        .unwrap_err();
        assert!(e.contains("SAFETY"));
        assert!(
            parse_http(Protocol::Ollama, 502, "<html>Bad gateway</html>")
                .unwrap_err()
                .starts_with("HTTP 502")
        );
    }

    /// What `claude --help` names, as the two versions on the machine this
    /// was written on print it.
    const CLAUDE_HELP_2_1_300: &str = "  --disable-slash-commands  Disable all skills\n  \
        --effort <level>\n  --no-session-persistence  Disable\n  --output-format <format>\n  \
        --permission-prompts <target>  Who answers\n  --safe-mode  Start with\n  \
        --strict-mcp-config  Only use\n  --system-prompt <prompt>\n  --tools <tools...>";
    const CLAUDE_HELP_2_1_131: &str = "  --bare  Minimal mode\n  --disable-slash-commands\n  \
        --effort <level>\n  --no-session-persistence\n  --output-format <format>\n  \
        --setting-sources <sources>\n  --strict-mcp-config\n  --system-prompt <prompt>\n  \
        --tools <tools...>";

    #[test]
    fn each_agent_is_run_as_its_own_command_line() {
        let p = sample();
        let work = Path::new("/w");
        let c = Config {
            kind: Kind::ClaudeCode,
            model: "sonnet[1m]".into(),
            effort: "high".into(),
            ..Config::default()
        };
        let call = cli_call(&c, &p, work, CLAUDE_HELP_2_1_300).unwrap();
        let a = call.args.join(" ");
        assert!(a.starts_with("-p --output-format json --tools  --no-session-persistence --safe-mode --strict-mcp-config --disable-slash-commands --permission-prompts none --system-prompt SYS"), "{a}");
        assert!(
            call.args.windows(2).any(|w| w == ["--tools", ""]),
            "no tool at all"
        );
        assert!(call.args.windows(2).any(|w| w == ["--model", "sonnet[1m]"]));
        assert!(call.args.windows(2).any(|w| w == ["--effort", "high"]));
        assert!(call
            .args
            .windows(2)
            .any(|w| w == ["--system-prompt", "SYS"]));
        assert_eq!(
            call.stdin.as_deref(),
            Some("BUNDLE\n\nQuestion: Q?"),
            "the archive through stdin"
        );
        // An older Claude Code knows no safe mode and asks nobody anyway:
        // what it knows is passed, and never an option it would refuse.
        let old = cli_call(&c, &p, work, CLAUDE_HELP_2_1_131).unwrap();
        assert!(old.args.windows(2).any(|w| w == ["--tools", ""]));
        for gone in ["--safe-mode", "--permission-prompts"] {
            assert!(!old.args.iter().any(|a| a == gone), "{gone}");
        }
        for kept in ["--no-session-persistence", "--strict-mcp-config"] {
            assert!(old.args.iter().any(|a| a == kept), "{kept}");
        }
        // Not even the help: only what is needed.
        let bare = cli_call(&c, &p, work, "").unwrap().args.join(" ");
        assert!(
            bare.starts_with("-p --output-format json --tools  --system-prompt SYS"),
            "{bare}"
        );

        let c = Config {
            kind: Kind::Codex,
            model: "gpt-5".into(),
            effort: "xhigh".into(),
            ..Config::default()
        };
        let call = cli_call(
            &c,
            &p,
            work,
            "  --ignore-user-config  Do not load\n  --ephemeral  Run without",
        )
        .unwrap();
        assert_eq!(call.args[0], "exec");
        let old = cli_call(&c, &p, work, "  --json  Print events").unwrap();
        assert!(!old
            .args
            .iter()
            .any(|a| a == "--ignore-user-config" || a == "--ephemeral"));
        assert!(
            old.args
                .windows(2)
                .any(|w| w == ["-c", "features.shell_tool=false"]),
            "no shell, whatever the version"
        );
        assert!(
            call.args.contains(&"--ignore-user-config".to_string()),
            "not the user's MCP servers"
        );
        assert!(
            call.args
                .windows(2)
                .any(|w| w == ["-c", "features.shell_tool=false"]),
            "no shell"
        );
        assert!(
            call.args
                .windows(2)
                .any(|w| w == ["-c", "web_search=\"disabled\""]),
            "no web"
        );
        assert!(call
            .args
            .windows(2)
            .any(|w| w == ["--sandbox", "read-only"]));
        assert!(call.args.contains(&"--ephemeral".to_string()));
        assert!(call
            .args
            .windows(2)
            .any(|w| w == ["-c", "model_reasoning_effort=\"xhigh\""]));
        assert!(call
            .args
            .windows(2)
            .any(|w| w == ["--output-last-message", "/w/answer.md"]));
        assert_eq!(
            call.args.last().map(String::as_str),
            Some("-"),
            "the prompt from stdin"
        );
        assert!(call.stdin.as_deref().unwrap().starts_with("SYS\n\nBUNDLE"));

        let c = Config {
            kind: Kind::Opencode,
            model: "anthropic/claude-sonnet-4".into(),
            effort: "high".into(),
            ..Config::default()
        };
        let call = cli_call(&c, &p, work, "").unwrap();
        assert_eq!(call.args[0], "run");
        assert!(call.args[1].contains("Question: Q?") && !call.args[1].contains("BUNDLE"));
        assert_eq!(
            &call.args[call.args.len() - 2..],
            ["--file", "/w/axgf-bundle.txt"],
            "the file last"
        );
        assert!(call.args.windows(2).any(|w| w == ["--variant", "high"]));
        assert_eq!(
            call.files,
            vec![("axgf-bundle.txt".to_string(), "BUNDLE".to_string())]
        );
        let perm = call
            .env
            .iter()
            .find(|(k, _)| k == "OPENCODE_PERMISSION")
            .map(|(_, v)| v.as_str());
        let perm: Value = serde_json::from_str(perm.unwrap()).unwrap();
        assert_eq!(
            perm,
            json!({"*": "deny", "read": "allow"}),
            "every tool denied but reading"
        );
        assert!(call
            .env
            .iter()
            .any(|(k, v)| k == "OPENCODE_PURE" && v == "1"));
        assert!(cli_call(&cfg(Kind::Openai), &p, work, "").is_none());
    }

    #[test]
    fn each_agents_answer_is_read_in_its_own_shape() {
        let a = parse_cli(
            Protocol::ClaudeCode,
            r#"{"type":"result","subtype":"success","is_error":false,"result":"Les Zając.","total_cost_usd":0.42,"usage":{"input_tokens":10,"cache_read_input_tokens":5,"output_tokens":3}}"#,
            None,
        )
        .unwrap();
        assert_eq!(
            (a.text.as_str(), a.cost_usd, a.input_tokens),
            ("Les Zając.", Some(0.42), Some(15))
        );
        let e = parse_cli(Protocol::ClaudeCode, r#"{"type":"result","subtype":"error_during_execution","is_error":true,"result":"Prompt is too long"}"#, None).unwrap_err();
        assert_eq!(e, "error_during_execution: Prompt is too long");
        let a = parse_cli(Protocol::Codex, "progress…", Some("Réponse.\n".into())).unwrap();
        assert_eq!(a.text, "Réponse.");
        let a = parse_cli(Protocol::OpenCode, "\u{1b}[1mLes\u{1b}[0m Delaunay\n", None).unwrap();
        assert_eq!(a.text, "Les Delaunay");
    }

    #[test]
    fn the_archive_is_its_json_files_without_its_binaries_or_sensitive_classes() {
        let flat = json!({
            "manifest": {"axgf": "1.1", "family": {"name": "M"}},
            "persons": {
                "a0000000-0000-4000-8000-000000000001": {"id": "a0000000-0000-4000-8000-000000000001",
                    "type": "person", "axgf_version": "1.1",
                    "identity": {"name": {"display": "Ama"}, "is_living": false, "visibility": "public"},
                    "health": {"blood_group": {"value": "O"}}}
            },
            "families": {}, "events": {}, "links": {}, "occupations": {}, "sources": {}, "places": {},
            "documents": {"d0000000-0000-4000-8000-000000000001": {"id": "d0000000-0000-4000-8000-000000000001",
                "type": "document", "axgf_version": "1.1", "title": "Portrait",
                "file": {"path": "documents/files/portrait.jpg", "mime_type": "image/jpeg"}}},
            // As the site holds a large archive: its photographs on disk, the
            // JSON only saying they exist; and one held in the JSON itself.
            "external_payloads": {"documents/files/portrait.jpg": {"size_bytes": 4, "crc32": 1}},
            "attachments": {"documents/files/scan.png": "iVBORw0KGgo="}
        });
        let a = archive(&flat, false).unwrap();
        assert!(
            a.files
                .iter()
                .any(|(n, c)| n == "documents/index.json" && c.contains("portrait.jpg")),
            "what a document says of its file goes, the file does not"
        );
        assert_eq!(a.files[0].0, "manifest.json", "the manifest first");
        assert!(a.files.iter().all(|(n, _)| n.ends_with(".json")));
        assert!(a.files.iter().any(|(n, _)| n.starts_with("persons/")));
        let person = &a
            .files
            .iter()
            .find(|(n, _)| n.starts_with("persons/"))
            .unwrap()
            .1;
        assert!(
            !person.contains("blood_group"),
            "health stays home unless chosen"
        );
        assert!(!person.contains('\n'), "one line each");
        assert_eq!((a.persons, a.families, a.is_empty()), (1, 0, false));
        let with = archive(&flat, true).unwrap();
        assert!(with.files.iter().any(|(_, c)| c.contains("blood_group")));
        let p = prompt(&a, "Combien ?", "Bundle vide", None);
        assert!(p.system.ends_with("answer exactly: Bundle vide"));
        assert!(p.bundle.contains("=== manifest.json ==="));
        assert!(
            p.brain.is_empty() && !p.system.contains("<memory>"),
            "nobody's brain"
        );
        // With the asker's brain: after the archive, before the question.
        let recall = crate::brain::Recall {
            memory: "- [2026-11-01] Researches the Exemple branch".into(),
            recent: vec![crate::brain::Exchange {
                at: "2026-11-01T10:00:00Z".into(),
                context: "test".into(),
                question: "Who is the eldest?".into(),
                answer: "Ama.".into(),
                ..Default::default()
            }],
        };
        let asker = Asker {
            name: "marie",
            person: Some(("a0000000-0000-4000-8000-000000000001", "Ama")),
            recall: &recall,
        };
        let p = prompt(&a, "Combien ?", "Bundle vide", Some(&asker));
        assert!(p.system.contains("their brain") && p.system.contains("<memory> block"));
        let user = p.user();
        let (bundle_at, brain_at, question_at) = (
            user.find("=== manifest.json ===").unwrap(),
            user.find("=== brain: marie ===").unwrap(),
            user.find("Question: Combien ?").unwrap(),
        );
        assert!(bundle_at < brain_at && brain_at < question_at, "{user}");
        for said in [
            "who says they are Ama",
            "Researches the Exemple branch",
            "Who is the eldest?",
        ] {
            assert!(p.brain.contains(said), "{said}");
        }
        let first = crate::brain::Recall::default();
        let p = prompt(
            &a,
            "Q",
            "Bundle vide",
            Some(&Asker {
                name: "lea",
                person: None,
                recall: &first,
            }),
        );
        assert!(p.brain.contains("first exchange"));
        let empty = json!({"manifest": {"axgf": "1.1"}, "persons": {}, "families": {}});
        assert!(archive(&empty, false).unwrap().is_empty());
    }

    #[test]
    fn one_test_at_a_time_and_the_last_few_kept() {
        let runs = Runs::default();
        let a = Archive {
            files: vec![],
            count: 0,
            bytes: 0,
            tokens: 0,
            persons: 0,
            families: 0,
            sensitive: false,
        };
        let first = Run::new(&cfg(Kind::Openai), &a, "Q", Some("marie"));
        let id = first.id.clone();
        runs.begin(first).unwrap();
        assert!(
            runs.begin(Run::new(&cfg(Kind::Openai), &a, "Q", None))
                .is_err(),
            "one at a time"
        );
        runs.end(
            &id,
            Ok(Answer {
                text: "A".into(),
                ..Answer::default()
            }),
            vec!["Writes in French".into()],
        );
        let done = runs.get(&id).unwrap();
        assert_eq!(done.state, "done");
        assert_eq!(
            (done.brain.as_deref(), done.remembered.len()),
            (Some("marie"), 1)
        );
        for _ in 0..12 {
            let r = Run::new(&cfg(Kind::Openai), &a, "Q", None);
            let rid = r.id.clone();
            runs.begin(r).unwrap();
            runs.end(&rid, Err(Failure::new("no")), Vec::new());
        }
        assert!(runs.get(&id).is_none(), "only the last ten");
        assert_eq!(runs.latest().unwrap().state, "failed");
        // The key never reaches a page: only its end.
        assert_eq!(cfg(Kind::Openai).key_hint().as_deref(), Some("…abcd"));
        assert!(
            !format!("{:?}", cfg(Kind::Openai)).contains("0123456789"),
            "nor a log"
        );
    }
}
