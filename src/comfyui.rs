//! ComfyUI as the AFG's picture service (docs/adr/0030): a photograph
//! restored by the workflow the administrator gave, on a machine of the
//! family's own.
//!
//! The workflow is ComfyUI's API format ("Save (API Format)"): an object of
//! nodes, each with its `class_type` and its `inputs`. The photograph is
//! uploaded (`/upload/image`) and named in every `LoadImage` node; any text
//! input holding `{{restore}}` is given the AFG's restoration instruction,
//! for a workflow whose model takes one. The workflow is queued
//! (`/prompt`), followed (`/history/<id>`), and the first picture it saved
//! is fetched (`/view`).

use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::afg::{self, Failure, ImageConfig};

/// The place in a workflow's text that is given the instruction.
pub const PLACEHOLDER: &str = "{{restore}}";

/// The most a picture or an answer may weigh.
const ANSWER_MAX: u64 = 64 * 1024 * 1024;

fn fail(what: impl Into<String>) -> Failure {
    Failure::new(format!("ComfyUI: {}", what.into()))
}

/// The workflow with the photograph and the instruction in it.
pub fn prepared(workflow: &str, image: &str, prompt: &str) -> Result<Value, Failure> {
    let mut wf: Value = serde_json::from_str(workflow)
        .map_err(|e| fail(format!("the workflow is not JSON: {e}")))?;
    let nodes = wf
        .as_object_mut()
        .ok_or_else(|| fail("the workflow is not in the API format"))?;
    let mut loads = 0;
    for node in nodes.values_mut() {
        let load = node.get("class_type").and_then(Value::as_str) == Some("LoadImage");
        let Some(inputs) = node.get_mut("inputs").and_then(Value::as_object_mut) else {
            continue;
        };
        if load {
            inputs.insert("image".into(), Value::from(image));
            loads += 1;
        }
        for v in inputs.values_mut() {
            if let Some(s) = v.as_str().filter(|s| s.contains(PLACEHOLDER)) {
                *v = Value::from(s.replace(PLACEHOLDER, prompt));
            }
        }
    }
    if loads == 0 {
        return Err(fail("the workflow has no LoadImage node"));
    }
    Ok(wf)
}

/// The first picture a finished prompt saved, as `/view` names it.
pub fn first_picture(entry: &Value) -> Option<(String, String, String)> {
    let outputs = entry.get("outputs")?.as_object()?;
    let mut keys: Vec<&String> = outputs.keys().collect();
    // Nodes in their numeric order, as the workflow numbered them.
    keys.sort_by_key(|k| (k.parse::<u64>().unwrap_or(u64::MAX), (*k).clone()));
    let mut fallback = None;
    for k in keys {
        for img in outputs[k]
            .get("images")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let name = img.get("filename").and_then(Value::as_str)?.to_string();
            let sub = img
                .get("subfolder")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            let kind = img
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or("output")
                .to_string();
            if kind == "output" {
                return Some((name, sub, kind));
            }
            fallback.get_or_insert((name, sub, kind));
        }
    }
    fallback
}

fn read(resp: &mut ureq::http::Response<ureq::Body>) -> Result<(u16, String), Failure> {
    let status = resp.status().as_u16();
    let text = resp
        .body_mut()
        .with_config()
        .limit(ANSWER_MAX)
        .read_to_string()
        .map_err(|e| fail(format!("reading the answer: {e}")))?;
    Ok((status, text))
}

/// Restore a photograph with the workflow.
pub async fn restore(
    cfg: ImageConfig,
    timeout: Duration,
    prompt: &'static str,
    mime: String,
    bytes: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
    tokio::task::spawn_blocking(move || {
        let deadline = Instant::now() + timeout;
        let agent = afg::http_agent(timeout);
        let base = cfg.base_url();
        let q = crate::view::query_value;

        // The photograph, into ComfyUI's input folder.
        let ext = match mime.as_str() {
            "image/png" => "png",
            "image/webp" => "webp",
            _ => "jpg",
        };
        let name = format!("axgf-{}.{ext}", uuid::Uuid::new_v4().simple());
        let boundary = format!("axgf{}", uuid::Uuid::new_v4().simple());
        let mut body = Vec::new();
        body.extend_from_slice(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"image\"; \
                 filename=\"{name}\"\r\nContent-Type: {mime}\r\n\r\n"
            )
            .as_bytes(),
        );
        body.extend_from_slice(&bytes);
        body.extend_from_slice(
            format!(
                "\r\n--{boundary}\r\nContent-Disposition: form-data; name=\"overwrite\"\r\n\r\ntrue\
                 \r\n--{boundary}--\r\n"
            )
            .as_bytes(),
        );
        let url = format!("{base}/upload/image");
        let mut resp = agent
            .post(&url)
            .header(
                "Content-Type",
                format!("multipart/form-data; boundary={boundary}"),
            )
            .send(&body[..])
            .map_err(|e| fail(format!("{url}: {e}")))?;
        let (status, text) = read(&mut resp)?;
        if !(200..300).contains(&status) {
            return Err(fail(afg::http_error(status, &text)));
        }
        let up: Value = serde_json::from_str(&text)
            .map_err(|e| fail(format!("the upload's answer is not JSON: {e}")))?;
        let stored = up.get("name").and_then(Value::as_str).unwrap_or(&name);
        let image = match up.get("subfolder").and_then(Value::as_str) {
            Some(sub) if !sub.is_empty() => format!("{sub}/{stored}"),
            _ => stored.to_string(),
        };

        // Queued.
        let workflow = prepared(&cfg.workflow, &image, prompt)?;
        let url = format!("{base}/prompt");
        let mut resp = agent
            .post(&url)
            .header("Content-Type", "application/json")
            .send(
                json!({"prompt": workflow, "client_id": uuid::Uuid::new_v4().to_string()})
                    .to_string(),
            )
            .map_err(|e| fail(format!("{url}: {e}")))?;
        let (status, text) = read(&mut resp)?;
        let queued: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
        if !(200..300).contains(&status) || queued.get("error").is_some() {
            let said = queued
                .pointer("/error/message")
                .and_then(Value::as_str)
                .map(str::to_string)
                .unwrap_or_else(|| afg::tail(&text, 300));
            return Err(fail(format!("the workflow was refused: {said}")));
        }
        let id = queued
            .get("prompt_id")
            .and_then(Value::as_str)
            .ok_or_else(|| fail("the queue gave no prompt_id"))?
            .to_string();

        // Followed until it is done.
        let entry = loop {
            if Instant::now() >= deadline {
                return Err(fail("no picture in the time allowed"));
            }
            let url = format!("{base}/history/{}", q(&id));
            let mut resp = agent
                .get(&url)
                .call()
                .map_err(|e| fail(format!("{url}: {e}")))?;
            let (status, text) = read(&mut resp)?;
            if (200..300).contains(&status) {
                let history: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
                if let Some(entry) = history.get(&id) {
                    let state = entry
                        .pointer("/status/status_str")
                        .and_then(Value::as_str)
                        .unwrap_or("");
                    if state == "error" {
                        return Err(fail(format!(
                            "the workflow failed: {}",
                            afg::tail(
                                &entry
                                    .pointer("/status/messages")
                                    .cloned()
                                    .unwrap_or_default()
                                    .to_string(),
                                300
                            )
                        )));
                    }
                    let done = entry
                        .pointer("/status/completed")
                        .and_then(Value::as_bool)
                        .unwrap_or(false);
                    if done || state == "success" {
                        break entry.clone();
                    }
                }
            }
            std::thread::sleep(Duration::from_secs(2));
        };

        // The picture it saved.
        let (filename, subfolder, kind) =
            first_picture(&entry).ok_or_else(|| fail("the workflow saved no picture"))?;
        let url = format!(
            "{base}/view?filename={}&subfolder={}&type={}",
            q(&filename),
            q(&subfolder),
            q(&kind)
        );
        let mut resp = agent
            .get(&url)
            .call()
            .map_err(|e| fail(format!("{url}: {e}")))?;
        let status = resp.status().as_u16();
        let picture = resp
            .body_mut()
            .with_config()
            .limit(ANSWER_MAX)
            .read_to_vec()
            .map_err(|e| fail(format!("{url}: {e}")))?;
        if !(200..300).contains(&status) {
            return Err(fail(format!("{url}: HTTP {status}")));
        }
        Ok(picture)
    })
    .await
    .map_err(|e| fail(e.to_string()))?
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workflow() -> String {
        json!({
            "3": {"class_type": "LoadImage", "inputs": {"image": "example.png"}},
            "6": {"class_type": "CLIPTextEncode", "inputs": {"text": "Please: {{restore}}"}},
            "9": {"class_type": "SaveImage", "inputs": {"images": ["8", 0]}},
        })
        .to_string()
    }

    #[test]
    fn the_photograph_and_the_instruction_go_into_the_workflow() {
        let wf = prepared(&workflow(), "axgf-1.jpg", "Restore it.").unwrap();
        assert_eq!(wf["3"]["inputs"]["image"], "axgf-1.jpg");
        assert_eq!(wf["6"]["inputs"]["text"], "Please: Restore it.");
        assert!(prepared(
            "{\"1\": {\"class_type\": \"SaveImage\", \"inputs\": {}}}",
            "a",
            "b"
        )
        .is_err());
        assert!(prepared("not json", "a", "b").is_err());
    }

    #[test]
    fn the_first_picture_saved_is_the_one_brought_back() {
        let entry = json!({"outputs": {
            "12": {"images": [{"filename": "late.png", "subfolder": "", "type": "output"}]},
            "9": {"images": [{"filename": "preview.png", "subfolder": "", "type": "temp"},
                             {"filename": "axgf_00001_.png", "subfolder": "fam", "type": "output"}]},
        }});
        assert_eq!(
            first_picture(&entry),
            Some(("axgf_00001_.png".into(), "fam".into(), "output".into()))
        );
        let only_temp =
            json!({"outputs": {"4": {"images": [{"filename": "t.png", "type": "temp"}]}}});
        assert_eq!(first_picture(&only_temp).map(|p| p.2), Some("temp".into()));
        assert_eq!(first_picture(&json!({"outputs": {}})), None);
    }
}
