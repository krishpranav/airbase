use axum::{extract::Request, middleware::Next, http::StatusCode};
use jsonwebtoken::*;
use uuid::Uuid;

use crate::{auth::jwt::Claims, auth::context::AuthUser};

pub async fn require_auth(
    mut req: Request,
    next: Next,
) -> Result<axum::response::Response, StatusCode> {
    let token = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(b"supersecretkey"),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?
    .claims;

    req.extensions_mut().insert(AuthUser {
        user_id: Uuid::parse_str(&claims.sub).unwrap(),
        role: claims.role,
    });

    Ok(next.run(req).await)
}