//! The administration console's own rooms (v0.5.0, docs/adr/0011): the AXGF
//! bundle, the infrastructure and the connectivity. The overview is the
//! dashboard (`admin::dashboard`) and the accounts are `admin::users`.
//!
//! Administrators only, every one of them: they say how the machine is
//! exposed, hand out the whole archive and hold the providers' secrets.

use axum::extract::{Form, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::{json, Value};

use super::admin::{require, Need};
use crate::routes::Shared;
use crate::{conformity, documents, findings, render};

/// Refuse anyone but an administrator, as every room here does.
macro_rules! admin_only {
    ($state:expr, $headers:expr) => {
        match require(&$state, &$headers, Need::Admin) {
            Ok(v) => v,
            Err(r) => return r,
        }
    };
}

// ---------------------------------------------------------------------------
// The AXGF bundle
// ---------------------------------------------------------------------------

/// `GET /admin/bundle`
pub async fn bundle(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<BundleQuery>,
) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    let note = BundleNote {
        notice: (q.family.as_deref() == Some("saved")).then(|| chrome.t("manifest-saved")),
        ..Default::default()
    };
    render_bundle(&state, &chrome, note)
}

/// `?family=saved`: back from the family's form.
#[derive(Debug, Default, Deserialize)]
pub struct BundleQuery {
    #[serde(default)]
    family: Option<String>,
}

/// What a bundle page says above its sections after a form.
#[derive(Default)]
struct BundleNote {
    error: Option<String>,
    notice: Option<String>,
    /// The key text the form carried, given back so a refused key can be
    /// corrected rather than pasted again.
    recipient_text: Option<String>,
    /// The family's form as it was sent, given back with its error.
    family: Option<FamilyForm>,
}

/// The attachments the bundle declares, by name.
fn declared_payloads(state: &Shared) -> Vec<String> {
    state.read(|flat| {
        flat.get("external_payloads")
            .and_then(Value::as_object)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    })
}

/// The bundle's conformity findings, from a validation already made. Empty
/// for an installation with no family yet: nothing to conform.
pub(super) fn bundle_items(
    state: &Shared,
    env: &axgf_rs::boundary::envelope::Envelope,
    lang: &str,
) -> Vec<findings::Item> {
    if state.is_unset() {
        return Vec::new();
    }
    let declared = declared_payloads(state);
    let missing = state
        .payloads()
        .missing_among(declared.iter().map(String::as_str))
        .len();
    let manifest = state.read(|flat| flat.get("manifest").cloned());
    let binding = state.acl_read(|acl| acl.check_binding(manifest.as_ref(), None));
    state.read(|flat| {
        conformity::check(
            flat,
            &conformity::Context {
                validation: env,
                missing_payloads: missing,
                declared_payloads: declared.len(),
                binding,
                size: state.bundle_size(),
                size_warn: state.size_warn(),
                lang,
            },
        )
    })
}

/// The four rooms in a line each, for the overview (docs/adr/0011 §1): the
/// bundle's verdict, the infrastructure's, the version, the accounts and the
/// ways in. Administrators only; the dashboard asks for it for them alone.
pub(super) fn overview(
    state: &Shared,
    headers: &HeaderMap,
    env: &axgf_rs::boundary::envelope::Envelope,
    lang: &str,
) -> Value {
    let bundle = bundle_items(state, env, lang);
    let (b_ok, b_warn, b_fail) = findings::tally(&bundle);
    let infra = measure(state, headers, lang);
    let (i_ok, i_warn, i_fail) = findings::tally(&infra.items);
    let site = state.site();
    let (active, admins) = state.acl_read(|acl| {
        (
            acl.users
                .iter()
                .filter(|u| u.status == crate::acl::Status::Active)
                .count(),
            acl.active_admins(),
        )
    });
    json!({
        "bundle": {
            "unset": state.is_unset(),
            "verdict": conformity::verdict(&bundle).as_str(),
            "tally": { "ok": b_ok, "warn": b_warn, "fail": b_fail },
        },
        "infra": {
            "verdict": findings::worst(&infra.items).as_str(),
            "tally": { "ok": i_ok, "warn": i_warn, "fail": i_fail },
        },
        "version": {
            "current": crate::releases::current(),
            "newer": infra.channel.as_ref().and_then(|c| c.newer_stable()).map(|r| r.tag.clone()),
            "known": infra.channel.as_ref().is_some_and(|c| !c.releases.is_empty()),
        },
        "accounts": { "active": active, "admins": admins, "registration": site.registration.mode.as_str() },
        "entry": {
            "services": site.providers.ready().len(),
            "https": site.public_is_https(),
            "address": site.public_base().is_some(),
        },
    })
}

fn render_bundle(state: &Shared, chrome: &render::Chrome, note: BundleNote) -> Response {
    let unset = state.is_unset();
    let env = state.inspect_with(axgf_rs::validate);
    let diagnostics = super::admin::diagnostics_json(&env.diagnostics, chrome.lang);
    let declared = declared_payloads(state);
    let size = state.bundle_size();
    let items = bundle_items(state, &env, chrome.lang);
    let (ok, warn, fail) = findings::tally(&items);
    let verdict = conformity::verdict(&items).as_str();
    let facts = state.read(|flat| conformity::facts(flat, state.bundle_path()));
    let anomalies = state.read(|flat| {
        conformity::anomalies(flat)
            .into_iter()
            .map(|(rule, open, checked)| {
                json!({
                    "rule": rule.slug(),
                    "group": rule.group(),
                    "open": open,
                    "checked": checked,
                })
            })
            .collect::<Vec<_>>()
    });
    let counts = state.counts();
    let total: usize = counts.iter().map(|(_, n)| n).sum();
    let persons = counts
        .iter()
        .find(|(k, _)| *k == "persons")
        .map(|(_, n)| *n)
        .unwrap_or(0);
    let site = state.site();
    let recipient_text = note
        .recipient_text
        .clone()
        .unwrap_or_else(|| site.export_recipient.clone());
    let recipient = if site.export_recipient.trim().is_empty() {
        Value::Null
    } else {
        match crate::seal::read_recipient(&site.export_recipient) {
            Ok(r) => recipient_json(&r),
            Err(e) => json!({ "error": chrome.t(e.key()) }),
        }
    };
    render::page_with(
        chrome,
        "admin_bundle.html",
        context! {
            nav => "admin",
            admin_page => "bundle",
            unset,
            verdict,
            tally => json!({ "ok": ok, "warn": warn, "fail": fail }),
            groups => findings::grouped(&items, &conformity::AREAS, chrome.lang),
            diagnostics,
            anomalies,
            facts,
            total,
            persons,
            bundle_path => state.bundle_path().display().to_string(),
            bundle_size => documents::human_size_in(size, chrome.lang),
            attachments => declared.len(),
            spec_current => conformity::SPEC,
            export_scopes => crate::sensitive::Scope::ALL.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            recipient,
            recipient_text,
            family => family_context(state, chrome, note.family),
            error => note.error,
            notice => note.notice,
        },
    )
}

// ---------------------------------------------------------------------------
// the family the archive describes (manifest.family, manifest.license)
// ---------------------------------------------------------------------------

/// The continents a family can be said to have lived on: the closed list of
/// `manifest.family.primary_place` (a human-readable place, written in
/// English in the file and named in the reader's language on the page).
pub const CONTINENTS: &[(&str, &str)] = &[
    ("Africa", "continent-africa"),
    ("Antarctica", "continent-antarctica"),
    ("Asia", "continent-asia"),
    ("Europe", "continent-europe"),
    ("North America", "continent-north-america"),
    ("South America", "continent-south-america"),
    ("Oceania", "continent-oceania"),
    ("Several continents", "continent-several"),
];

/// The licences an archive may be shared under: the closed list of
/// `manifest.license.type`, each with the address the file records for it
/// (none for the two that are not public licences) and the catalogue key of
/// what it lets others do.
pub const LICENCES: &[(&str, Option<&str>, &str)] = &[
    ("all-rights-reserved", None, "licence-all-rights-reserved"),
    ("family-private", None, "licence-family-private"),
    (
        "CC0-1.0",
        Some("https://creativecommons.org/publicdomain/zero/1.0/"),
        "licence-cc0",
    ),
    (
        "CC-BY-4.0",
        Some("https://creativecommons.org/licenses/by/4.0/"),
        "licence-cc-by",
    ),
    (
        "CC-BY-SA-4.0",
        Some("https://creativecommons.org/licenses/by-sa/4.0/"),
        "licence-cc-by-sa",
    ),
    (
        "CC-BY-NC-4.0",
        Some("https://creativecommons.org/licenses/by-nc/4.0/"),
        "licence-cc-by-nc",
    ),
    (
        "CC-BY-NC-SA-4.0",
        Some("https://creativecommons.org/licenses/by-nc-sa/4.0/"),
        "licence-cc-by-nc-sa",
    ),
    (
        "CC-BY-ND-4.0",
        Some("https://creativecommons.org/licenses/by-nd/4.0/"),
        "licence-cc-by-nd",
    ),
    (
        "CC-BY-NC-ND-4.0",
        Some("https://creativecommons.org/licenses/by-nc-nd/4.0/"),
        "licence-cc-by-nc-nd",
    ),
    (
        "ODbL-1.0",
        Some("https://opendatacommons.org/licenses/odbl/1-0/"),
        "licence-odbl",
    ),
];

/// Who chose the licence, and when: `manifest.license.extensions`, so that
/// another reader of the file keeps it and may ignore it.
pub const LICENCE_STAMP: &str = "axgf-cms:license/v1";

/// The earliest and the latest birth the archive records, as the file
/// writes dates (`1702`, `1702-03`, `1702-03-14`): the period it covers.
pub fn births_span(flat: &Value) -> Option<(String, String)> {
    let mut lo: Option<String> = None;
    let mut hi: Option<String> = None;
    let persons = flat.get("persons").and_then(Value::as_object)?;
    for p in persons.values() {
        let Some(v) = p.pointer("/birth/date/value").and_then(Value::as_str) else {
            continue;
        };
        if crate::notes::date_of(v).is_none() {
            continue;
        }
        if lo.as_deref().is_none_or(|l| v < l) {
            lo = Some(v.to_string());
        }
        if hi.as_deref().is_none_or(|h| v > h) {
            hi = Some(v.to_string());
        }
    }
    Some((lo?, hi?))
}

/// The family's form: `manifest.family` and `manifest.license`. The place
/// and the licence are each one of a closed list (or what the file already
/// says, kept until it is changed); the period is the births', never typed.
#[derive(Debug, Default, Clone, Deserialize, serde::Serialize)]
pub struct FamilyForm {
    #[serde(default)]
    token: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    primary_culture: String,
    #[serde(default)]
    primary_place: String,
    #[serde(default)]
    license_type: String,
}

impl FamilyForm {
    fn of(editable: &Value) -> Self {
        let get = |p: &str| {
            editable
                .pointer(p)
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string()
        };
        FamilyForm {
            token: crate::state::manifest_token(editable),
            name: get("/family/name"),
            description: get("/family/description"),
            primary_culture: get("/family/primary_culture"),
            primary_place: get("/family/primary_place"),
            license_type: get("/license/type"),
        }
    }

    /// Every field trimmed, its control characters dropped (a line break is
    /// kept in the description).
    fn cleaned(mut self) -> Self {
        let line = |s: &str| {
            s.chars()
                .filter(|c| !c.is_control())
                .collect::<String>()
                .trim()
                .to_string()
        };
        self.name = line(&self.name);
        self.description = self
            .description
            .replace("\r\n", "\n")
            .chars()
            .filter(|c| *c == '\n' || !c.is_control())
            .collect::<String>()
            .trim()
            .to_string();
        self.primary_culture = line(&self.primary_culture);
        self.primary_place = line(&self.primary_place);
        self.license_type = line(&self.license_type);
        self
    }

    /// What is wrong with it, as a catalogue key, or nothing. `stored` is
    /// the manifest's editable part: a place or a licence outside the lists
    /// is accepted only as the one the file already has.
    fn refusal(&self, stored: &Value) -> Option<&'static str> {
        let long = |s: &str, n: usize| s.chars().count() > n;
        if long(&self.name, 120) || long(&self.description, 2000) {
            return Some("manifest-error-long");
        }
        let culture_ok = self.primary_culture.is_empty()
            || (self.primary_culture.len() <= 35
                && self
                    .primary_culture
                    .split('-')
                    .enumerate()
                    .all(|(i, part)| {
                        let n = part.len();
                        if i == 0 {
                            (2..=8).contains(&n) && part.bytes().all(|b| b.is_ascii_alphabetic())
                        } else {
                            (1..=8).contains(&n) && part.bytes().all(|b| b.is_ascii_alphanumeric())
                        }
                    }));
        if !culture_ok {
            return Some("manifest-error-culture");
        }
        let kept = |p: &str, v: &str| stored.pointer(p).and_then(Value::as_str) == Some(v);
        if !self.primary_place.is_empty()
            && !CONTINENTS.iter().any(|(c, _)| *c == self.primary_place)
            && !kept("/family/primary_place", &self.primary_place)
        {
            return Some("manifest-error-place");
        }
        if !self.license_type.is_empty()
            && !LICENCES.iter().any(|(c, _, _)| *c == self.license_type)
            && !kept("/license/type", &self.license_type)
        {
            return Some("manifest-error-licence");
        }
        None
    }

    /// Write it into a manifest: a field left empty is taken out, and an
    /// object left with nothing in it goes too, as the format asks. The
    /// period is the births' (`span`); a licence chosen anew carries who
    /// chose it and when, and its address from the list.
    fn apply(
        &self,
        manifest: &mut serde_json::Map<String, Value>,
        span: Option<&(String, String)>,
        who: &str,
        now: &str,
    ) {
        fn set(obj: &mut serde_json::Map<String, Value>, key: &str, value: &str) {
            if value.is_empty() {
                obj.remove(key);
            } else {
                obj.insert(key.into(), Value::from(value));
            }
        }
        let mut family = manifest
            .get("family")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        set(&mut family, "name", &self.name);
        set(&mut family, "description", &self.description);
        set(&mut family, "primary_culture", &self.primary_culture);
        set(&mut family, "primary_place", &self.primary_place);
        match span {
            Some((lo, hi)) => {
                let mut ts = family
                    .get("time_span")
                    .and_then(Value::as_object)
                    .cloned()
                    .unwrap_or_default();
                ts.insert("earliest".into(), Value::from(lo.as_str()));
                ts.insert("latest".into(), Value::from(hi.as_str()));
                family.insert("time_span".into(), Value::Object(ts));
            }
            None => {
                family.remove("time_span");
            }
        }
        let mut license = manifest
            .get("license")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let before = license
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        if self.license_type.is_empty() {
            license.clear();
        } else if self.license_type != before {
            license.insert("type".into(), Value::from(self.license_type.as_str()));
            match LICENCES
                .iter()
                .find(|(c, _, _)| *c == self.license_type)
                .and_then(|l| l.1)
            {
                Some(url) => license.insert("url".into(), Value::from(url)),
                None => license.remove("url"),
            };
            // A note written for the licence before says nothing of this one.
            license.remove("note");
            let mut ext = license
                .get("extensions")
                .and_then(Value::as_object)
                .cloned()
                .unwrap_or_default();
            ext.insert(
                LICENCE_STAMP.into(),
                serde_json::json!({ "by": who, "at": now }),
            );
            license.insert("extensions".into(), Value::Object(ext));
        }
        for (key, obj) in [("family", family), ("license", license)] {
            if obj.is_empty() {
                manifest.remove(key);
            } else {
                manifest.insert(key.into(), Value::Object(obj));
            }
        }
    }
}

/// The family's form for the page: the form (as sent with an error, or as
/// the manifest stands), the lists it chooses from, the period the births
/// give, and who chose the licence.
fn family_context(state: &Shared, chrome: &render::Chrome, sent: Option<FamilyForm>) -> Value {
    let editable = state.manifest_editable();
    let form = sent.unwrap_or_else(|| FamilyForm::of(&editable));
    let span = state.read(births_span);
    let stored_place = editable
        .pointer("/family/primary_place")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let stored_licence = editable
        .pointer("/license/type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let mut places: Vec<Value> = CONTINENTS
        .iter()
        .map(|(c, k)| json!({ "value": c, "label": chrome.t(k) }))
        .collect();
    if !stored_place.is_empty() && !CONTINENTS.iter().any(|(c, _)| *c == stored_place) {
        places.insert(0, json!({ "value": stored_place, "label": stored_place }));
    }
    let mut licences: Vec<Value> = LICENCES
        .iter()
        .map(|(c, _, k)| {
            json!({ "value": c, "label": chrome.t(&format!("{k}-name")),
                    "about": chrome.t(&format!("{k}-about")) })
        })
        .collect();
    if !stored_licence.is_empty() && !LICENCES.iter().any(|(c, _, _)| *c == stored_licence) {
        licences.insert(
            0,
            json!({ "value": stored_licence, "label": stored_licence, "about": "" }),
        );
    }
    let stamp = editable
        .pointer(&format!(
            "/license/extensions/{}",
            LICENCE_STAMP.replace('/', "~1")
        ))
        .cloned();
    json!({
        "form": form,
        "places": places,
        "licences": licences,
        "span": span.map(|(lo, hi)| json!({ "earliest": lo, "latest": hi })),
        "author": stamp.as_ref().and_then(|s| s.get("by")).and_then(Value::as_str)
            .map(|w| crate::journal::who_in(w, chrome.lang)),
        "author_at": stamp.as_ref().and_then(|s| s.get("at")).and_then(Value::as_str),
    })
}

/// `POST /admin/bundle/family` — the family's name and what the archive
/// says about itself (manifest.family, manifest.license).
pub async fn family(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<FamilyForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let f = f.cleaned();
    let refuse = |key: &str, status: StatusCode, f: FamilyForm| {
        let note = BundleNote {
            error: Some(chrome.t(key)),
            family: Some(f),
            ..Default::default()
        };
        let mut resp = render_bundle(&state, &chrome, note);
        *resp.status_mut() = status;
        resp
    };
    if let Some(key) = f.refusal(&state.manifest_editable()) {
        return refuse(key, StatusCode::UNPROCESSABLE_ENTITY, f);
    }
    let span = state.read(births_span);
    let now = crate::view::now_iso8601();
    let outcome = state.update_manifest(viewer.name(), &f.token, |m| {
        f.apply(m, span.as_ref(), viewer.name(), &now)
    });
    match outcome {
        Ok(crate::state::ManifestOutcome::Applied) => {
            tracing::info!(by = viewer.name(), "manifest changed");
            Redirect::to("/admin/bundle?family=saved#family").into_response()
        }
        Ok(crate::state::ManifestOutcome::Unchanged) => {
            Redirect::to("/admin/bundle#family").into_response()
        }
        Ok(crate::state::ManifestOutcome::Conflict) => {
            // What was sent stays in the form, now made from the version
            // that is there: saved again, it replaces it knowingly.
            let mut f = f;
            f.token = crate::state::manifest_token(&state.manifest_editable());
            refuse("manifest-error-stale", StatusCode::CONFLICT, f)
        }
        Err(e) => {
            tracing::error!(error = %e, "could not write the manifest");
            refuse("manifest-error-io", StatusCode::INTERNAL_SERVER_ERROR, f)
        }
    }
}

/// A recipient as the page shows it.
fn recipient_json(r: &crate::seal::Recipient) -> Value {
    json!({
        "fingerprint": r.fingerprint,
        "encryption_fingerprint": r.encryption_fingerprint,
        "user_ids": r.user_ids,
        "algorithm": r.algorithm,
        "fips": r.fips_approved,
        "expires": r.expires,
    })
}

/// The encrypted export's form.
#[derive(Debug, Default, Deserialize)]
pub struct SealForm {
    #[serde(default)]
    recipient: String,
    /// Keep this key for next time.
    #[serde(default)]
    remember: Option<String>,
    /// Only read the key and say what it is; export nothing.
    #[serde(default)]
    check: Option<String>,
    #[serde(default)]
    health: Option<String>,
    #[serde(default)]
    biometrics: Option<String>,
    #[serde(default)]
    genomics: Option<String>,
    #[serde(default)]
    legal: Option<String>,
    #[serde(default)]
    behaviour: Option<String>,
    /// The page's token, sent back in a cookie once the file starts.
    #[serde(default)]
    download_token: Option<String>,
}

impl SealForm {
    fn scopes(&self) -> crate::sensitive::Scopes {
        use crate::sensitive::Scope;
        [
            (Scope::Health, &self.health),
            (Scope::Biometrics, &self.biometrics),
            (Scope::Genomics, &self.genomics),
            (Scope::Legal, &self.legal),
            (Scope::Behaviour, &self.behaviour),
        ]
        .into_iter()
        .filter(|(_, v)| v.as_deref() == Some("include"))
        .map(|(s, _)| s)
        .collect()
    }
}

/// `POST /admin/bundle/sealed` — the bundle, encrypted to one recipient's
/// OpenPGP key (docs/adr/0011 §4).
///
/// The key is read and checked before anything is exported. The export is
/// written beside the bundle as every download is, encrypted into a second
/// temp file through a fixed buffer, and the plain copy removed before the
/// encrypted one is sent: at no point does the response carry, or the disk
/// keep, an unencrypted copy longer than the encryption takes.
pub async fn sealed(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<SealForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let typed = f.recipient.trim().to_string();
    let text = if typed.is_empty() {
        state.site().export_recipient
    } else {
        typed.clone()
    };
    let refuse = |key: &str, args: &[(&str, fluent::FluentValue)]| {
        let note = BundleNote {
            error: Some(chrome.t_args(key, args)),
            recipient_text: Some(typed.clone()),
            ..Default::default()
        };
        let mut resp = render_bundle(&state, &chrome, note);
        *resp.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
        resp
    };
    if text.is_empty() {
        return refuse("seal-error-no-key", &[]);
    }
    let recipient = match crate::seal::read_recipient(&text) {
        Ok(r) => r,
        Err(e) => {
            let bits = match &e {
                crate::seal::SealError::TooWeak { bits } => *bits as i64,
                _ => 0,
            };
            return refuse(
                e.key(),
                &[
                    ("bits", bits.into()),
                    ("min", (crate::seal::MIN_RSA_BITS as i64).into()),
                ],
            );
        }
    };
    let remember = f.remember.as_deref() == Some("on") || f.check.is_some();
    if remember && text != state.site().export_recipient {
        if let Err(e) = state.site_mutate(|s| s.export_recipient = text.clone()) {
            tracing::warn!(error = %e, "could not keep the export recipient");
        }
    }
    if f.check.is_some() {
        let note = BundleNote {
            notice: Some(chrome.t_args(
                "seal-key-kept",
                &[("fingerprint", recipient.fingerprint.clone().into())],
            )),
            ..Default::default()
        };
        return render_bundle(&state, &chrome, note);
    }

    let plain = match state.export_to_temp_file_with(f.scopes()) {
        Ok(t) => t,
        Err(e) => return refuse("seal-error-io", &[("error", e.to_string().into())]),
    };
    let size = std::fs::metadata(&plain).map(|m| m.len()).unwrap_or(0);
    if let Err(e) = crate::space::ensure_room_for(
        state.data_dir(),
        &crate::space::Need {
            bytes: size,
            what: "the encrypted copy of the bundle",
        },
    ) {
        let _ = std::fs::remove_file(&plain);
        return refuse("seal-error-io", &[("error", e.to_string().into())]);
    }
    let out = state.temp_path("sealed");
    let name = state
        .bundle_path()
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "family.axgf".into());
    let (from, to, inner, who) = (plain.clone(), out.clone(), name.clone(), recipient.clone());
    let done =
        tokio::task::spawn_blocking(move || crate::seal::seal_file(&from, &to, &who, &inner)).await;
    let _ = std::fs::remove_file(&plain);
    match done {
        Ok(Ok(())) => {}
        Ok(Err(e)) => {
            let _ = std::fs::remove_file(&out);
            return refuse(e.key(), &[("error", format!("{e:?}").into())]);
        }
        Err(e) => {
            let _ = std::fs::remove_file(&out);
            return refuse("seal-error-io", &[("error", e.to_string().into())]);
        }
    }
    tracing::info!(
        by = viewer.name(),
        recipient = %recipient.fingerprint,
        algorithm = %recipient.algorithm,
        bytes = size,
        "encrypted export made"
    );
    let mut resp = render::download_from(
        &chrome,
        &format!("{name}.gpg"),
        out,
        "application/pgp-encrypted",
    )
    .await;
    render::mark_download(&mut resp, &headers, f.download_token.as_deref());
    resp
}

/// `POST /admin/bundle/forget-recipient` — stop keeping the export's key.
pub async fn forget_recipient(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    if let Err(e) = state.site_mutate(|s| s.export_recipient.clear()) {
        return render_bundle(
            &state,
            &chrome,
            BundleNote {
                error: Some(
                    chrome.t_args("accounts-not-saved", &[("error", e.to_string().into())]),
                ),
                ..Default::default()
            },
        );
    }
    axum::response::Redirect::to("/admin/bundle#export").into_response()
}

// ---------------------------------------------------------------------------
// The infrastructure
// ---------------------------------------------------------------------------

/// Everything the Infrastructure page and its JSON are made from.
struct Infra {
    items: Vec<findings::Item>,
    volumes: crate::host::Volumes,
    compute: crate::host::Compute,
    channel: Option<crate::releases::Channel>,
    helper: bool,
}

fn measure(state: &Shared, headers: &HeaderMap, lang: &str) -> Infra {
    let cache_base = state
        .payloads()
        .dir()
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| state.payloads().dir().to_path_buf());
    let volumes = crate::host::volumes(
        state.bundle_path(),
        &cache_base,
        state.backup_dir(),
        state.space_probe(),
    );
    let compute = crate::host::compute(state.events().counting_for());
    let channel = crate::releases::read(state.data_dir());
    let helper = crate::releases::helper_installed(state.data_dir());
    let items = crate::selfcheck::run(&crate::selfcheck::Inputs {
        state,
        request_tls: crate::session::is_tls(headers),
        volumes: &volumes,
        compute: &compute,
        channel: channel.as_ref(),
        helper,
        lang,
    });
    Infra {
        items,
        volumes,
        compute,
        channel,
        helper,
    }
}

/// `GET /admin/infrastructure`
pub async fn infrastructure(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    render_infrastructure(&state, &headers, &chrome, None, None)
}

fn render_infrastructure(
    state: &Shared,
    headers: &HeaderMap,
    chrome: &render::Chrome,
    error: Option<String>,
    notice: Option<String>,
) -> Response {
    let lang = chrome.lang;
    let infra = measure(state, headers, lang);
    let (ok, warn, fail) = findings::tally(&infra.items);
    let verdict = findings::worst(&infra.items).as_str();
    let size = |n: u64| documents::human_size_in(n, lang);
    let v = &infra.volumes;
    let ours = v.ours().max(1);
    let share = |n: u64| ((n as f64) * 100.0 / ours as f64).clamp(0.0, 100.0);
    let backups_total = v.backups.as_ref().map(|b| b.total).unwrap_or(0);
    let parts = [
        ("bundle", v.bundle),
        ("cache", v.cache),
        ("backups", backups_total),
        ("binary", v.binary),
        ("journal", v.journal),
        ("accounts", v.accounts),
    ]
    .iter()
    .map(|(k, n)| json!({ "key": k, "size": size(*n), "share": share(*n) }))
    .collect::<Vec<_>>();
    let room = v.room.map(|r| {
        let used = r.used_fraction();
        json!({
            "free": size(r.free),
            "total": size(r.total),
            "used": size(r.used()),
            "used_pct": (used * 100.0).round(),
            "level": if used > 1.0 - crate::health::DISK_FAIL_FRACTION { "fail" }
                     else if used > 1.0 - crate::health::DISK_WARN_FRACTION { "warn" } else { "ok" },
            "ours_pct": ((v.ours() as f64) * 100.0 / (r.total.max(1) as f64)).max(0.2),
        })
    });
    let backup_room = v
        .backup_room
        .map(|r| json!({ "free": size(r.free), "total": size(r.total) }));
    let c = &infra.compute;
    let mem = match (c.mem_total, c.mem_available) {
        (Some(t), Some(a)) => Some(json!({
            "total": size(t), "available": size(a),
            "used_pct": (((t - a.min(t)) as f64) * 100.0 / (t.max(1) as f64)).round(),
        })),
        _ => None,
    };
    let cgroup = c.cgroup_current.map(|cur| {
        json!({
            "current": size(cur),
            "max": c.cgroup_max.map(size),
            "pct": c.cgroup_max.map(|m| ((cur as f64) * 100.0 / (m.max(1) as f64)).round()),
        })
    });
    let (up_n, up_unit) =
        crate::selfcheck::coarse(std::time::Duration::from_secs(c.process_uptime_secs));
    let sys_up = c.system_uptime_secs.map(|s| {
        let (n, u) = crate::selfcheck::coarse(std::time::Duration::from_secs(s));
        json!({ "n": n, "unit": u })
    });
    let channel = infra.channel.as_ref();
    let latest = channel
        .and_then(|ch| ch.latest_stable())
        .map(|r| r.tag.clone());
    let newer = channel
        .and_then(|ch| ch.newer_stable())
        .map(|r| r.tag.clone());
    let releases = channel
        .map(|ch| {
            ch.releases
                .iter()
                .map(|r| {
                    let cmp = crate::releases::compare(&r.tag, crate::releases::current());
                    json!({
                        "tag": r.tag,
                        "prerelease": r.prerelease,
                        "published": r.published_at.get(..10).unwrap_or(""),
                        "url": r.url,
                        "relation": match cmp {
                            std::cmp::Ordering::Greater => "newer",
                            std::cmp::Ordering::Equal => "current",
                            std::cmp::Ordering::Less => "older",
                        },
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let ev = state.events();
    let day = crate::events::WINDOW;
    let (count_n, count_unit) = crate::selfcheck::coarse(ev.counting_for().min(day));
    render::page_with(
        chrome,
        "admin_infrastructure.html",
        context! {
            nav => "admin",
            admin_page => "infrastructure",
            verdict,
            tally => json!({ "ok": ok, "warn": warn, "fail": fail }),
            groups => findings::grouped(&infra.items, &crate::selfcheck::AREAS, lang),
            current => crate::releases::current(),
            target => format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS),
            latest,
            newer,
            releases,
            checked_at => channel.and_then(|ch| ch.checked_at.clone()),
            check_error => channel.and_then(|ch| ch.error.clone()),
            source => channel.map(|ch| ch.source.clone()),
            helper => infra.helper,
            pending => crate::releases::pending(state.data_dir()),
            status => crate::releases::status(state.data_dir()),
            token_file => crate::releases::TOKEN_FILE,
            token_set => std::env::var(crate::releases::TOKEN_ENV).is_ok_and(|t| !t.trim().is_empty())
                || channel.is_some_and(|ch| ch.authenticated),
            parts,
            room,
            backup_room,
            binary_path => v.binary_path.clone(),
            data_dir => state.data_dir().display().to_string(),
            bundle_size => size(v.bundle),
            ours => size(v.ours()),
            cache_files => v.cache_files,
            backups => v.backups.as_ref().map(|b| json!({
                "dir": b.dir, "count": b.count, "total": size(b.total),
                "newest": b.newest, "newest_size": size(b.newest_size),
                "newest_at": b.newest_at, "oldest_at": b.oldest_at,
            })),
            growth_month => v.growth_per_day.map(|g| if g >= 0.0 { size((g * 30.0) as u64) } else { format!("−{}", size((-g * 30.0) as u64)) }),
            days_left => v.days_left.map(|d| d.round() as i64),
            compute => c,
            mem,
            cgroup,
            rss => c.rss.map(size),
            uptime => json!({ "n": up_n, "unit": up_unit }),
            sys_up,
            failed => ev.count(crate::events::Kind::SignInFailed, day),
            signed_in_count => ev.count(crate::events::Kind::SignedIn, day),
            registered => ev.count(crate::events::Kind::Registered, day),
            cross_site => ev.count(crate::events::Kind::CrossSiteRefused, day),
            counting => json!({ "n": count_n, "unit": count_unit }),
            sessions => state.sessions().live(),
            error,
            notice,
        },
    )
}

/// `GET /admin/infrastructure.json` — the same checks for a monitor: an
/// administrator's session, every sentence in English.
pub async fn infrastructure_json(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (_viewer, _chrome) = admin_only!(state, headers);
    let infra = measure(&state, &headers, "en");
    let (ok, warn, fail) = findings::tally(&infra.items);
    let body = json!({
        "status": findings::worst(&infra.items).as_str(),
        "tally": { "ok": ok, "warn": warn, "fail": fail },
        "checks": infra.items.iter().map(|i| i.to_json("en")).collect::<Vec<_>>(),
        "version": {
            "current": crate::releases::current(),
            "latest": infra.channel.as_ref().and_then(|c| c.latest_stable()).map(|r| r.tag.clone()),
            "upgrade_helper": infra.helper,
        },
        "volumes": infra.volumes,
        "compute": infra.compute,
    });
    (
        [
            (axum::http::header::CONTENT_TYPE, "application/json"),
            (axum::http::header::CACHE_CONTROL, "no-store"),
        ],
        body.to_string(),
    )
        .into_response()
}

/// `POST /admin/infrastructure/releases` — fetch the list of releases again:
/// through the helper when it is installed, from this process otherwise.
pub async fn refresh_releases(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    let dir = state.data_dir().to_path_buf();
    if crate::releases::helper_installed(&dir) {
        if let Err(e) = crate::releases::request_list(&dir) {
            return render_infrastructure(
                &state,
                &headers,
                &chrome,
                Some(chrome.t_args(
                    "infra-releases-not-asked",
                    &[("error", e.to_string().into())],
                )),
                None,
            );
        }
        return render_infrastructure(
            &state,
            &headers,
            &chrome,
            None,
            Some(chrome.t("infra-releases-asked")),
        );
    }
    // From this process: at most once a minute, whoever is clicking.
    if state
        .note_release_check()
        .is_some_and(|last| last.elapsed() < std::time::Duration::from_secs(60))
    {
        return axum::response::Redirect::to("/admin/infrastructure#version").into_response();
    }
    let _ = tokio::task::spawn_blocking(move || crate::releases::fetch_direct(&dir)).await;
    axum::response::Redirect::to("/admin/infrastructure#version").into_response()
}

#[derive(Debug, Default, Deserialize)]
pub struct UpgradeForm {
    #[serde(default)]
    version: String,
    /// For a version older than this one: the tag, typed again.
    #[serde(default)]
    confirm: String,
}

/// `POST /admin/infrastructure/upgrade` — ask the helper for a version
/// (docs/adr/0011 §5). Only a published tag, only with the helper installed,
/// and an older version only when its tag is typed a second time.
pub async fn upgrade(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<UpgradeForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let dir = state.data_dir().to_path_buf();
    let refuse = |key: &str| {
        let mut r = render_infrastructure(
            &state,
            &headers,
            &chrome,
            Some(chrome.t_args(key, &[("version", f.version.clone().into())])),
            None,
        );
        *r.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
        r
    };
    if !crate::releases::helper_installed(&dir) {
        return refuse("infra-upgrade-no-helper");
    }
    if let Some(waiting) = crate::releases::pending(&dir) {
        // One request at a time: a second would race the first in the helper.
        let mut r = render_infrastructure(
            &state,
            &headers,
            &chrome,
            Some(chrome.t_args(
                "infra-upgrade-already-pending",
                &[("version", waiting.version.into())],
            )),
            None,
        );
        *r.status_mut() = StatusCode::CONFLICT;
        return r;
    }
    let Some(channel) = crate::releases::read(&dir) else {
        return refuse("infra-upgrade-unknown-version");
    };
    if !crate::releases::is_tag(&f.version) || !channel.has(&f.version) {
        return refuse("infra-upgrade-unknown-version");
    }
    if crate::releases::compare(&f.version, crate::releases::current()) == std::cmp::Ordering::Equal
    {
        return refuse("infra-upgrade-same-version");
    }
    let older = crate::releases::compare(&f.version, crate::releases::current())
        == std::cmp::Ordering::Less;
    if older && f.confirm.trim() != f.version {
        return refuse("infra-upgrade-confirm-older");
    }
    match crate::releases::request_upgrade(&dir, &f.version, viewer.name()) {
        Ok(req) => {
            tracing::warn!(
                by = viewer.name(),
                version = %req.version,
                from = crate::releases::current(),
                "upgrade requested from the Infrastructure page"
            );
            render_infrastructure(
                &state,
                &headers,
                &chrome,
                None,
                Some(chrome.t_args("infra-upgrade-asked", &[("version", req.version.into())])),
            )
        }
        Err(e) => render_infrastructure(
            &state,
            &headers,
            &chrome,
            Some(chrome.t_args("infra-releases-not-asked", &[("error", e.into())])),
            None,
        ),
    }
}
