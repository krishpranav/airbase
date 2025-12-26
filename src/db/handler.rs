use axum::{Json, extract::{Path, State}};
use serde_json::Value;
use uuid::Uuid;
use crate::state::AppState;

pub async fn insert(
    Path(collection): Path<String>,
    State(state): State<AppState>,
    Json(data): Json<Value>,
) -> Json<Value> {
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO documents (id, collection, data) VALUES ($1,$2,$3)"
    )
    .bind(id)
    .bind(collection)
    .bind(data.clone())
    .execute(&state.db)
    .await
    .unwrap();

    let _ = state.events.send("db.insert".into());
    Json(data)
}