use axum::Router;
use axum::routing::*;
use sqlx::PgPool;
use tokio::sync::broadcast;

use crate::{auth, db, storage, realtime};

pub fn create_router(db: PgPool, tx: broadcast::Sender<String>) -> Router {
    let state = crate::state::AppState { db, events: tx };

    Router::new()
        .nest("/auth", auth::routes())
        .nest("/db", db::routes())
        .nest("/storage", storage::routes())
        .nest("/realtime", realtime::routes())
        .with_state(state)
}