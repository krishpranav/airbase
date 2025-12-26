use axum::Router;
use crate::{auth, db, realtime, storage, state::AppState};

pub fn create(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/db", db::routes())
        .nest("/realtime", realtime::routes())
        .nest("/storage", storage::routes())
        .with_state(state)
}