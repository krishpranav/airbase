pub mod handler;
pub mod jwt;
pub mod context;

use axum::{Router, routing::*};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(handler::signup))
        .route("/login", post(handler::login))
}