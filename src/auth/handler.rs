use axum::{Json, extract::State};
use serde::Deserialize;
use uuid::Uuid;

use crate::{state::AppState, utils::password, auth::jwt};

#[derive(Deserialize)]
pub struct Signup {
    pub email: String,
    pub password: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(data): Json<Signup>,
) -> Json<String> {
    let hash = password::hash(&data.password);
    let id = Uuid::new_v4();

    sqlx::query("INSERT INTO users VALUES ($1,$2,$3)")
        .bind(id)
        .bind(&data.email)
        .bind(&hash)
        .execute(&state.db)
        .await
        .unwrap();

    Json(jwt::sign(id.to_string(), "secret"))
}