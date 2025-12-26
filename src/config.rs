use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub storage_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap(),
            jwt_secret: env::var("JWT_SECRET").unwrap(),
            storage_path: env::var("STORAGE_PATH").unwrap_or("storage".into()),
        }
    }
}
