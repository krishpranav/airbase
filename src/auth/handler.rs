use axum::{Json, extract::State};
use serde::Deserialize;
use uuid::Uuid;

use crate::{state::AppState, utils::password, auth::jwt};

#[derive(Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(data): Json<Credentials>,
) -> Json<String> {
    let id = Uuid::new_v4();
    let hash = password::hash(&data.password);

    sqlx::query("INSERT INTO users (id,email,password,role) VALUES ($1,$2,$3,'user')")
        .bind(id)
        .bind(&data.email)
        .bind(&hash)
        .execute(&state.db)
        .await
        .unwrap();

    Json(jwt::sign(id.to_string(), "user".into(), &state.jwt_secret))
}

pub async fn login(
    State(state): State<AppState>,
    Json(data): Json<Credentials>,
) -> Json<String> {
    let record = sqlx::query(
        "SELECT id,password,role FROM users WHERE email=$1"
    )
    .bind(&data.email)
    .fetch_one(&state.db)
    .await
    .unwrap();

    password::verify(&data.password, &record.password);

    Json(jwt::sign(record.id.to_string(), record.role, &state.jwt_secret))
}