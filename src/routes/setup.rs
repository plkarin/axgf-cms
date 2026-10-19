//! Setting an installation up (v0.5.0, docs/adr/0005): three doors into a
//! family, open while it has none.
//!
//! An installation started without a bundle creates an empty one. Until it
//! holds a family, an administrator is offered three ways in:
//!
//! 1. **an `.axgf` archive** — another installation's, or a backup's
//!    `family.axgf` — streamed to disk beside the bundle and taken in as the
//!    server takes its own at startup, photographs and all;
//! 2. **nothing** — the empty archive stays, and the first person is added
//!    with the ordinary editor;
//! 3. **a GEDCOM** of any version, in any encoding, bare or in a `.gdz` —
//!    read by [`crate::gedcom_in`], converted, shown with how it was read
//!    and what came across, and only then, on a second press, kept.
//!
//! Once a family is there every door is closed, and none of them ever
//! writes over one: replacing a family that exists stays `axgf-cms restore`,
//! from a backup, on the command line, on purpose.

use std::io::Write as _;

use axum::extract::{Multipart, Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use minijinja::context;
use serde_json::Value;

use super::admin::{require, Need};
use super::convert::{convert_and_show, Purpose};
use crate::routes::Shared;
use crate::state::{format_diagnostics, Adoption};
use crate::{journal, render};

/// The ceiling on an uploaded archive. A family's photographs make an `.axgf`
/// far larger than its GEDCOM — the operator's is 433 MB — and it is
/// streamed to disk, never held in memory, so the ceiling is about the disk.
pub const MAX_ARCHIVE: usize = 4 * 1024 * 1024 * 1024;

macro_rules! admin_while_unset {
    ($state:expr, $headers:expr) => {{
        let (viewer, chrome) = match require(&$state, &$headers, Need::Admin) {
            Ok(v) => v,
            Err(r) => return r,
        };
        if !$state.is_unset() {
            return closed(&chrome);
        }
        (viewer, chrome)
    }};
}

/// `GET /admin/setup` — the three doors.
pub async fn page(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (_viewer, chrome) = admin_while_unset!(state, headers);
    render::page_with(
        &chrome,
        "admin_setup.html",
        context! {
            nav => "admin",
            max_mb => crate::convert::MAX_UPLOAD / (1024 * 1024),
            max_archive_gb => MAX_ARCHIVE / (1024 * 1024 * 1024),
            lang => chrome.lang,
        },
    )
}

/// `POST /admin/setup/gedcom` — the third door: read, convert, show. Nothing
/// is kept yet; the page ends on the button that keeps it.
pub async fn gedcom(
    State(state): State<Shared>,
    headers: HeaderMap,
    multipart: Multipart,
) -> Response {
    let (_viewer, chrome) = admin_while_unset!(state, headers);
    convert_and_show(&state, &chrome, multipart, Purpose::Setup).await
}

/// `POST /admin/setup/gedcom/:id` — keep the conversion just shown.
pub async fn adopt_gedcom(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let (viewer, chrome) = admin_while_unset!(state, headers);
    let Some((name, bytes)) = state.conversions().get(&id) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-download-expired-title",
            "error-download-expired-detail",
        );
    };
    // The conversion was kept as the archive it would download as; reading
    // it back is reading exactly what the page showed.
    let st = state.clone();
    let outcome =
        tokio::task::spawn_blocking(move || st.adopt(axgf_rs::import_bundle(&bytes))).await;
    finish(
        &state,
        &chrome,
        viewer.name(),
        &format!("GEDCOM · {name}"),
        outcome,
    )
}

/// `POST /admin/setup/archive` — the first door: an `.axgf`, streamed to a
/// file beside the bundle (same disk, so the final write is a rename away),
/// then taken in.
pub async fn archive(
    State(state): State<Shared>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let (viewer, chrome) = admin_while_unset!(state, headers);
    let dir = state
        .bundle_path()
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(std::path::Path::new("."))
        .to_path_buf();
    // The archive, the bundle written from it and its photographs unpacked:
    // three times its size, refused before a byte is written rather than
    // discovered when the disk fills.
    if let Some(len) = headers
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
    {
        if let Err(e) = crate::space::ensure_room_for(
            &dir,
            &crate::space::Need {
                bytes: len.saturating_mul(3),
                what: "the archive",
            },
        ) {
            return refused(
                &chrome,
                StatusCode::INSUFFICIENT_STORAGE,
                &e.to_string(),
                true,
            );
        }
    }
    let part = dir.join(format!(
        ".setup-{}.axgf.part",
        uuid::Uuid::new_v4().simple()
    ));
    let mut name = String::new();
    let mut total = 0usize;
    let written: Result<(), String> = async {
        let mut file = std::fs::File::create(&part).map_err(|e| e.to_string())?;
        while let Some(mut field) = multipart.next_field().await.map_err(|e| e.to_string())? {
            if field.name() != Some("file") {
                continue;
            }
            name = field.file_name().unwrap_or("family.axgf").to_string();
            while let Some(chunk) = field.chunk().await.map_err(|e| e.to_string())? {
                total += chunk.len();
                if total > MAX_ARCHIVE {
                    return Err(chrome.t("setup-archive-too-large"));
                }
                file.write_all(&chunk).map_err(|e| e.to_string())?;
            }
        }
        file.sync_all().map_err(|e| e.to_string())
    }
    .await;
    if let Err(e) = written {
        let _ = std::fs::remove_file(&part);
        return refused(&chrome, StatusCode::BAD_REQUEST, &e, false);
    }
    let is_zip = std::fs::File::open(&part)
        .and_then(|mut f| {
            let mut head = [0u8; 4];
            std::io::Read::read_exact(&mut f, &mut head).map(|_| head)
        })
        .is_ok_and(|h| h == *b"PK\x03\x04");
    if total == 0 || !is_zip {
        let _ = std::fs::remove_file(&part);
        return refused(
            &chrome,
            StatusCode::BAD_REQUEST,
            &chrome.t("setup-not-archive"),
            false,
        );
    }
    let (st, path) = (state.clone(), part.clone());
    let outcome = tokio::task::spawn_blocking(move || st.adopt_archive(&path)).await;
    let _ = std::fs::remove_file(&part);
    finish(
        &state,
        &chrome,
        viewer.name(),
        &format!("AXGF · {name}"),
        outcome,
    )
}

/// Every door ends here: the family taken in and journalled, or why not.
fn finish(
    state: &Shared,
    chrome: &render::Chrome,
    who: &str,
    how: &str,
    outcome: Result<anyhow::Result<Adoption>, tokio::task::JoinError>,
) -> Response {
    match outcome {
        Ok(Ok(Adoption::Done(report))) => {
            state.journal_mutation(&journal::entry_for(journal::Record {
                who,
                action: "setup",
                kind: "bundle",
                entity_id: "",
                label: Some(how.to_string()),
                version_num: None,
                before: None,
                after: None,
            }));
            let counts: Vec<Value> = state
                .counts()
                .iter()
                .zip(crate::admin::KINDS.iter())
                .map(|((_, n), singular)| serde_json::json!({"singular": singular, "n": n}))
                .collect();
            let total: usize = state.counts().iter().map(|(_, n)| n).sum();
            render::page_with(
                chrome,
                "admin_setup_result.html",
                context! {
                    nav => "admin",
                    ok => true,
                    how,
                    total,
                    counts,
                    photos => report.extracted + report.reused,
                    mismatches => report.mismatches,
                },
            )
        }
        Ok(Ok(Adoption::Occupied)) => closed(chrome),
        Ok(Ok(Adoption::Refused(diags))) => refused(
            chrome,
            StatusCode::UNPROCESSABLE_ENTITY,
            &format_diagnostics(&diags),
            true,
        ),
        Ok(Err(e)) => refused(
            chrome,
            StatusCode::INTERNAL_SERVER_ERROR,
            &e.to_string(),
            true,
        ),
        Err(e) => refused(
            chrome,
            StatusCode::INTERNAL_SERVER_ERROR,
            &e.to_string(),
            true,
        ),
    }
}

/// A door that could not open: nothing was written, and why — in the
/// reader's words, or (`technical`) the library's or the system's own, which
/// name what somebody repairing it needs and are not translated.
fn refused(chrome: &render::Chrome, status: StatusCode, why: &str, technical: bool) -> Response {
    let mut resp = render::page_with(
        chrome,
        "admin_setup_result.html",
        context! { nav => "admin", ok => false, why, technical },
    );
    *resp.status_mut() = status;
    resp
}

/// The doors once a family is there.
fn closed(chrome: &render::Chrome) -> Response {
    render::error_page_in(
        chrome,
        StatusCode::CONFLICT,
        "setup-closed-title",
        "setup-closed-detail",
    )
    .into_response()
}
