pub mod handler;

use axum::{Router, routing::*};
use crate::{state::AppState, middleware::auth::require_auth};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/upload", post(handler::upload))
        .layer(axum::middleware::from_fn(require_auth))
}