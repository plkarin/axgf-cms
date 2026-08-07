//! HTTP routing.

mod public;

use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::state::AppState;

/// Shared handler state.
pub type Shared = Arc<AppState>;

/// Build the application router.
pub fn router(state: Shared) -> Router {
    Router::new()
        .route("/", get(public::home))
        .route("/health", get(public::health))
        .route("/static/app.css", get(public::css))
        .fallback(public::not_found)
        .with_state(state)
}
