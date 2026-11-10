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
//! Once a family is there, the archive and GEDCOM doors replace it, and
//! only on three conditions (docs/adr/0042, amending docs/adr/0005 §4): the
//! page says what will be lost, the administrator types the word that
//! confirms it, and a backup of the family is written and verified first.
//! A family changed while it was being backed up is not replaced.

use std::io::Write as _;

use axum::extract::{Multipart, Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use minijinja::context;
use serde_json::Value;

use super::admin::{require, Need};
use super::convert::{convert_and_show, Purpose};
use crate::routes::Shared;
use crate::state::{format_diagnostics, Adoption, Over};
use crate::{journal, render};

/// An archive's limit in gigabytes, as the page says it: `4`, or `1,5`.
fn archive_gb(mb: u32, lang: &str) -> String {
    if mb.is_multiple_of(1024) {
        (mb / 1024).to_string()
    } else {
        crate::i18n::decimal(lang, &format!("{:.1}", mb as f64 / 1024.0))
    }
}

macro_rules! admin_door {
    ($state:expr, $headers:expr) => {{
        match require(&$state, &$headers, Need::Admin) {
            // Over a family, the doors replace it (docs/adr/0042).
            Ok((viewer, chrome)) => (viewer, chrome, !$state.is_unset()),
            Err(r) => return r,
        }
    }};
}

/// `GET /admin/setup` — the three doors.
pub async fn page(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (_viewer, chrome, replacing) = admin_door!(state, headers);
    let count = |c: &str| {
        state
            .counts()
            .iter()
            .find(|(k, _)| *k == c)
            .map(|(_, n)| *n)
            .unwrap_or(0)
    };
    render::page_with(
        &chrome,
        "admin_setup.html",
        context! {
            nav => "admin",
            max_mb => state.limits().gedcom_mb,
            max_archive_gb => archive_gb(state.limits().archive_mb, chrome.lang),
            lang => chrome.lang,
            // What a replacement would lose, and where its backup goes.
            replacing,
            persons => count("persons"),
            families => count("families"),
            documents => count("documents"),
            keyword => chrome.t("setup-replace-keyword"),
            backup_dir => backup_dir(&state).display().to_string(),
        },
    )
}

/// Where the backup taken before a replacement goes: the operator's backup
/// directory, or `backups` beside the bundle.
fn backup_dir(state: &Shared) -> std::path::PathBuf {
    state
        .backup_dir()
        .map(std::path::Path::to_path_buf)
        .unwrap_or_else(|| state.data_dir().join("backups"))
}

/// Whether `typed` is the word that confirms a replacement, in the page's
/// language: case, spaces and the apostrophe's shape aside.
fn confirms(chrome: &render::Chrome, typed: &str) -> bool {
    let norm = |s: &str| {
        s.replace(['’', 'ʼ'], "'")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase()
    };
    let want = norm(&chrome.t("setup-replace-keyword"));
    !want.is_empty() && norm(typed) == want
}

/// The family about to be replaced, backed up and verified first
/// (docs/adr/0042): the generation it was at and its archive, or why not.
async fn backed_up(state: &Shared) -> Result<(u64, std::path::PathBuf), String> {
    let at = state.generation();
    let dest = backup_dir(state);
    let bundle = state.bundle_path().to_path_buf();
    let run = tokio::task::spawn_blocking(move || {
        crate::backup::run_as(
            &bundle,
            &dest,
            crate::backup::Retention::default(),
            crate::backup::Kind::PreImport,
        )
    })
    .await;
    match run {
        Ok(Ok(report)) => Ok((at, report.archive)),
        Ok(Err(e)) => Err(format!("{e:#}")),
        Err(e) => Err(e.to_string()),
    }
}

/// A replacement refused before anything was written: the word did not
/// confirm it, or the backup failed.
fn replace_refused(chrome: &render::Chrome, key: &str, error: Option<String>) -> Response {
    let why = match error {
        Some(e) => chrome.t_args(key, &[("error", e.into())]),
        None => chrome.t_args(
            key,
            &[("keyword", chrome.t("setup-replace-keyword").into())],
        ),
    };
    refused(chrome, StatusCode::BAD_REQUEST, &why, false)
}

/// `POST /admin/setup/gedcom` — the third door: read, convert, show. Nothing
/// is kept yet; the page ends on the button that keeps it.
pub async fn gedcom(
    State(state): State<Shared>,
    headers: HeaderMap,
    multipart: Multipart,
) -> Response {
    let (_viewer, chrome, _replacing) = admin_door!(state, headers);
    convert_and_show(&state, &chrome, multipart, Purpose::Setup).await
}

/// `POST /admin/setup/gedcom/:id` — keep the conversion just shown.
pub async fn adopt_gedcom(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    form: Option<axum::extract::Form<ConfirmForm>>,
) -> Response {
    let (viewer, chrome, replacing) = admin_door!(state, headers);
    let typed = form.map(|f| f.0.confirm).unwrap_or_default();
    if replacing && !confirms(&chrome, &typed) {
        return replace_refused(&chrome, "setup-replace-keyword-wrong", None);
    }
    let Some((name, bytes)) = state.conversions().get(&id) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-download-expired-title",
            "error-download-expired-detail",
        );
    };
    // Over a family, it is backed up and verified first (docs/adr/0042).
    let (over, backup) = if replacing {
        match backed_up(&state).await {
            Ok((at, archive)) => (Over::Family(at), Some(archive)),
            Err(e) => return replace_refused(&chrome, "setup-replace-backup-failed", Some(e)),
        }
    } else {
        (Over::Nothing, None)
    };
    // The conversion was kept as the archive it would download as; reading
    // it back is reading exactly what the page showed.
    let st = state.clone();
    let outcome =
        tokio::task::spawn_blocking(move || st.take_in(axgf_rs::import_bundle(&bytes), over)).await;
    finish(
        &state,
        &chrome,
        viewer.name(),
        &format!("GEDCOM · {name}"),
        backup.as_deref(),
        outcome,
    )
}

/// The word that confirms a replacement, as the keep button's form sends it.
#[derive(Debug, Default, serde::Deserialize)]
pub struct ConfirmForm {
    #[serde(default)]
    confirm: String,
}

/// `POST /admin/setup/archive` — the first door: an `.axgf`, streamed to a
/// file beside the bundle (same disk, so the final write is a rename away),
/// then taken in.
pub async fn archive(
    State(state): State<Shared>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let (viewer, chrome, replacing) = admin_door!(state, headers);
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
    let mut typed = String::new();
    let mut total = 0usize;
    // What the administration lets an archive weigh (docs/adr/0041); a body
    // cut at that weight is said to be too large, not unreadable.
    let most = state.limits().archive_bytes();
    let too_large = |e: axum::extract::multipart::MultipartError| {
        if e.status() == StatusCode::PAYLOAD_TOO_LARGE {
            chrome.t("setup-archive-too-large")
        } else {
            e.to_string()
        }
    };
    let written: Result<(), String> = async {
        let mut file = std::fs::File::create(&part).map_err(|e| e.to_string())?;
        while let Some(mut field) = multipart.next_field().await.map_err(too_large)? {
            if field.name() == Some("confirm") {
                typed = field.text().await.unwrap_or_default();
                continue;
            }
            if field.name() != Some("file") {
                continue;
            }
            name = field.file_name().unwrap_or("family.axgf").to_string();
            while let Some(chunk) = field.chunk().await.map_err(too_large)? {
                total += chunk.len();
                if total > most {
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
    // Over a family: the word that confirms it, then a verified backup
    // (docs/adr/0042).
    let (over, backup) = if replacing {
        if !confirms(&chrome, &typed) {
            let _ = std::fs::remove_file(&part);
            return replace_refused(&chrome, "setup-replace-keyword-wrong", None);
        }
        match backed_up(&state).await {
            Ok((at, archive)) => (Over::Family(at), Some(archive)),
            Err(e) => {
                let _ = std::fs::remove_file(&part);
                return replace_refused(&chrome, "setup-replace-backup-failed", Some(e));
            }
        }
    } else {
        (Over::Nothing, None)
    };
    let (st, path) = (state.clone(), part.clone());
    let outcome = tokio::task::spawn_blocking(move || st.take_in_archive(&path, over)).await;
    let _ = std::fs::remove_file(&part);
    finish(
        &state,
        &chrome,
        viewer.name(),
        &format!("AXGF · {name}"),
        backup.as_deref(),
        outcome,
    )
}

/// Every door ends here: the family taken in and journalled, or why not.
fn finish(
    state: &Shared,
    chrome: &render::Chrome,
    who: &str,
    how: &str,
    backup: Option<&std::path::Path>,
    outcome: Result<anyhow::Result<Adoption>, tokio::task::JoinError>,
) -> Response {
    // A replacement names the archive the family it replaced was kept in.
    let backup_name = backup
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned());
    match outcome {
        Ok(Ok(Adoption::Done(report))) => {
            let label = match &backup_name {
                Some(b) => format!("{how} → {b}"),
                None => how.to_string(),
            };
            state.journal_mutation(&journal::entry_for(journal::Record {
                who,
                action: "setup",
                kind: "bundle",
                entity_id: "",
                label: Some(label),
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
            // The masthead of the archive as it now is — its emblem, its
            // name, and no "Import" once it holds a family — rather than as
            // it was when the request came in.
            let chrome = chrome.clone().with_archive(state);
            render::page_with(
                &chrome,
                "admin_setup_result.html",
                context! {
                    nav => "admin",
                    ok => true,
                    how,
                    total,
                    counts,
                    photos => report.extracted + report.reused,
                    mismatches => report.mismatches,
                    backup => backup.map(|p| p.display().to_string()),
                },
            )
        }
        Ok(Ok(Adoption::Occupied)) => closed(chrome),
        Ok(Ok(Adoption::Changed)) => refused(
            chrome,
            StatusCode::CONFLICT,
            &chrome.t("setup-replace-changed"),
            false,
        ),
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
