use dotenvy::dotenv;
use tracing_subscriber::fmt::init;
use tokio::net::TcpListener;
mod config;
mod state;
mod router;
mod auth;
mod middleware;
mod db;
mod realtime;
mod storage;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init();

    let config = config::Config::from_env();
    let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
    let (tx, _) = tokio::sync::broadcast::channel(100);

    let app = router::create(pool, tx, config);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}