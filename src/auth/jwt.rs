use jsonwebtoken::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn sign(sub: String, secret: &str) -> String {
    encode(
        &Header::default(),
        &Claims { sub, exp: 2000000000 },
        &EncodingKey::from_secret(secret.as_ref()),
    ).unwrap()
}