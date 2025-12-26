use axum::{Json, extract::{Path, State, Extension}, http::StatusCode};
use serde_json::Value;
use uuid::Uuid;
use serde_json::json;
use crate::{state::AppState, auth::context::AuthUser};

pub async fn insert(
    Path(collection): Path<String>,
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(data): Json<Value>,
) -> Json<Value> {
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO documents (id,collection,data,owner_id)
         VALUES ($1,$2,$3,$4)"
    )
    .bind(id)
    .bind(&collection)
    .bind(&data)
    .bind(user.user_id)
    .execute(&state.db)
    .await
    .unwrap();

    state.events.send(
        json!({"type":"db.insert","collection":collection,"id":id}).to_string()
    ).ok();

    Json(data)
}

pub async fn get(
    Path((collection, id)): Path<(String, Uuid)>,
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Value>, StatusCode> {
    let record = sqlx::query!(
        "SELECT data, owner_id FROM documents WHERE id=$1 AND collection=$2",
        id,
        collection
    )
    .bind(id)
    .bind(&collection)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    if record.owner_id != user.user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(Json(record.data))
}