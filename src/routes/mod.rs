//! HTTP routing.

mod admin;
mod convert;
mod public;

use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

/// Shared handler state.
pub type Shared = Arc<AppState>;

/// Build the application router.
pub fn router(state: Shared) -> Router {
    Router::new()
        .route("/", get(public::home))
        .route("/tree", get(public::tree))
        .route("/person/:id", get(public::person))
        .route("/document/:id/raw", get(public::document_raw))
        .route("/document/:id/view", get(public::document_view))
        .route("/document/:id/thumb", get(public::document_thumb))
        .route("/convert", get(convert::form))
        .route(
            "/convert/gedcom",
            post(convert::gedcom).layer(DefaultBodyLimit::max(
                crate::convert::MAX_UPLOAD + 64 * 1024,
            )),
        )
        .route("/convert/download/:id", get(convert::download))
        .route("/health", get(public::health))
        // Static admin segments are declared alongside the ":kind" routes;
        // the router matches literal segments in preference to a parameter.
        .route("/admin", get(admin::dashboard))
        .route("/admin/login", get(admin::login_form).post(admin::login))
        .route("/admin/logout", post(admin::logout))
        .route("/admin/validate", post(admin::validate))
        .route("/admin/dedup", post(admin::dedup))
        .route("/admin/export", get(admin::export))
        // Declared before the ":kind" routes so the literal "person" segment
        // and the trailing "document" segment win over the parameter forms.
        .route(
            "/admin/person/:id/document",
            post(admin::upload_document).layer(DefaultBodyLimit::max(
                crate::documents::MAX_UPLOAD + 64 * 1024,
            )),
        )
        .route("/admin/:kind", get(admin::list).post(admin::create))
        .route("/admin/:kind/new", get(admin::new_form))
        .route("/admin/:kind/:id/edit", get(admin::edit_form))
        .route("/admin/:kind/:id", post(admin::update))
        .route("/admin/:kind/:id/delete", post(admin::delete))
        .route("/static/app.css", get(public::css))
        .route("/static/tree.js", get(public::tree_js))
        .fallback(public::not_found)
        .with_state(state)
}
