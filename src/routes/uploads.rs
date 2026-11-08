//! Reading a file a form sends.
//!
//! A file is kept up to the site's limit ([`crate::documents::MAX_UPLOAD`]).
//! When it is over it, the rest is still read, and thrown away, before the
//! answer goes: a browser still sending when the server answers and closes
//! can lose that answer, and the operator saw a blank page instead of "this
//! file is over the limit" (a 20 MB PDF, 8 November 2026). The routes that
//! take a file therefore accept a request of up to
//! [`crate::documents::MAX_REQUEST`], so that what is over can be read to
//! its end.

use axum::extract::multipart::{Field, MultipartError};

/// A file field read to its end: its bytes, or `None` when it was over
/// `max`.
pub async fn read_capped(
    mut field: Field<'_>,
    max: usize,
) -> Result<Option<Vec<u8>>, MultipartError> {
    let mut kept: Vec<u8> = Vec::new();
    let mut over = false;
    while let Some(chunk) = field.chunk().await? {
        if over {
            continue;
        }
        if kept.len() + chunk.len() > max {
            over = true;
            kept = Vec::new();
        } else {
            kept.extend_from_slice(&chunk);
        }
    }
    Ok((!over).then_some(kept))
}
