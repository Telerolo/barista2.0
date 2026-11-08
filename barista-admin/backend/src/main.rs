use axum::{routing::get, Json, Router};
use serde::Serialize;
use sqlx::PgPool;
use std::env;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

#[tokio::main]
async fn main() {
    // Загружаем .env если он есть локально
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Подключение к пулу PostgreSQL
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    println!("Database connected successfully!");

    // Роутер Axum
    let app = Router::new()
        .route("/api/health", get(health_check))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server started on 0.0.0.0:8000");

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Barista Admin API is running on Rust Axum".to_string(),
    })
}
