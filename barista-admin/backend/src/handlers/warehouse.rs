use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub sku: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub reserved_quantity: i32,
    pub unit: String,
}

#[derive(Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub sku: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub unit: Option<String>,
}

// Получить список всех товаров
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

// Создать новый товар
async fn create_product(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProductDto>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    let unit = payload.unit.unwrap_or_else(|| "шт".to_string());

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
        unit
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::BAD_REQUEST
    })?;

    Ok((StatusCode::CREATED, Json(product)))
}
