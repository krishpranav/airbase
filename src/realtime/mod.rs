pub mod ws;

use axum::{Router, routing::*};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(ws::handler))
}