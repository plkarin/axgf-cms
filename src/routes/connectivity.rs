//! Connectivity (v0.5.0, docs/adr/0012): the site's public address, the
//! sign-in services and the rules every sign-in follows — and, from the
//! Accounts page, the registration policy.
//!
//! Administrators only. A provider's client secret is written here and never
//! shown again: the page says whether one is set, and a blank field keeps it.
//!
//! Two rules keep an administrator from locking every administrator out:
//! a policy that takes passwords away from administrators, or a change to a
//! provider, is refused while no active administrator could still sign in
//! some other way. The emergency token remains, but it is the way back from a
//! mistake, not a reason to allow one.

use axum::extract::{Form, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::json;

use super::admin::{require, Need};
use crate::acl::{Acl, Role, Status};
use crate::render;
use crate::routes::Shared;
use crate::site::{PasswordPolicy, RegistrationMode, Site};
use crate::sso::{self, Kind};

macro_rules! admin_only {
    ($state:expr, $headers:expr) => {
        match require(&$state, &$headers, Need::Admin) {
            Ok(v) => v,
            Err(r) => return r,
        }
    };
}

/// Could an administrator still sign in under `site`? With a password while
/// the policy lets administrators use one; otherwise only through a service
/// that is ready and linked to an active administrator.
pub(crate) fn admins_can_sign_in(site: &Site, acl: &Acl) -> bool {
    if site.sign_in.password.allows(Role::Admin) {
        return true;
    }
    let ready: Vec<&str> = site.providers.ready().into_iter().map(Kind::slug).collect();
    acl.users.iter().any(|u| {
        u.status == Status::Active
            && u.role == Role::Admin
            && u.identities
                .iter()
                .any(|i| ready.contains(&i.provider.as_str()))
    })
}

/// `GET /admin/connectivity`
pub async fn connectivity(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    render_connectivity(&state, &chrome, StatusCode::OK, None, None)
}

fn render_connectivity(
    state: &Shared,
    chrome: &render::Chrome,
    status: StatusCode,
    error: Option<String>,
    notice: Option<String>,
) -> Response {
    let site = state.site();
    let base = site.public_base();
    let providers = Kind::ALL
        .into_iter()
        .map(|k| {
            let p = site.providers.get(k);
            json!({
                "slug": k.slug(),
                "brand": k.brand(),
                "name": k.label(p),
                "enabled": p.enabled,
                "ready": p.is_ready(k),
                "client_id": p.client_id,
                // Whether one is set, never what it is.
                "secret_set": !p.client_secret.trim().is_empty(),
                "tenant": p.tenant,
                "issuer": p.issuer,
                "label": p.label,
                "needs_tenant": k == Kind::Microsoft,
                "needs_issuer": k == Kind::Oidc,
                "redirect_uri": base.as_deref().map(|b| sso::callback_url(b, k)),
                "console": k.console_url(),
            })
        })
        .collect::<Vec<_>>();
    let (linked_admins, without_way) = state.acl_read(|acl| {
        let active = acl.users.iter().filter(|u| u.status == Status::Active);
        let linked_admins = active
            .clone()
            .filter(|u| u.role == Role::Admin && !u.identities.is_empty())
            .count();
        // Accounts that a "no passwords" policy would leave with no way in:
        // nothing linked, and no address for a first sign-in to link by.
        let without_way = active
            .filter(|u| {
                u.identities.is_empty() && u.email.as_deref().is_none_or(|e| e.trim().is_empty())
            })
            .count();
        (linked_admins, without_way)
    });
    let ready = site.providers.ready().len();
    let mut r = render::page_with(
        chrome,
        "admin_connectivity.html",
        context! {
            nav => "admin",
            admin_page => "connectivity",
            public_url => site.public_url.clone(),
            public_base => base.clone(),
            public_https => site.public_is_https(),
            providers,
            ready,
            password_policy => site.sign_in.password.as_str(),
            policies => PasswordPolicy::ALL.iter().map(|p| p.as_str()).collect::<Vec<_>>(),
            link_by_email => site.sign_in.link_by_email,
            domains => site.sign_in.allowed_domains.join("\n"),
            registration => site.registration.mode.as_str(),
            linked_admins,
            without_way,
            outbound_note => state.bind().is_some(),
            error,
            notice,
        },
    );
    *r.status_mut() = status;
    r
}

fn refused(state: &Shared, chrome: &render::Chrome, msg: String) -> Response {
    render_connectivity(
        state,
        chrome,
        StatusCode::UNPROCESSABLE_ENTITY,
        Some(msg),
        None,
    )
}

fn saved(state: &Shared, chrome: &render::Chrome, key: &str) -> Response {
    render_connectivity(state, chrome, StatusCode::OK, None, Some(chrome.t(key)))
}

#[derive(Debug, Default, Deserialize)]
pub struct AddressForm {
    #[serde(default)]
    public_url: String,
}

/// `POST /admin/connectivity/address`
pub async fn save_address(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<AddressForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let raw = f.public_url.trim();
    let value = if raw.is_empty() {
        String::new()
    } else {
        match crate::site::normalise_base(raw) {
            Some(b) => b,
            None => return refused(&state, &chrome, chrome.t("connectivity-address-invalid")),
        }
    };
    if let Err(e) = state.site_mutate(|s| s.public_url = value.clone()) {
        return refused(
            &state,
            &chrome,
            chrome.t_args("connectivity-not-saved", &[("error", e.to_string().into())]),
        );
    }
    // Where the providers send readers back has changed with it.
    state.flows().forget_discovery();
    tracing::info!(by = viewer.name(), public_url = %value, "public address changed");
    saved(&state, &chrome, "connectivity-address-saved")
}

#[derive(Debug, Default, Deserialize)]
pub struct PolicyForm {
    #[serde(default)]
    password: String,
    #[serde(default)]
    link_by_email: Option<String>,
    #[serde(default)]
    domains: String,
}

/// `POST /admin/connectivity/policy`
pub async fn save_policy(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<PolicyForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let Some(policy) = PasswordPolicy::parse(&f.password) else {
        return refused(&state, &chrome, chrome.t("connectivity-policy-unknown"));
    };
    let domains: Vec<String> = f
        .domains
        .split(['\n', ',', ' '])
        .map(|d| d.trim().trim_start_matches('@').to_ascii_lowercase())
        .filter(|d| !d.is_empty())
        .collect();
    if let Some(bad) = domains.iter().find(|d| {
        !d.contains('.')
            || !d
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    }) {
        return refused(
            &state,
            &chrome,
            chrome.t_args(
                "connectivity-domain-invalid",
                &[("domain", bad.clone().into())],
            ),
        );
    }
    let mut proposed = state.site();
    proposed.sign_in.password = policy;
    proposed.sign_in.link_by_email = f.link_by_email.is_some();
    proposed.sign_in.allowed_domains = domains;
    if !state.acl_read(|acl| admins_can_sign_in(&proposed, acl)) {
        return refused(&state, &chrome, chrome.t("connectivity-policy-lockout"));
    }
    let sign_in = proposed.sign_in.clone();
    if let Err(e) = state.site_mutate(|s| s.sign_in = sign_in) {
        return refused(
            &state,
            &chrome,
            chrome.t_args("connectivity-not-saved", &[("error", e.to_string().into())]),
        );
    }
    tracing::warn!(
        by = viewer.name(),
        password = policy.as_str(),
        "sign-in policy changed"
    );
    saved(&state, &chrome, "connectivity-policy-saved")
}

#[derive(Debug, Default, Deserialize)]
pub struct ProviderForm {
    #[serde(default)]
    enabled: Option<String>,
    #[serde(default)]
    client_id: String,
    /// Blank keeps the one set.
    #[serde(default)]
    client_secret: String,
    #[serde(default)]
    clear_secret: Option<String>,
    #[serde(default)]
    tenant: String,
    #[serde(default)]
    issuer: String,
    #[serde(default)]
    label: String,
}

/// `POST /admin/connectivity/provider/:slug`
pub async fn save_provider(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(slug): Path<String>,
    Form(f): Form<ProviderForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let Some(kind) = Kind::parse(&slug) else {
        return refused(&state, &chrome, chrome.t("sso-error-not-configured"));
    };
    let mut proposed = state.site();
    {
        let p = proposed.providers.get_mut(kind);
        p.enabled = f.enabled.is_some();
        p.client_id = f.client_id.trim().to_string();
        if f.clear_secret.is_some() {
            p.client_secret.clear();
        } else if !f.client_secret.trim().is_empty() {
            p.client_secret = f.client_secret.trim().to_string();
        }
        p.tenant = f.tenant.trim().to_string();
        p.issuer = f.issuer.trim().trim_end_matches('/').to_string();
        p.label = f.label.trim().chars().take(60).collect();
    }
    let p = proposed.providers.get(kind).clone();
    if kind == Kind::Oidc && !p.issuer.is_empty() && kind.discovery_url(&p).is_none() {
        return refused(&state, &chrome, chrome.t("connectivity-issuer-invalid"));
    }
    if kind == Kind::Microsoft && !p.tenant.is_empty() && kind.discovery_url(&p).is_none() {
        return refused(&state, &chrome, chrome.t("connectivity-tenant-invalid"));
    }
    if p.enabled && !p.is_ready(kind) {
        return refused(
            &state,
            &chrome,
            chrome.t_args(
                "connectivity-provider-incomplete",
                &[("name", kind.label(&p).into())],
            ),
        );
    }
    if !state.acl_read(|acl| admins_can_sign_in(&proposed, acl)) {
        return refused(&state, &chrome, chrome.t("connectivity-policy-lockout"));
    }
    if let Err(e) = state.site_mutate(|s| *s.providers.get_mut(kind) = p.clone()) {
        return refused(
            &state,
            &chrome,
            chrome.t_args("connectivity-not-saved", &[("error", e.to_string().into())]),
        );
    }
    state.flows().forget_discovery();
    tracing::warn!(
        by = viewer.name(),
        provider = kind.slug(),
        enabled = p.enabled,
        "sign-in service changed"
    );
    render_connectivity(
        &state,
        &chrome,
        StatusCode::OK,
        None,
        Some(chrome.t_args(
            "connectivity-provider-saved",
            &[("name", kind.label(&p).into())],
        )),
    )
}

/// `POST /admin/connectivity/test/:slug` — can this machine reach it?
pub async fn test_provider(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    let Some(kind) = Kind::parse(&slug) else {
        return refused(&state, &chrome, chrome.t("sso-error-not-configured"));
    };
    let site = state.site();
    let p = site.providers.get(kind).clone();
    match sso::probe(state.flows(), kind, &p).await {
        Ok(found) => render_connectivity(
            &state,
            &chrome,
            StatusCode::OK,
            None,
            Some(chrome.t_args(
                "connectivity-test-ok",
                &[("name", kind.label(&p).into()), ("answer", found.into())],
            )),
        ),
        Err(e) => {
            tracing::info!(provider = kind.slug(), error = %e.detail(), "a sign-in service could not be reached");
            render_connectivity(
                &state,
                &chrome,
                StatusCode::OK,
                Some(chrome.t_args(
                    "connectivity-test-failed",
                    &[
                        ("name", kind.label(&p).into()),
                        ("detail", e.detail().into()),
                    ],
                )),
                None,
            )
        }
    }
}

// ---------------------------------------------------------------------------
// The registration policy, from the Accounts page
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize)]
pub struct RegistrationForm {
    #[serde(default)]
    mode: String,
    /// Draw a new secret, which shuts the old one out.
    #[serde(default)]
    rotate: Option<String>,
}

/// `POST /admin/users/registration`
pub async fn save_registration(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<RegistrationForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let Some(mode) = RegistrationMode::parse(&f.mode) else {
        let mut r = super::admin::render_users(
            &state,
            &chrome,
            &viewer,
            Some(&chrome.t("registration-unknown")),
            None,
        );
        *r.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
        return r;
    };
    let by = viewer.name().to_string();
    let rotate = f.rotate.is_some();
    let r = state.site_mutate(|s| {
        let reg = &mut s.registration;
        if mode == RegistrationMode::Secret && (reg.secret.is_empty() || rotate) {
            reg.secret = crate::site::generate_secret();
        }
        reg.mode = mode;
        reg.changed_at = Some(crate::view::now_iso8601());
        reg.changed_by = Some(by.clone());
    });
    if let Err(e) = r {
        let msg = chrome.t_args("accounts-not-saved", &[("error", e.to_string().into())]);
        return super::admin::render_users(&state, &chrome, &viewer, Some(&msg), None);
    }
    tracing::warn!(by = %by, mode = mode.as_str(), rotated = rotate, "registration policy changed");
    // Back to the page, at the section, so a reload does not post it again.
    let _ = &chrome;
    Redirect::to("/admin/users#registration").into_response()
}
