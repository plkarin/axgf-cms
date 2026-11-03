//! The AFG in conversation (the operator's request of 3 November 2026,
//! docs/adr/0026): instructions, the family's archive when a project takes
//! it, and the turns of a conversation with the files they carry, to
//! wherever the AFG's computing comes from (docs/adr/0023).
//!
//! Every service is asked to stream, and every piece of the answer is added
//! to a [`Partial`] as it arrives, which the conversation's page reads while
//! the AFG writes:
//!
//! * Ollama: `"stream": true`, one JSON object a line;
//! * OpenAI, Mistral, vLLM: server-sent events of `choices[0].delta`;
//! * Claude (Anthropic API): server-sent events, `content_block_delta`;
//! * Gemini: `:streamGenerateContent?alt=sse`.
//!
//! Claude Code writes its answer as it comes too
//! (`--output-format stream-json --include-partial-messages`, when the
//! version installed knows them); Codex and OpenCode give theirs whole, at
//! the end.
//!
//! Each file goes the way its protocol reads it: pictures to every service
//! and to Claude Code and Codex, PDF documents to Claude, Gemini and OpenAI,
//! sound and film to Gemini only. What a way of computing cannot read is
//! named to it instead, so that it says so rather than guesses.

use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::{Arc, Mutex};

use base64::Engine;
use serde_json::{json, Value};

use crate::afg::{self, Answer, Config, Failure, Kind, Protocol};

/// What the AFG has written so far of the answer it is writing.
pub type Partial = Arc<Mutex<String>>;

fn push(partial: &Partial, s: &str) {
    partial
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push_str(s);
}

fn set(partial: &Partial, s: &str) {
    let mut p = partial.lock().unwrap_or_else(|e| e.into_inner());
    p.clear();
    p.push_str(s);
}

/// What is read of an answer, at most: past it, the reading stops.
const ANSWER_MAX: usize = 2 * 1024 * 1024;

/// Who speaks in a turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speaker {
    User,
    Assistant,
}

/// A piece of a turn.
#[derive(Debug, Clone, PartialEq)]
pub enum Part {
    Text(String),
    /// A picture, sized for a model.
    Image {
        mime: String,
        name: String,
        data: Vec<u8>,
    },
    Pdf {
        name: String,
        data: Vec<u8>,
    },
    /// Sound or film.
    Media {
        mime: String,
        name: String,
        data: Vec<u8>,
    },
}

/// One turn of a conversation: the member's, or the AFG's.
#[derive(Debug, Clone, PartialEq)]
pub struct Turn {
    pub speaker: Speaker,
    pub parts: Vec<Part>,
}

/// Everything the AFG is given for one answer.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChatPrompt {
    /// Its instructions: who it is, the project's code of conduct and memory.
    pub system: String,
    /// The family's archive, when the project takes it; empty otherwise.
    /// Kept apart: a program's command line could not carry it.
    pub context: String,
    pub turns: Vec<Turn>,
}

impl ChatPrompt {
    /// The instructions with the archive after them, as a service takes them.
    fn system_whole(&self) -> String {
        if self.context.is_empty() {
            self.system.clone()
        } else {
            format!("{}\n\n{}", self.system, self.context)
        }
    }

    /// Roughly how many tokens it is, a token for three bytes.
    fn tokens(&self) -> usize {
        let parts: usize = self
            .turns
            .iter()
            .flat_map(|t| &t.parts)
            .map(|p| match p {
                Part::Text(s) => s.len(),
                _ => 3 * 1024,
            })
            .sum();
        (self.system.len() + self.context.len() + parts) / 3
    }
}

fn b64(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

fn data_url(mime: &str, data: &[u8]) -> String {
    format!("data:{mime};base64,{}", b64(data))
}

/// A file a way of computing cannot read, named to it.
fn unread(what: &str, name: &str) -> String {
    format!("[{what} \"{name}\" was sent here, which this way of computing cannot read.]")
}

fn texts(parts: &[Part]) -> String {
    parts
        .iter()
        .filter_map(|p| match p {
            Part::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

// ---------------------------------------------------------------------------
// the services: one streamed request each
// ---------------------------------------------------------------------------

fn ollama_messages(p: &ChatPrompt) -> Value {
    let mut out = vec![json!({"role": "system", "content": p.system_whole()})];
    for t in &p.turns {
        let mut words = Vec::new();
        let mut images = Vec::new();
        for part in &t.parts {
            match part {
                Part::Text(s) => words.push(s.clone()),
                Part::Image { data, .. } => images.push(b64(data)),
                Part::Pdf { name, .. } => words.push(unread("A PDF document", name)),
                Part::Media { name, .. } => words.push(unread("A sound or video file", name)),
            }
        }
        let role = match t.speaker {
            Speaker::User => "user",
            Speaker::Assistant => "assistant",
        };
        let mut m = json!({"role": role, "content": words.join("\n\n")});
        if !images.is_empty() {
            m["images"] = json!(images);
        }
        out.push(m);
    }
    Value::Array(out)
}

fn openai_messages(kind: Kind, p: &ChatPrompt) -> Value {
    let mut out = vec![json!({"role": "system", "content": p.system_whole()})];
    for t in &p.turns {
        if t.speaker == Speaker::Assistant {
            out.push(json!({"role": "assistant", "content": texts(&t.parts)}));
            continue;
        }
        if t.parts.iter().all(|p| matches!(p, Part::Text(_))) {
            out.push(json!({"role": "user", "content": texts(&t.parts)}));
            continue;
        }
        let content: Vec<Value> = t
            .parts
            .iter()
            .map(|part| match part {
                Part::Text(s) => json!({"type": "text", "text": s}),
                Part::Image { mime, data, .. } => {
                    json!({"type": "image_url", "image_url": {"url": data_url(mime, data)}})
                }
                // OpenAI's own API reads a PDF; Mistral and vLLM do not
                // take it in this shape.
                Part::Pdf { name, data } if kind == Kind::Openai => json!({"type": "file",
                    "file": {"filename": name, "file_data": data_url("application/pdf", data)}}),
                Part::Pdf { name, .. } => {
                    json!({"type": "text", "text": unread("A PDF document", name)})
                }
                Part::Media { name, .. } => {
                    json!({"type": "text", "text": unread("A sound or video file", name)})
                }
            })
            .collect();
        out.push(json!({"role": "user", "content": content}));
    }
    Value::Array(out)
}

fn anthropic_messages(p: &ChatPrompt) -> Value {
    let out: Vec<Value> = p
        .turns
        .iter()
        .map(|t| {
            if t.speaker == Speaker::Assistant {
                return json!({"role": "assistant", "content": texts(&t.parts)});
            }
            let content: Vec<Value> = t
                .parts
                .iter()
                .map(|part| match part {
                    Part::Text(s) => json!({"type": "text", "text": s}),
                    Part::Image { mime, data, .. } => json!({"type": "image",
                        "source": {"type": "base64", "media_type": mime, "data": b64(data)}}),
                    Part::Pdf { data, .. } => json!({"type": "document",
                        "source": {"type": "base64", "media_type": "application/pdf", "data": b64(data)}}),
                    Part::Media { name, .. } => {
                        json!({"type": "text", "text": unread("A sound or video file", name)})
                    }
                })
                .collect();
            json!({"role": "user", "content": content})
        })
        .collect();
    Value::Array(out)
}

fn gemini_contents(p: &ChatPrompt) -> Value {
    let out: Vec<Value> = p
        .turns
        .iter()
        .map(|t| {
            let role = match t.speaker {
                Speaker::User => "user",
                Speaker::Assistant => "model",
            };
            let parts: Vec<Value> = t
                .parts
                .iter()
                .map(|part| match part {
                    Part::Text(s) => json!({"text": s}),
                    Part::Image { mime, data, .. } | Part::Media { mime, data, .. } => {
                        json!({"inlineData": {"mimeType": mime, "data": b64(data)}})
                    }
                    Part::Pdf { data, .. } => {
                        json!({"inlineData": {"mimeType": "application/pdf", "data": b64(data)}})
                    }
                })
                .collect();
            json!({"role": role, "parts": parts})
        })
        .collect();
    Value::Array(out)
}

/// The streamed request a conversation's next answer is asked with.
pub fn http_chat(cfg: &Config, model: &str, p: &ChatPrompt) -> Option<afg::HttpCall> {
    let base = cfg.base_url();
    let key = cfg.api_key.trim();
    let protocol = cfg.kind.protocol();
    let mut call = match protocol {
        Protocol::Ollama => afg::HttpCall {
            url: format!("{base}/api/chat"),
            headers: Vec::new(),
            body: json!({"model": model, "stream": true, "messages": ollama_messages(p),
                         "options": {"num_ctx": afg::num_ctx(p.tokens())}}),
        },
        Protocol::OpenAi => {
            let mut body = json!({"model": model, "stream": true,
                                  "messages": openai_messages(cfg.kind, p)});
            if matches!(cfg.kind, Kind::Openai | Kind::Vllm) {
                body["stream_options"] = json!({"include_usage": true});
            }
            afg::HttpCall {
                url: format!("{base}/chat/completions"),
                headers: if key.is_empty() {
                    Vec::new()
                } else {
                    vec![("Authorization".into(), format!("Bearer {key}"))]
                },
                body,
            }
        }
        Protocol::Anthropic => afg::HttpCall {
            url: format!("{base}/v1/messages"),
            headers: vec![
                ("x-api-key".into(), key.to_string()),
                ("anthropic-version".into(), "2023-06-01".into()),
            ],
            body: json!({"model": model, "max_tokens": afg::ANTHROPIC_MAX_TOKENS, "stream": true,
                         "system": p.system_whole(), "messages": anthropic_messages(p)}),
        },
        Protocol::Gemini => afg::HttpCall {
            url: format!(
                "{base}/v1beta/models/{}:streamGenerateContent?alt=sse",
                afg::gemini_model(model)
            ),
            headers: vec![("x-goog-api-key".into(), key.to_string())],
            body: json!({"systemInstruction": {"parts": [{"text": p.system_whole()}]},
                         "contents": gemini_contents(p)}),
        },
        _ => return None,
    };
    afg::apply_effort(protocol, &mut call.body, cfg.effort.trim());
    Some(call)
}

/// An answer as it is read, a piece at a time.
#[derive(Debug, Default)]
struct Reading {
    text: String,
    model: Option<String>,
    input: Option<u64>,
    output: Option<u64>,
    error: Option<String>,
    finish: Option<String>,
    done: bool,
}

impl Reading {
    fn add(&mut self, s: &str, partial: &Partial) {
        if s.is_empty() || self.text.len() > ANSWER_MAX {
            return;
        }
        self.text.push_str(s);
        push(partial, s);
    }

    fn answer(self, model: &str) -> Result<Answer, Failure> {
        if let Some(e) = self.error {
            return Err(Failure::new(e));
        }
        if self.text.trim().is_empty() {
            return Err(Failure::new(format!(
                "an answer with no text ({})",
                self.finish.as_deref().unwrap_or("no text")
            )));
        }
        Ok(Answer {
            text: self.text,
            model: self.model.or_else(|| Some(model.to_string())),
            input_tokens: self.input,
            output_tokens: self.output,
            cost_usd: None,
        })
    }
}

fn str_at<'a>(v: &'a Value, path: &str) -> Option<&'a str> {
    v.pointer(path).and_then(Value::as_str)
}

fn on_ollama(v: &Value, r: &mut Reading, partial: &Partial) {
    if let Some(e) = v.get("error").and_then(Value::as_str) {
        r.error = Some(e.to_string());
        return;
    }
    if let Some(s) = str_at(v, "/message/content") {
        r.add(s, partial);
    }
    if let Some(m) = v.get("model").and_then(Value::as_str) {
        r.model = Some(m.to_string());
    }
    if v.get("done").and_then(Value::as_bool) == Some(true) {
        r.done = true;
        r.input = v.get("prompt_eval_count").and_then(Value::as_u64);
        r.output = v.get("eval_count").and_then(Value::as_u64);
        r.finish = v
            .get("done_reason")
            .and_then(Value::as_str)
            .map(str::to_string);
    }
}

fn on_openai(v: &Value, r: &mut Reading, partial: &Partial) {
    if let Some(e) = afg::error_message(v).filter(|_| v.get("error").is_some()) {
        r.error = Some(e);
        return;
    }
    if let Some(m) = v.get("model").and_then(Value::as_str) {
        r.model = Some(m.to_string());
    }
    if let Some(s) = str_at(v, "/choices/0/delta/content") {
        r.add(s, partial);
    }
    if let Some(f) = str_at(v, "/choices/0/finish_reason") {
        r.finish = Some(f.to_string());
    }
    if let Some(n) = v.pointer("/usage/prompt_tokens").and_then(Value::as_u64) {
        r.input = Some(n);
    }
    if let Some(n) = v
        .pointer("/usage/completion_tokens")
        .and_then(Value::as_u64)
    {
        r.output = Some(n);
    }
}

fn on_anthropic(v: &Value, r: &mut Reading, partial: &Partial) {
    match v.get("type").and_then(Value::as_str).unwrap_or_default() {
        "message_start" => {
            r.model = str_at(v, "/message/model").map(str::to_string);
            let n = |p: &str| v.pointer(p).and_then(Value::as_u64).unwrap_or(0);
            r.input = v
                .pointer("/message/usage/input_tokens")
                .and_then(Value::as_u64)
                .map(|i| {
                    i + n("/message/usage/cache_read_input_tokens")
                        + n("/message/usage/cache_creation_input_tokens")
                });
        }
        "content_block_delta" if str_at(v, "/delta/type") == Some("text_delta") => {
            if let Some(s) = str_at(v, "/delta/text") {
                r.add(s, partial);
            }
        }
        "message_delta" => {
            if let Some(n) = v.pointer("/usage/output_tokens").and_then(Value::as_u64) {
                r.output = Some(n);
            }
            r.finish = str_at(v, "/delta/stop_reason").map(str::to_string);
        }
        "message_stop" => r.done = true,
        "error" => {
            r.error = Some(
                str_at(v, "/error/message")
                    .unwrap_or("the service stopped")
                    .to_string(),
            )
        }
        _ => {}
    }
}

fn on_gemini(v: &Value, r: &mut Reading, partial: &Partial) {
    if v.get("error").is_some() {
        r.error = afg::error_message(v).or_else(|| Some(v["error"].to_string()));
        return;
    }
    if let Some(parts) = v
        .pointer("/candidates/0/content/parts")
        .and_then(Value::as_array)
    {
        for part in parts {
            // A thought summary is the model's, not the answer.
            if part.get("thought").and_then(Value::as_bool) == Some(true) {
                continue;
            }
            if let Some(s) = part.get("text").and_then(Value::as_str) {
                r.add(s, partial);
            }
        }
    }
    if let Some(f) = str_at(v, "/candidates/0/finishReason") {
        r.finish = Some(f.to_string());
    }
    if let Some(m) = v.get("modelVersion").and_then(Value::as_str) {
        r.model = Some(m.to_string());
    }
    if let Some(n) = v
        .pointer("/usageMetadata/promptTokenCount")
        .and_then(Value::as_u64)
    {
        r.input = Some(n);
    }
    if let Some(n) = v
        .pointer("/usageMetadata/candidatesTokenCount")
        .and_then(Value::as_u64)
    {
        r.output = Some(n);
    }
}

/// Read server-sent events: `on` is given each event's name and data, and
/// stops the reading by answering `false`.
fn read_sse(reader: impl BufRead, mut on: impl FnMut(&str, &str) -> bool) -> std::io::Result<()> {
    let mut event = String::new();
    let mut data = String::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.strip_suffix('\r').unwrap_or(&line);
        if line.is_empty() {
            if !data.is_empty() && !on(&event, &data) {
                return Ok(());
            }
            event.clear();
            data.clear();
            continue;
        }
        if let Some(rest) = line.strip_prefix("event:") {
            event = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("data:") {
            if !data.is_empty() {
                data.push('\n');
            }
            data.push_str(rest.strip_prefix(' ').unwrap_or(rest));
        }
    }
    if !data.is_empty() {
        on(&event, &data);
    }
    Ok(())
}

/// Read a streamed answer from a service's response body.
fn read_stream(
    protocol: Protocol,
    body: impl BufRead,
    partial: &Partial,
) -> std::io::Result<Reading> {
    let mut r = Reading::default();
    if protocol == Protocol::Ollama {
        for line in body.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let Ok(v) = serde_json::from_str::<Value>(&line) else {
                continue;
            };
            on_ollama(&v, &mut r, partial);
            if r.done || r.error.is_some() || r.text.len() > ANSWER_MAX {
                break;
            }
        }
        return Ok(r);
    }
    read_sse(body, |_event, data| {
        if data.trim() == "[DONE]" {
            return false;
        }
        let Ok(v) = serde_json::from_str::<Value>(data) else {
            return true;
        };
        match protocol {
            Protocol::OpenAi => on_openai(&v, &mut r, partial),
            Protocol::Anthropic => on_anthropic(&v, &mut r, partial),
            Protocol::Gemini => on_gemini(&v, &mut r, partial),
            _ => {}
        }
        !r.done && r.error.is_none() && r.text.len() <= ANSWER_MAX
    })?;
    Ok(r)
}

fn chat_http(cfg: &Config, p: &ChatPrompt, partial: &Partial) -> Result<Answer, Failure> {
    let agent = afg::http_agent(cfg.timeout());
    let model = match cfg.model.trim() {
        "" if cfg.kind == Kind::Ollama => afg::ollama_model(&agent, &cfg.base_url())?,
        m => m.to_string(),
    };
    let call = http_chat(cfg, &model, p).ok_or_else(|| Failure::new("not a service"))?;
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
    if !(200..300).contains(&status) {
        let body = resp.body_mut().read_to_string().unwrap_or_default();
        return Err(Failure::new(afg::http_error(status, &body)));
    }
    let body = BufReader::new(resp.body_mut().as_reader());
    read_stream(cfg.kind.protocol(), body, partial)
        .map_err(|e| Failure::new(format!("{}: {e}", call.url)))?
        .answer(&model)
}

// ---------------------------------------------------------------------------
// the agents: one command line each
// ---------------------------------------------------------------------------

/// The conversation as an agent reads it: the archive when there is one,
/// then every turn, the member's new message last. Files the agent is given
/// beside it are said to be attached; the others are named as unread.
fn transcript(p: &ChatPrompt, attached: impl Fn(&Part) -> bool) -> String {
    let mut out = String::new();
    if !p.context.is_empty() {
        out.push_str(&p.context);
        out.push_str("\n\n");
    }
    let last = p.turns.len().saturating_sub(1);
    if p.turns.len() > 1 {
        out.push_str("=== The conversation so far ===\n");
    }
    for (i, t) in p.turns.iter().enumerate() {
        out.push_str(match (t.speaker, i == last) {
            (Speaker::User, true) => "\n=== Their new message, which you answer now ===\n",
            (Speaker::User, false) => "\n--- They wrote:\n",
            (Speaker::Assistant, _) => "\n--- You answered:\n",
        });
        for part in &t.parts {
            match part {
                Part::Text(s) => out.push_str(s),
                Part::Image { name, .. } | Part::Pdf { name, .. } | Part::Media { name, .. }
                    if attached(part) =>
                {
                    out.push_str(&format!("[The file \"{name}\" is attached.]"))
                }
                Part::Image { name, .. } => out.push_str(&unread("A picture", name)),
                Part::Pdf { name, .. } => out.push_str(&unread("A PDF document", name)),
                Part::Media { name, .. } => out.push_str(&unread("A sound or video file", name)),
            }
            out.push('\n');
        }
    }
    out
}

/// A safe name for a file written into the agent's working directory.
fn work_name(i: usize, part: &Part) -> String {
    let ext = match part {
        Part::Image { mime, .. } => match mime.as_str() {
            "image/png" => "png",
            "image/gif" => "gif",
            "image/webp" => "webp",
            _ => "jpg",
        },
        Part::Pdf { .. } => "pdf",
        Part::Media { mime, .. } => mime.rsplit('/').next().unwrap_or("bin"),
        Part::Text(_) => "txt",
    };
    format!("file-{i}.{ext}")
}

/// One run of an agent in conversation, before it starts.
#[derive(Debug, Default)]
pub struct ChatCall {
    pub args: Vec<String>,
    pub stdin: Option<Vec<u8>>,
    pub env: Vec<(String, String)>,
    /// Files written into the working directory first.
    pub files: Vec<(String, Vec<u8>)>,
    pub answer_file: Option<String>,
}

/// The file OpenCode is given the conversation in.
const OPENCODE_CONVERSATION: &str = "conversation.txt";

/// `help` is what the agent's own help printed, for the options only some
/// versions know (`afg::knows`).
pub fn chat_cli_call(cfg: &Config, p: &ChatPrompt, work: &Path, help: &str) -> Option<ChatCall> {
    let model = cfg.model.trim();
    let effort = cfg.effort.trim();
    let s = |x: &str| x.to_string();
    let mut call = ChatCall::default();
    let binaries: Vec<&Part> = p
        .turns
        .iter()
        .flat_map(|t| &t.parts)
        .filter(|part| !matches!(part, Part::Text(_)))
        .collect();
    match cfg.kind.protocol() {
        Protocol::ClaudeCode => {
            let partial = afg::knows(help, "--include-partial-messages");
            let mut args: Vec<String> = ["-p", "--output-format", "stream-json", "--verbose"]
                .map(s)
                .to_vec();
            if partial {
                args.push(s("--include-partial-messages"));
            }
            args.extend([s("--tools"), s("")]);
            for option in afg::CLAUDE_KEEP_OUT {
                if afg::knows(help, option[0]) {
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
            let readable = |part: &Part| matches!(part, Part::Image { .. } | Part::Pdf { .. });
            let text = transcript(p, readable);
            if binaries.iter().any(|part| readable(part)) {
                // The pictures and documents go with the words, as content
                // blocks of one message on its input.
                args.extend([s("--input-format"), s("stream-json")]);
                let mut content = vec![json!({"type": "text", "text": text})];
                for part in &binaries {
                    match part {
                        Part::Image { mime, name, data } => {
                            content.push(
                                json!({"type": "text", "text": format!("The file \"{name}\":")}),
                            );
                            content.push(json!({"type": "image",
                                "source": {"type": "base64", "media_type": mime, "data": b64(data)}}));
                        }
                        Part::Pdf { name, data } => {
                            content.push(
                                json!({"type": "text", "text": format!("The file \"{name}\":")}),
                            );
                            content.push(json!({"type": "document",
                                "source": {"type": "base64", "media_type": "application/pdf", "data": b64(data)}}));
                        }
                        _ => {}
                    }
                }
                let line = json!({"type": "user",
                                  "message": {"role": "user", "content": content}});
                call.stdin = Some(format!("{line}\n").into_bytes());
            } else {
                call.stdin = Some(text.into_bytes());
            }
            call.args = args;
        }
        Protocol::Codex => {
            let answer = work.join("answer.md");
            let mut args = vec![s("exec")];
            args.extend(
                afg::CODEX_KEEP_OUT
                    .iter()
                    .filter(|o| afg::knows(help, o))
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
            // Pictures by `--image`, when this version knows it.
            let images = afg::knows(help, "--image");
            let readable = move |part: &Part| images && matches!(part, Part::Image { .. });
            for (i, part) in binaries.iter().enumerate() {
                if readable(part) {
                    let name = work_name(i, part);
                    if let Part::Image { data, .. } = part {
                        call.files.push((name.clone(), data.clone()));
                    }
                    args.extend([s("--image"), work.join(name).to_string_lossy().into_owned()]);
                }
            }
            args.push(s("-"));
            call.stdin = Some(format!("{}\n\n{}", p.system, transcript(p, readable)).into_bytes());
            call.answer_file = Some(s("answer.md"));
            call.args = args;
        }
        Protocol::OpenCode => {
            // Everything in files: the conversation in one, each picture or
            // document in its own, attached after the message.
            let readable = |part: &Part| matches!(part, Part::Image { .. } | Part::Pdf { .. });
            let message = format!(
                "Your instructions and the conversation are in the attached file \
                 {OPENCODE_CONVERSATION}; the files it names as attached are attached too. \
                 Answer their new message, following the instructions."
            );
            let mut args = vec![s("run"), message];
            if !model.is_empty() {
                args.extend([s("--model"), s(model)]);
            }
            if !effort.is_empty() {
                args.extend([s("--variant"), s(effort)]);
            }
            call.files.push((
                s(OPENCODE_CONVERSATION),
                format!("{}\n\n{}", p.system, transcript(p, readable)).into_bytes(),
            ));
            args.extend([
                s("--file"),
                work.join(OPENCODE_CONVERSATION)
                    .to_string_lossy()
                    .into_owned(),
            ]);
            for (i, part) in binaries.iter().enumerate() {
                if readable(part) {
                    let name = work_name(i, part);
                    let data = match part {
                        Part::Image { data, .. } | Part::Pdf { data, .. } => data.clone(),
                        _ => continue,
                    };
                    call.files.push((name.clone(), data));
                    args.push(work.join(name).to_string_lossy().into_owned());
                }
            }
            call.env
                .push((s("OPENCODE_PERMISSION"), s(afg::OPENCODE_DENY)));
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
            call.args = args;
        }
        _ => return None,
    }
    Some(call)
}

/// Claude Code's stream, a line at a time: what it has written so far.
#[derive(Debug, Default)]
struct ClaudeStream {
    /// Whether pieces of the answer came as they were written.
    pieces: bool,
    /// The text of its finished messages.
    said: String,
    /// The `result` line, its last word.
    result: Option<String>,
}

impl ClaudeStream {
    fn line(&mut self, line: &str, partial: &Partial) {
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            return;
        };
        match v.get("type").and_then(Value::as_str).unwrap_or_default() {
            "stream_event" => {
                let e = &v["event"];
                if str_at(e, "/type") == Some("content_block_delta")
                    && str_at(e, "/delta/type") == Some("text_delta")
                {
                    if let Some(s) = str_at(e, "/delta/text") {
                        self.pieces = true;
                        push(partial, s);
                    }
                }
            }
            "assistant" if !self.pieces => {
                let text: String = v
                    .pointer("/message/content")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter(|b| b.get("type").and_then(Value::as_str) == Some("text"))
                    .filter_map(|b| b.get("text").and_then(Value::as_str))
                    .collect();
                if !text.is_empty() {
                    if !self.said.is_empty() {
                        self.said.push_str("\n\n");
                    }
                    self.said.push_str(&text);
                    set(partial, &self.said);
                }
            }
            "result" => self.result = Some(line.to_string()),
            _ => {}
        }
    }
}

/// Working directories older than this, left by a process stopped while an
/// agent answered, are removed when the next one starts.
const STALE_WORK: std::time::Duration = std::time::Duration::from_secs(6 * 3600);

async fn chat_cli(
    cfg: &Config,
    p: &ChatPrompt,
    base: &Path,
    partial: &Partial,
) -> Result<Answer, Failure> {
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
    let protocol = cfg.kind.protocol();
    let program = afg::find_program(cfg.kind, &cfg.program).ok_or_else(|| {
        Failure::new(format!(
            "{}: not found on this machine",
            match cfg.program.trim() {
                "" => cfg.kind.program(),
                p => p,
            }
        ))
    })?;
    // Several conversations may be answered at once: only directories old
    // enough to be a stopped process's are taken away.
    if let Ok(entries) = std::fs::read_dir(base) {
        for e in entries.flatten() {
            let name = e.file_name();
            let ours = name
                .to_str()
                .and_then(|n| n.strip_prefix(".afg-chat-"))
                .is_some_and(|id| uuid::Uuid::parse_str(id).is_ok());
            let old = e
                .metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.elapsed().ok())
                .is_some_and(|age| age > STALE_WORK);
            if ours && old {
                let _ = std::fs::remove_dir_all(e.path());
            }
        }
    }
    let work = base.join(format!(".afg-chat-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&work).map_err(|e| Failure::new(format!("{}: {e}", work.display())))?;
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&work, std::fs::Permissions::from_mode(0o700));
    }
    let outcome = async {
        let help = match protocol {
            Protocol::ClaudeCode => afg::help_of(&program, &["--help"], &work).await,
            Protocol::Codex => afg::help_of(&program, &["exec", "--help"], &work).await,
            _ => String::new(),
        };
        let call =
            chat_cli_call(cfg, p, &work, &help).ok_or_else(|| Failure::new("not an agent"))?;
        for (name, content) in &call.files {
            std::fs::write(work.join(name), content)
                .map_err(|e| Failure::new(format!("{name}: {e}")))?;
        }
        let mut cmd = afg::agent_command(&program, &work);
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
        let writer = match (call.stdin, child.stdin.take()) {
            (Some(input), Some(mut pipe)) => Some(tokio::spawn(async move {
                let _ = pipe.write_all(&input).await;
                let _ = pipe.shutdown().await;
            })),
            _ => None,
        };
        // Its error output is read to its end, so that a full pipe never
        // stops it, and its beginning kept.
        let stderr = child.stderr.take();
        let errors = tokio::spawn(async move {
            let mut kept = Vec::new();
            if let Some(mut e) = stderr {
                let mut buf = [0u8; 8192];
                while let Ok(n) = e.read(&mut buf).await {
                    if n == 0 {
                        break;
                    }
                    if kept.len() < 256 * 1024 {
                        kept.extend_from_slice(&buf[..n]);
                    }
                }
            }
            String::from_utf8_lossy(&kept).into_owned()
        });
        let stdout = child.stdout.take();
        let reading = async {
            let mut all = String::new();
            let mut claude = ClaudeStream::default();
            if let Some(out) = stdout {
                // A line at a time, whatever its bytes, to its end.
                let mut reader = tokio::io::BufReader::new(out);
                let mut buf = Vec::new();
                loop {
                    buf.clear();
                    match reader.read_until(b'\n', &mut buf).await {
                        Ok(0) | Err(_) => break,
                        Ok(_) => {}
                    }
                    let line = String::from_utf8_lossy(&buf);
                    let line = line.trim_end_matches(['\n', '\r']);
                    if protocol == Protocol::ClaudeCode {
                        claude.line(line, partial);
                    }
                    if all.len() < ANSWER_MAX {
                        all.push_str(line);
                        all.push('\n');
                    }
                }
            }
            let status = child.wait().await;
            (status, all, claude)
        };
        let waited = tokio::time::timeout(cfg.timeout(), reading).await;
        if let Some(w) = writer {
            w.abort();
        }
        let (status, stdout, claude) = match waited {
            Err(_) => {
                errors.abort();
                return Err(Failure::new(format!(
                    "{}: no answer after {} s",
                    cfg.kind.program(),
                    cfg.timeout().as_secs()
                )));
            }
            Ok(r) => r,
        };
        let stderr = errors.await.unwrap_or_default();
        let status = status.map_err(|e| Failure::new(format!("{}: {e}", program.display())))?;
        let answer_file = call
            .answer_file
            .as_ref()
            .and_then(|f| std::fs::read_to_string(work.join(f)).ok())
            .filter(|t| !t.trim().is_empty());
        let parsed = match protocol {
            Protocol::ClaudeCode => afg::parse_cli(
                protocol,
                claude.result.as_deref().unwrap_or(stdout.trim()),
                None,
            ),
            _ => afg::parse_cli(protocol, &stdout, answer_file),
        };
        if let Ok(a) = &parsed {
            set(partial, &a.text);
        }
        match (status.success(), parsed) {
            (true, Ok(a)) => Ok(a),
            (true, Err(m)) => Err(Failure {
                message: m,
                detail: Some(afg::tail(&stderr, 2000)).filter(|d| !d.is_empty()),
            }),
            (false, parsed) => Err(Failure {
                message: match parsed {
                    Err(m)
                        if protocol == Protocol::ClaudeCode && !m.starts_with("not the JSON") =>
                    {
                        m
                    }
                    _ => format!("{} exited with {status}", cfg.kind.program()),
                },
                detail: Some(afg::tail(&format!("{stderr}\n{stdout}"), 2000))
                    .filter(|d| !d.is_empty()),
            }),
        }
    }
    .await;
    let _ = std::fs::remove_dir_all(&work);
    outcome
}

/// Ask the AFG for a conversation's next answer, its pieces added to
/// `partial` as they come.
pub async fn chat(
    cfg: Config,
    p: ChatPrompt,
    base: std::path::PathBuf,
    partial: Partial,
) -> Result<Answer, Failure> {
    match cfg.kind.protocol() {
        Protocol::None => Err(Failure::new("the AFG is off")),
        Protocol::Ollama | Protocol::OpenAi | Protocol::Anthropic | Protocol::Gemini => {
            tokio::task::spawn_blocking(move || chat_http(&cfg, &p, &partial))
                .await
                .map_err(|e| Failure::new(e.to_string()))?
        }
        Protocol::ClaudeCode | Protocol::Codex | Protocol::OpenCode => {
            chat_cli(&cfg, &p, &base, &partial).await
        }
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

    fn prompt() -> ChatPrompt {
        ChatPrompt {
            system: "SYS".into(),
            context: String::new(),
            turns: vec![
                Turn {
                    speaker: Speaker::User,
                    parts: vec![Part::Text("Bonjour".into())],
                },
                Turn {
                    speaker: Speaker::Assistant,
                    parts: vec![Part::Text("Bonjour !".into())],
                },
                Turn {
                    speaker: Speaker::User,
                    parts: vec![
                        Part::Image {
                            mime: "image/png".into(),
                            name: "mamie.png".into(),
                            data: vec![1, 2, 3],
                        },
                        Part::Pdf {
                            name: "acte.pdf".into(),
                            data: vec![4, 5],
                        },
                        Part::Media {
                            mime: "audio/mpeg".into(),
                            name: "voix.mp3".into(),
                            data: vec![6],
                        },
                        Part::Text("Qui est-ce ?".into()),
                    ],
                },
            ],
        }
    }

    fn partial() -> Partial {
        Partial::default()
    }

    fn text(p: &Partial) -> String {
        p.lock().unwrap().clone()
    }

    #[test]
    fn each_service_is_asked_to_stream_the_whole_conversation_with_its_files() {
        let p = prompt();
        let call = http_chat(&cfg(Kind::Ollama), "llava", &p).unwrap();
        assert_eq!(call.url, "http://127.0.0.1:11434/api/chat");
        assert_eq!(call.body["stream"], true);
        let m = &call.body["messages"];
        assert_eq!(m[0], json!({"role": "system", "content": "SYS"}));
        assert_eq!(m[2]["role"], "assistant");
        assert_eq!(m[3]["images"], json!(["AQID"]));
        let words = m[3]["content"].as_str().unwrap();
        assert!(words.contains("\"acte.pdf\" was sent here") && words.contains("Qui est-ce ?"));

        let call = http_chat(&cfg(Kind::Openai), "gpt", &p).unwrap();
        assert_eq!(call.url, "https://api.openai.com/v1/chat/completions");
        assert_eq!(call.body["stream_options"], json!({"include_usage": true}));
        let user = &call.body["messages"][3]["content"];
        assert_eq!(user[0]["image_url"]["url"], "data:image/png;base64,AQID");
        assert_eq!(user[1]["type"], "file", "OpenAI reads a PDF");
        assert_eq!(
            call.body["messages"][1]["content"], "Bonjour",
            "words alone stay words"
        );
        let call = http_chat(&cfg(Kind::Mistral), "m", &p).unwrap();
        assert_eq!(
            call.body["messages"][3]["content"][1]["type"], "text",
            "Mistral is told"
        );
        assert!(call.body.get("stream_options").is_none());

        let mut c = cfg(Kind::Anthropic);
        c.effort = "low".into();
        let call = http_chat(&c, "claude-opus-5-5", &p).unwrap();
        assert_eq!(call.body["system"], "SYS");
        assert_eq!(call.body["output_config"], json!({"effort": "low"}));
        let blocks = &call.body["messages"][2]["content"];
        assert_eq!(blocks[0]["source"]["media_type"], "image/png");
        assert_eq!(blocks[1]["type"], "document");
        assert_eq!(blocks[2]["type"], "text", "sound is named");

        let mut c = cfg(Kind::Gemini);
        c.effort = "high".into();
        let mut with_archive = p.clone();
        with_archive.context = "THE ARCHIVE".into();
        let call = http_chat(&c, "models/gemini-4-argon", &with_archive).unwrap();
        assert_eq!(
            call.url,
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-4-argon:streamGenerateContent?alt=sse"
        );
        assert_eq!(
            call.body["systemInstruction"]["parts"][0]["text"],
            "SYS\n\nTHE ARCHIVE"
        );
        let parts = &call.body["contents"][2]["parts"];
        assert_eq!(
            parts[2]["inlineData"]["mimeType"], "audio/mpeg",
            "Gemini hears it"
        );
        assert_eq!(call.body["contents"][1]["role"], "model");
        assert_eq!(
            call.body["generationConfig"]["thinkingConfig"]["thinkingLevel"],
            "high"
        );
    }

    #[test]
    fn each_stream_is_read_as_it_comes() {
        let p = partial();
        let ollama = "{\"model\":\"q\",\"message\":{\"content\":\"Bon\"},\"done\":false}\n\
                      {\"model\":\"q\",\"message\":{\"content\":\"jour\"},\"done\":false}\n\
                      {\"model\":\"q\",\"message\":{\"content\":\"\"},\"done\":true,\"prompt_eval_count\":12,\"eval_count\":3}\n";
        let r = read_stream(Protocol::Ollama, ollama.as_bytes(), &p).unwrap();
        assert_eq!(
            (r.text.as_str(), r.input, r.output),
            ("Bonjour", Some(12), Some(3))
        );
        assert_eq!(text(&p), "Bonjour");

        let p = partial();
        let openai = "data: {\"model\":\"gpt\",\"choices\":[{\"delta\":{\"role\":\"assistant\"}}]}\n\n\
                      data: {\"choices\":[{\"delta\":{\"content\":\"Un\"}}]}\n\n\
                      data: {\"choices\":[{\"delta\":{\"content\":\" deux\"},\"finish_reason\":\"stop\"}]}\n\n\
                      data: {\"choices\":[],\"usage\":{\"prompt_tokens\":5,\"completion_tokens\":2}}\n\n\
                      data: [DONE]\n\n";
        let r = read_stream(Protocol::OpenAi, openai.as_bytes(), &p).unwrap();
        assert_eq!((r.text.as_str(), r.output), ("Un deux", Some(2)));
        assert_eq!(r.answer("x").unwrap().model.as_deref(), Some("gpt"));

        let p = partial();
        let anthropic = "event: message_start\ndata: {\"type\":\"message_start\",\"message\":{\"model\":\"claude-opus-5-5\",\"usage\":{\"input_tokens\":7,\"cache_read_input_tokens\":3}}}\n\n\
                         event: content_block_start\ndata: {\"type\":\"content_block_start\",\"index\":0,\"content_block\":{\"type\":\"thinking\"}}\n\n\
                         event: content_block_delta\ndata: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"thinking_delta\",\"thinking\":\"hmm\"}}\n\n\
                         event: content_block_delta\ndata: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"text_delta\",\"text\":\"Trois\"}}\n\n\
                         event: message_delta\ndata: {\"type\":\"message_delta\",\"delta\":{\"stop_reason\":\"end_turn\"},\"usage\":{\"output_tokens\":4}}\n\n\
                         event: message_stop\ndata: {\"type\":\"message_stop\"}\n\n";
        let r = read_stream(Protocol::Anthropic, anthropic.as_bytes(), &p).unwrap();
        assert_eq!(
            (r.text.as_str(), r.input, r.output),
            ("Trois", Some(10), Some(4)),
            "its thinking is not its answer"
        );

        let p = partial();
        let gemini = "data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"pensée\",\"thought\":true},{\"text\":\"Qua\"}]}}]}\r\n\r\n\
                      data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"tre\"}]},\"finishReason\":\"STOP\"}],\"usageMetadata\":{\"promptTokenCount\":9,\"candidatesTokenCount\":2},\"modelVersion\":\"gemini-4-argon\"}\r\n\r\n";
        let r = read_stream(Protocol::Gemini, gemini.as_bytes(), &p).unwrap();
        assert_eq!((r.text.as_str(), r.input), ("Quatre", Some(9)));
        assert_eq!(text(&p), "Quatre");

        // A refusal mid-stream is said in the service's words.
        let p = partial();
        let err = "event: error\ndata: {\"type\":\"error\",\"error\":{\"type\":\"overloaded_error\",\"message\":\"Overloaded\"}}\n\n";
        let r = read_stream(Protocol::Anthropic, err.as_bytes(), &p).unwrap();
        assert_eq!(r.answer("x").unwrap_err().message, "Overloaded");
        let r = read_stream(
            Protocol::Ollama,
            "{\"error\":\"model not found\"}\n".as_bytes(),
            &partial(),
        )
        .unwrap();
        assert_eq!(r.answer("x").unwrap_err().message, "model not found");
    }

    const CLAUDE_HELP: &str = "  --include-partial-messages  Include\n  --input-format <format>\n  \
        --no-session-persistence\n  --output-format <format>\n  --safe-mode\n  --system-prompt <prompt>\n  \
        --tools <tools...>\n  --verbose";

    #[test]
    fn each_agent_is_given_the_conversation_and_what_it_can_read_of_its_files() {
        let p = prompt();
        let work = Path::new("/w");
        let mut c = cfg(Kind::ClaudeCode);
        c.model = "claude-opus-5-5".into();
        let call = chat_cli_call(&c, &p, work, CLAUDE_HELP).unwrap();
        let a = call.args.join(" ");
        assert!(
            a.starts_with(
                "-p --output-format stream-json --verbose --include-partial-messages --tools "
            ),
            "{a}"
        );
        assert!(
            call.args
                .windows(2)
                .any(|w| w == ["--input-format", "stream-json"]),
            "the picture goes with it"
        );
        assert!(call
            .args
            .windows(2)
            .any(|w| w == ["--system-prompt", "SYS"]));
        let line: Value = serde_json::from_slice(call.stdin.as_ref().unwrap()).unwrap();
        let content = &line["message"]["content"];
        let words = content[0]["text"].as_str().unwrap();
        assert!(
            words.contains("--- They wrote:\nBonjour")
                && words.contains("--- You answered:\nBonjour !")
        );
        assert!(words.contains("[The file \"mamie.png\" is attached.]"));
        assert!(words.contains("\"voix.mp3\" was sent here"), "{words}");
        assert!(words.contains("=== Their new message, which you answer now ===\n"));
        assert_eq!(content[2]["type"], "image");
        assert_eq!(content[4]["type"], "document");
        // Words alone go as words, and an older version is not asked for
        // pieces it cannot give.
        let mut words_only = prompt();
        words_only.turns.truncate(2);
        words_only.turns.push(Turn {
            speaker: Speaker::User,
            parts: vec![Part::Text("Et toi ?".into())],
        });
        let call = chat_cli_call(&c, &words_only, work, "  --verbose").unwrap();
        assert!(!call
            .args
            .iter()
            .any(|a| a == "--include-partial-messages" || a == "--input-format"));
        assert!(String::from_utf8(call.stdin.unwrap())
            .unwrap()
            .ends_with("Et toi ?\n"));

        let call = chat_cli_call(&cfg(Kind::Codex), &p, work, "  --image <FILE>  Attach").unwrap();
        assert!(call
            .args
            .windows(2)
            .any(|w| w == ["--image", "/w/file-0.png"]));
        assert_eq!(call.files[0].0, "file-0.png");
        assert_eq!(call.args.last().map(String::as_str), Some("-"));
        let stdin = String::from_utf8(call.stdin.unwrap()).unwrap();
        assert!(stdin.starts_with("SYS\n\n") && stdin.contains("\"acte.pdf\" was sent here"));

        let mut with_archive = prompt();
        with_archive.context = "THE ARCHIVE".into();
        let call = chat_cli_call(&cfg(Kind::Opencode), &with_archive, work, "").unwrap();
        assert_eq!(call.args[0], "run");
        assert!(
            !call.args[1].contains("THE ARCHIVE"),
            "never on the command line"
        );
        let files: Vec<&str> = call.files.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(files, ["conversation.txt", "file-0.png", "file-1.pdf"]);
        let conversation = String::from_utf8(call.files[0].1.clone()).unwrap();
        assert!(conversation.starts_with("SYS\n\nTHE ARCHIVE"));
        assert!(call.args.iter().any(|a| a == "/w/file-1.pdf"));
    }

    #[test]
    fn claude_codes_stream_is_read_a_piece_at_a_time() {
        let p = partial();
        let mut s = ClaudeStream::default();
        for line in [
            r#"{"type":"system","subtype":"init"}"#,
            r#"{"type":"stream_event","event":{"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Les "}}}"#,
            r#"{"type":"stream_event","event":{"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Zając."}}}"#,
            r#"{"type":"assistant","message":{"content":[{"type":"text","text":"Les Zając."}]}}"#,
            r#"{"type":"result","subtype":"success","is_error":false,"result":"Les Zając.","total_cost_usd":0.01}"#,
        ] {
            s.line(line, &p);
        }
        assert_eq!(text(&p), "Les Zając.", "the pieces, not twice");
        let a = afg::parse_cli(Protocol::ClaudeCode, s.result.as_deref().unwrap(), None).unwrap();
        assert_eq!((a.text.as_str(), a.cost_usd), ("Les Zając.", Some(0.01)));
        // Without its pieces, each finished message is shown whole.
        let p = partial();
        let mut s = ClaudeStream::default();
        s.line(
            r#"{"type":"assistant","message":{"content":[{"type":"text","text":"Un."}]}}"#,
            &p,
        );
        assert_eq!(text(&p), "Un.");
    }
}
