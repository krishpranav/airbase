use axum::Router;
use dotenvy::dotenv;
use tracing_subscriber::fmt::init;

mod config;
mod state;
mod router;
mod auth;
mod db;
mod storage;
mod realtime;
mod middleware;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init();

    let cfg = config::Config::from_env();
    let db = sqlx::PgPool::connect(&cfg.database_url).await.unwrap();
    let (tx, _) = tokio::sync::broadcast::channel(100);

    let app = router::create_router(db, tx);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}