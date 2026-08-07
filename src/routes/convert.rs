//! The GEDCOM conversion page.

use axum::extract::{Multipart, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use minijinja::context;
use serde_json::{json, Value};

use crate::convert::{axgf_name_for, counts_of, looks_like_gedcom, MAX_UPLOAD};
use crate::routes::Shared;
use crate::state::envelope_into_data;
use crate::{auth, render};

/// `GET /convert` — the upload form.
pub async fn form(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let is_admin = auth::is_admin(&headers, state.admin_token());
    render::page(
        "convert.html",
        context! {
            nav => "convert",
            is_admin,
            max_mb => MAX_UPLOAD / (1024 * 1024),
        },
    )
}

/// Fields pulled out of the upload form.
#[derive(Default)]
struct Upload {
    filename: String,
    bytes: Vec<u8>,
    confidence: f64,
    lang: String,
}

/// `POST /convert/gedcom` — convert, report, offer the download.
pub async fn gedcom(
    State(state): State<Shared>,
    headers: HeaderMap,
    multipart: Multipart,
) -> Response {
    let is_admin = auth::is_admin(&headers, state.admin_token());

    let upload = match read_upload(multipart).await {
        Ok(u) => u,
        Err((status, msg)) => return fail(is_admin, status, &msg),
    };

    if upload.bytes.is_empty() {
        return fail(
            is_admin,
            StatusCode::OK,
            "No file was uploaded. Choose a .ged file first.",
        );
    }
    if upload.bytes.len() > MAX_UPLOAD {
        return fail(
            is_admin,
            StatusCode::PAYLOAD_TOO_LARGE,
            &format!(
                "That file is {:.1} MB. The limit is {} MB. Nothing was converted.",
                upload.bytes.len() as f64 / (1024.0 * 1024.0),
                MAX_UPLOAD / (1024 * 1024)
            ),
        );
    }
    if !looks_like_gedcom(&upload.bytes) {
        return fail(
            is_admin,
            StatusCode::OK,
            "That does not look like a GEDCOM file. A GEDCOM 5.5.1 file starts \
             with a `0 HEAD` line. Nothing was converted.",
        );
    }

    // The library owns the conversion. This crate only reports the result.
    let env = axgf_rs::convert_gedcom(&upload.bytes, upload.confidence, &upload.lang);
    let diagnostics = render_diagnostics(&env.diagnostics);

    let data = match envelope_into_data(env) {
        Ok(d) => d,
        Err(_) => {
            return render::page(
                "convert_result.html",
                context! {
                    nav => "convert",
                    is_admin,
                    ok => false,
                    error => "The converter refused this file. The diagnostics below \
                              say why. The served bundle was not touched.",
                    diagnostics,
                    filename => upload.filename,
                },
            );
        }
    };

    let bundle = data.get("bundle").cloned().unwrap_or(Value::Null);
    let counts = counts_of(&bundle);
    let total: usize = counts.iter().map(|(_, n)| n).sum();

    // Export to the bytes the visitor will download. The served bundle is
    // untouched: this never goes near AppState::mutate.
    let bytes = match crate::state::export_to_bytes(&bundle.to_string()) {
        Ok(b) => b,
        Err(e) => {
            return fail(
                is_admin,
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("The bundle converted but could not be packaged: {e}"),
            )
        }
    };
    let size = bytes.len();

    let download_name = axgf_name_for(&upload.filename);
    let id = state.conversions().put(download_name.clone(), bytes);

    // Unrecognised tags are the interesting part of this page, not noise:
    // they are the proof that nothing was silently dropped.
    let skipped: Vec<Value> = diagnostics
        .iter()
        .filter(|d| d["code"] == "GEDCOM_UNRECOGNIZED_TAG")
        .cloned()
        .collect();
    let others: Vec<Value> = diagnostics
        .iter()
        .filter(|d| d["code"] != "GEDCOM_UNRECOGNIZED_TAG")
        .cloned()
        .collect();

    render::page(
        "convert_result.html",
        context! {
            nav => "convert",
            is_admin,
            ok => true,
            filename => upload.filename,
            download_id => id,
            download_name,
            size_kb => size / 1024,
            total,
            counts => counts.iter().map(|(k, n)| json!({"kind": k, "n": n}))
                            .collect::<Vec<_>>(),
            skipped,
            others,
            confidence => upload.confidence,
            lang => upload.lang,
        },
    )
}

/// `GET /convert/download/:id` — the converted bundle.
pub async fn download(State(state): State<Shared>, Path(id): Path<String>) -> Response {
    match state.conversions().get(&id) {
        Some((name, bytes)) => render::bundle_download(&name, bytes),
        None => render::error_page(
            StatusCode::NOT_FOUND,
            "That download has expired",
            "Converted bundles are held for fifteen minutes. Convert the file again.",
        ),
    }
}

/// Read the multipart form, tolerating fields in any order.
///
/// The error carries a status because the two failure modes deserve different
/// ones: a body past the limit is a 413, anything else malformed is a 400.
async fn read_upload(mut multipart: Multipart) -> Result<Upload, (StatusCode, String)> {
    let mut out = Upload {
        confidence: 0.8,
        lang: "en".to_string(),
        ..Default::default()
    };

    let too_large = || {
        (
            StatusCode::PAYLOAD_TOO_LARGE,
            format!(
                "That upload is larger than the {} MB limit. Nothing was converted.",
                MAX_UPLOAD / (1024 * 1024)
            ),
        )
    };

    loop {
        let field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            // A truncated or malformed body is a user error, not a panic. The
            // body-limit layer surfaces here too, so it is separated out.
            Err(e) if e.status() == StatusCode::PAYLOAD_TOO_LARGE => return Err(too_large()),
            Err(e) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("The upload could not be read: {e}"),
                ))
            }
        };

        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "file" => {
                out.filename = field.file_name().unwrap_or("upload.ged").to_string();
                match field.bytes().await {
                    Ok(b) => out.bytes = b.to_vec(),
                    // In practice this is the body limit firing mid-field.
                    Err(_) => return Err(too_large()),
                }
            }
            "confidence" => {
                if let Ok(t) = field.text().await {
                    if let Ok(v) = t.trim().parse::<f64>() {
                        out.confidence = v.clamp(0.0, 1.0);
                    }
                }
            }
            "lang" => {
                if let Ok(t) = field.text().await {
                    let t = t.trim();
                    if !t.is_empty() {
                        out.lang = t.to_string();
                    }
                }
            }
            _ => {
                let _ = field.bytes().await;
            }
        }
    }
    Ok(out)
}

/// Diagnostics as plain JSON for the template.
fn render_diagnostics(diags: &[axgf_rs::boundary::envelope::Diagnostic]) -> Vec<Value> {
    diags
        .iter()
        .map(|d| {
            json!({
                "code": d.code.as_str(),
                "severity": format!("{:?}", d.severity).to_lowercase(),
                "message": d.message,
                "entity_ref": d.entity_ref,
            })
        })
        .collect()
}

/// Render the result page in its failure shape, with a fitting status.
fn fail(is_admin: bool, status: StatusCode, message: &str) -> Response {
    let mut resp = render::page(
        "convert_result.html",
        context! {
            nav => "convert",
            is_admin,
            ok => false,
            error => message,
            diagnostics => Vec::<Value>::new(),
        },
    );
    *resp.status_mut() = status;
    resp
}
