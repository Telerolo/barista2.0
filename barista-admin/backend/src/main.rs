use axum::{
    extract::{State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use std::env;

// --- Модели данных ---
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub sku: String,
    pub price: Decimal,
    pub stock_quantity: i32,
    pub reserved_quantity: i32,
    pub unit: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub sku: String,
    pub price: Decimal,
    pub stock_quantity: i32,
    pub unit: Option<String>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

// --- Точка входа ---
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    println!("Database connected successfully!");

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/products", get(get_products).post(create_product))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server started on 0.0.0.0:8000");

    axum::serve(listener, app).await.unwrap();
}

// --- Обработчики (Handlers) ---

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Barista Admin API is running on Rust Axum".to_string(),
    })
}

// Получить список товаров
async fn get_products(State(pool): State<PgPool>) -> Result<Json<Vec<Product>>, StatusCode> {
    let rows = sqlx::query_as!(
        Product,
        r#"
        SELECT id, name, sku, price, stock_quantity, reserved_quantity, unit 
        FROM products ORDER BY id DESC
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(rows))
}

// Создать товар
async fn create_product(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProductDto>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    let product = sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (name, sku, price, stock_quantity, reserved_quantity, unit)
        VALUES ($1, $2, $3, $4, 0, $5)
        RETURNING id, name, sku, price, stock_quantity, reserved_quantity, unit
        "#,
        payload.name,
        payload.sku,
        payload.price,
        payload.stock_quantity,
        payload.unit.as_deref()
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::BAD_REQUEST
    })?;

    Ok((StatusCode::CREATED, Json(product)))
}
