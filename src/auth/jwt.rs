use jsonwebtoken::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn sign(sub: String, role: String, secret: &str) -> String {
    encode(
        &Header::default(),
        &Claims { sub, role, exp: 2000000000 },
        &EncodingKey::from_secret(secret.as_bytes()),
    ).unwrap()
}