use axum::{extract::{Multipart, State, Extension}, Json};
use uuid::Uuid;
use std::fs::create_dir_all;

use crate::{state::AppState, auth::context::AuthUser};

pub async fn upload(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    mut mp: Multipart,
) -> Json<String> {
    create_dir_all("storage").unwrap();
    let id = Uuid::new_v4();
    let data = mp.next_field().await.unwrap().unwrap().bytes().await.unwrap();

    std::fs::write(format!("storage/{}", id), data).unwrap();

    sqlx::query(
        "INSERT INTO files (id, owner_id, path) VALUES ($1,$2,$3)"
    )
    .bind(id)
    .bind(user.user_id)
    .bind(format!("storage/{}", id))
    .execute(&state.db)
    .await
    .unwrap();

    Json(id.to_string())
}