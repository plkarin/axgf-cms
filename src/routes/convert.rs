//! The GEDCOM conversion page.

use axum::extract::{Multipart, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use minijinja::context;
use serde_json::{json, Value};

use crate::convert::{axgf_name_for, counts_of, MAX_UPLOAD};
use crate::gedcom_in::{self, Refusal};
use crate::routes::Shared;
use crate::state::envelope_into_data;
use crate::{auth, render};

/// `GET /convert` — the upload form.
pub async fn form(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/convert");
    render::page_with(
        &chrome,
        "convert.html",
        context! {
            nav => "convert",
            max_mb => MAX_UPLOAD / (1024 * 1024),
        },
    )
}

/// Fields pulled out of the upload form.
#[derive(Default)]
pub(super) struct Upload {
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
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/convert");
    convert_and_show(&state, &chrome, multipart, Purpose::Download).await
}

/// What a conversion is for: a file to take away, or the family an
/// installation is about to be set up with (docs/adr/0005). The same reading,
/// the same conversion and the same report either way; only the last
/// section differs — a download, or the button that sets the archive up.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Purpose {
    Download,
    Setup,
}

/// Read an uploaded GEDCOM of any version (`crate::gedcom_in`), convert it,
/// and show what came across and how the file was read.
pub(super) async fn convert_and_show(
    state: &Shared,
    chrome: &render::Chrome,
    multipart: Multipart,
    purpose: Purpose,
) -> Response {
    let chrome = chrome.clone();
    let setup = purpose == Purpose::Setup;
    let fail = |chrome: &render::Chrome, status: StatusCode, message: &str| {
        fail(chrome, status, message, setup)
    };
    let limit_mb = || ((MAX_UPLOAD / (1024 * 1024)) as i64).into();
    let mut upload = match read_upload(multipart).await {
        Ok(u) => u,
        Err(UploadError::TooLarge) => {
            return fail(
                &chrome,
                StatusCode::PAYLOAD_TOO_LARGE,
                &chrome.t_args("convert-error-too-large", &[("limit", limit_mb())]),
            )
        }
        Err(UploadError::Unreadable(e)) => {
            return fail(
                &chrome,
                StatusCode::BAD_REQUEST,
                &chrome.t_args("convert-error-unreadable", &[("error", e.into())]),
            )
        }
    };

    if upload.bytes.is_empty() {
        return fail(&chrome, StatusCode::OK, &chrome.t("convert-error-no-file"));
    }
    if upload.bytes.len() > MAX_UPLOAD {
        let size = crate::i18n::decimal(
            chrome.lang,
            &format!("{:.1}", upload.bytes.len() as f64 / (1024.0 * 1024.0)),
        );
        return fail(
            &chrome,
            StatusCode::PAYLOAD_TOO_LARGE,
            &chrome.t_args(
                "convert-error-file-too-large",
                &[("size", size.into()), ("limit", limit_mb())],
            ),
        );
    }
    // The reading, the conversion, the analyses and the export are seconds
    // of work for a large file (7.5 s for 40,000 people in 9.8 MB): done on
    // the blocking pool, so that a conversion holds one of its threads and
    // not one of the few that answer every other request.
    let lang = chrome.lang;
    let confidence = upload.confidence;
    let place_lang = upload.lang.clone();
    let bytes = std::mem::take(&mut upload.bytes);
    let work =
        tokio::task::spawn_blocking(move || work(&bytes, confidence, &place_lang, lang)).await;
    let done = match work {
        Ok(Work::Refused(Refusal::NotGedcom)) => {
            return fail(
                &chrome,
                StatusCode::OK,
                &chrome.t("convert-error-not-gedcom"),
            )
        }
        Ok(Work::Refused(Refusal::NoDataset)) => {
            return fail(
                &chrome,
                StatusCode::OK,
                &chrome.t("convert-error-no-dataset"),
            )
        }
        Ok(Work::Refused(Refusal::TooLarge)) => {
            return fail(
                &chrome,
                StatusCode::PAYLOAD_TOO_LARGE,
                &chrome.t_args("convert-error-too-large", &[("limit", limit_mb())]),
            )
        }
        Ok(Work::NotConverted {
            reading,
            diagnostics,
        }) => {
            return render::page_with(
                &chrome,
                "convert_result.html",
                context! {
                    nav => if setup { "admin" } else { "convert" },
                    ok => false,
                    setup,
                    error => chrome.t("convert-error-refused"),
                    diagnostics,
                    filename => upload.filename,
                    reading,
                },
            );
        }
        Ok(Work::NotPackaged(e)) => {
            return fail(
                &chrome,
                StatusCode::INTERNAL_SERVER_ERROR,
                &chrome.t_args("convert-error-packaging", &[("error", e.into())]),
            )
        }
        Ok(Work::Done(d)) => d,
        // The work panicked: a bug, said as one, rather than a dropped
        // connection.
        Err(e) => {
            return fail(
                &chrome,
                StatusCode::INTERNAL_SERVER_ERROR,
                &chrome.t_args(
                    "convert-error-packaging",
                    &[("error", e.to_string().into())],
                ),
            )
        }
    };
    let Done {
        reading,
        diagnostics,
        counts,
        completeness,
        quality,
        quality_total,
        bytes,
    } = *done;
    let total: usize = counts.iter().map(|(_, n)| n).sum();
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

    render::page_with(
        &chrome,
        "convert_result.html",
        context! {
            nav => if setup { "admin" } else { "convert" },
            ok => true,
            setup,
            reading,
            filename => upload.filename,
            download_id => id,
            download_name,
            quality,
            quality_total,
            quality_listed => QUALITY_LISTED,
            size_kb => size / 1024,
            total,
            counts => counts.iter().zip(crate::admin::KINDS.iter())
                            .map(|((_, n), singular)| json!({"singular": singular, "n": n}))
                            .collect::<Vec<_>>(),
            skipped,
            others,
            confidence => upload.confidence,
            lang => upload.lang,
            completeness,
        },
    )
}

/// How many of a converted file's data-quality findings its page lists.
pub const QUALITY_LISTED: usize = 200;

/// What a conversion came to, worked out off the async runtime.
enum Work {
    /// Not a GEDCOM, or a zip with none in it, or one too large.
    Refused(Refusal),
    /// Read, and refused by the converter.
    NotConverted {
        reading: Value,
        diagnostics: Vec<Value>,
    },
    /// Converted, and the library would not package it.
    NotPackaged(String),
    Done(Box<Done>),
}

struct Done {
    reading: Value,
    diagnostics: Vec<Value>,
    counts: Vec<(&'static str, usize)>,
    completeness: crate::completeness::Report,
    quality: Vec<Value>,
    quality_total: usize,
    bytes: Vec<u8>,
}

/// Read an upload as GEDCOM of any version (`crate::gedcom_in`), convert it,
/// say what the data-quality screen would say about it, and package it.
fn work(upload: &[u8], confidence: f64, place_lang: &str, lang: &'static str) -> Work {
    // Any version, any encoding, bare or in a `.gdz`: read into the dialect
    // the converter reads, and what was mended on the way kept for the page.
    let read = match gedcom_in::read(upload, MAX_UPLOAD) {
        Ok(r) => r,
        Err(r) => return Work::Refused(r),
    };
    let reading = reading_view(&read.reading);

    // The library owns the conversion. This crate only reports the result.
    let env = axgf_rs::convert_gedcom(read.text.as_bytes(), confidence, place_lang);
    drop(read);
    let diagnostics = super::admin::diagnostics_json(&env.diagnostics, lang);
    let data = match envelope_into_data(env) {
        Ok(d) => d,
        Err(_) => {
            return Work::NotConverted {
                reading,
                diagnostics,
            }
        }
    };
    let bundle = data.get("bundle").cloned().unwrap_or(Value::Null);
    drop(data);
    let counts = counts_of(&bundle);
    // Counted from the file the visitor just uploaded, so the argument for the
    // format is made with their own data rather than a curated demo.
    let completeness = crate::completeness::analyse(&bundle, lang);
    // What the data-quality screen would say about this file, said now: the
    // uploader's own file, so every finding is theirs to see. Listed up to a
    // point: a messy file of 20,000 people had 32,000 of them, and the page
    // that listed every one weighed 4.7 MB. The count is whole; the
    // data-quality page lists them all once the file is served.
    let findings = crate::quality::detect(&bundle);
    let quality_total = findings.len();
    let quality: Vec<Value> = findings
        .iter()
        .take(QUALITY_LISTED)
        .map(|f| json!({"rule": f.rule.slug(), "message": crate::quality::message(f, lang)}))
        .collect();
    // Export to the bytes the visitor will download. The served bundle is
    // untouched: this never goes near AppState::mutate.
    let bytes = match crate::state::export_to_bytes(&bundle.to_string()) {
        Ok(b) => b,
        Err(e) => return Work::NotPackaged(e.to_string()),
    };
    Work::Done(Box::new(Done {
        reading,
        diagnostics,
        counts,
        completeness,
        quality,
        quality_total,
        bytes,
    }))
}

/// `GET /convert/download/:id` — the converted bundle.
pub async fn download(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, "/convert");
    match state.conversions().get(&id) {
        Some((name, bytes)) => render::bundle_download(&name, bytes),
        None => render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-download-expired-title",
            "error-download-expired-detail",
        ),
    }
}

/// Why an upload could not be read. Two cases, because they deserve different
/// statuses — a body past the limit is a 413, anything else malformed a 400 —
/// and the sentence for each is the reader's, chosen by the handler.
pub(super) enum UploadError {
    TooLarge,
    /// The multipart parser's own message, kept as it wrote it.
    Unreadable(String),
}

/// Read the multipart form, tolerating fields in any order.
pub(super) async fn read_upload(mut multipart: Multipart) -> Result<Upload, UploadError> {
    let mut out = Upload {
        confidence: 0.8,
        lang: "en".to_string(),
        ..Default::default()
    };

    loop {
        let field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            // A truncated or malformed body is a user error, not a panic. The
            // body-limit layer surfaces here too, so it is separated out.
            Err(e) if e.status() == StatusCode::PAYLOAD_TOO_LARGE => {
                return Err(UploadError::TooLarge)
            }
            Err(e) => return Err(UploadError::Unreadable(e.to_string())),
        };

        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "file" => {
                out.filename = field.file_name().unwrap_or("upload.ged").to_string();
                match field.bytes().await {
                    Ok(b) => out.bytes = b.to_vec(),
                    // In practice this is the body limit firing mid-field.
                    Err(_) => return Err(UploadError::TooLarge),
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

/// Render the result page in its failure shape, with a fitting status.
fn fail(chrome: &render::Chrome, status: StatusCode, message: &str, setup: bool) -> Response {
    let mut resp = render::page_with(
        chrome,
        "convert_result.html",
        context! {
            nav => if setup { "admin" } else { "convert" },
            ok => false,
            setup,
            error => message,
            diagnostics => Vec::<Value>::new(),
        },
    );
    *resp.status_mut() = status;
    resp
}

/// How a file was read, for `_gedcom_reading.html`: the facts, which the
/// template words in the reader's language.
fn reading_view(r: &gedcom_in::Reading) -> Value {
    json!({
        "version": r.version,
        "encoding": r.encoding.map(|e| e.label()),
        "declared": r.declared,
        "declared_otherwise": r.declared_otherwise(),
        "gedzip": r.gedzip,
        "media": r.media,
        "lone_crs": r.lone_crs,
        "voids": r.voids,
        "shared_notes": r.shared_notes,
        "stray_bytes": r.stray_bytes,
        "tidied": r.tidied,
        "too_deep": r.too_deep,
    })
}
