use axum::{
    extract::{State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

// ==================== МОДЕЛИ ДАННЫХ ====================

// --- Товары / Склад ---
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

// --- Юрлица (Клиенты) ---
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub inn: String,
    pub kpp: Option<String>,
    pub legal_address: Option<String>,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateCompanyDto {
    pub name: String,
    pub inn: String,
    pub kpp: Option<String>,
    pub legal_address: Option<String>,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
}

// --- Кофемашины ---
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Machine {
    pub id: i32,
    pub company_id: Option<i32>,
    pub model: String,
    pub serial_number: String,
    pub installation_address: String,
    pub qr_token: String,
}

#[derive(Deserialize)]
pub struct CreateMachineDto {
    pub company_id: Option<i32>,
    pub model: String,
    pub serial_number: String,
    pub installation_address: String,
}

// --- Заказы и резервирование ---
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Order {
    pub id: i32,
    pub company_id: i32,
    pub machine_id: Option<i32>,
    pub status: String,
    pub total_amount: Decimal,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OrderItemDto {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateOrderDto {
    pub company_id: i32,
    pub machine_id: Option<i32>,
    pub items: Vec<OrderItemDto>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

// ==================== ТОЧКА ВХОДА ====================

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
        .route("/api/companies", get(get_companies).post(create_company))
        .route("/api/machines", get(get_machines).post(create_machine))
        .route("/api/orders", post(create_order))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server started on 0.0.0.0:8000");

    axum::serve(listener, app).await.unwrap();
}

// ==================== ОБРАБОТЧИКИ (HANDLERS) ====================

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Barista Admin API is running on Rust Axum".to_string(),
    })
}

// --- Товары ---

async fn get_products(State(pool): State<PgPool>) -> Result<Json<Vec<Product>>, StatusCode> {
    let rows = sqlx::query_as!(
        Product,
        r#"SELECT id, name, sku, price, stock_quantity, reserved_quantity, unit FROM products ORDER BY id DESC"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| { eprintln!("DB error: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    Ok(Json(rows))
}

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
    .map_err(|e| { eprintln!("DB error: {}", e); StatusCode::BAD_REQUEST })?;

    Ok((StatusCode::CREATED, Json(product)))
}

// --- Клиенты (Юрлица) ---

async fn get_companies(State(pool): State<PgPool>) -> Result<Json<Vec<Company>>, StatusCode> {
    let rows = sqlx::query_as!(
        Company,
        r#"SELECT id, name, inn, kpp, legal_address, contact_person, phone FROM companies ORDER BY id DESC"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| { eprintln!("DB error: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    Ok(Json(rows))
}

async fn create_company(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCompanyDto>,
) -> Result<(StatusCode, Json<Company>), StatusCode> {
    let company = sqlx::query_as!(
        Company,
        r#"
        INSERT INTO companies (name, inn, kpp, legal_address, contact_person, phone)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, inn, kpp, legal_address, contact_person, phone
        "#,
        payload.name,
        payload.inn,
        payload.kpp.as_deref(),
        payload.legal_address.as_deref(),
        payload.contact_person.as_deref(),
        payload.phone.as_deref()
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| { eprintln!("DB error: {}", e); StatusCode::BAD_REQUEST })?;

    Ok((StatusCode::CREATED, Json(company)))
}

// --- Кофемашины ---

async fn get_machines(State(pool): State<PgPool>) -> Result<Json<Vec<Machine>>, StatusCode> {
    let rows = sqlx::query_as!(
        Machine,
        r#"SELECT id, company_id, model, serial_number, installation_address, qr_token FROM machines ORDER BY id DESC"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| { eprintln!("DB error: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    Ok(Json(rows))
}

async fn create_machine(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateMachineDto>,
) -> Result<(StatusCode, Json<Machine>), StatusCode> {
    let qr_token = Uuid::new_v4().to_string();

    let machine = sqlx::query_as!(
        Machine,
        r#"
        INSERT INTO machines (company_id, model, serial_number, installation_address, qr_token)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, company_id, model, serial_number, installation_address, qr_token
        "#,
        payload.company_id,
        payload.model,
        payload.serial_number,
        payload.installation_address,
        qr_token
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| { eprintln!("DB error: {}", e); StatusCode::BAD_REQUEST })?;

    Ok((StatusCode::CREATED, Json(machine)))
}

// --- Заказы и двухэтапное резервирование ---

async fn create_order(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateOrderDto>,
) -> Result<(StatusCode, Json<Order>), StatusCode> {
    if payload.items.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tx = pool.begin().await.map_err(|e| {
        eprintln!("Transaction start error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let order = sqlx::query_as!(
        Order,
        r#"
        INSERT INTO orders (company_id, machine_id, status, total_amount)
        VALUES ($1, $2, 'pending', 0)
        RETURNING id, company_id, machine_id, status, total_amount, created_at
        "#,
        payload.company_id,
        payload.machine_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        eprintln!("Order insert error: {}", e);
        StatusCode::BAD_REQUEST
    })?;

    let mut total_amount = Decimal::ZERO;

    for item in payload.items {
        let product = sqlx::query!(
            r#"
            SELECT price, stock_quantity, reserved_quantity 
            FROM products WHERE id = $1 FOR UPDATE
            "#,
            item.product_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Product fetch error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let product = match product {
            Some(p) => p,
            None => return Err(StatusCode::NOT_FOUND),
        };

        let available = product.stock_quantity - product.reserved_quantity;
        if available < item.quantity {
            return Err(StatusCode::CONFLICT);
        }

        let item_total = product.price * Decimal::from(item.quantity);
        total_amount += item_total;

        sqlx::query!(
            r#"
            INSERT INTO order_items (order_id, product_id, quantity, price)
            VALUES ($1, $2, $3, $4)
            "#,
            order.id,
            item.product_id,
            item.quantity,
            product.price
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Order item insert error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        sqlx::query!(
            r#"
            UPDATE products 
            SET reserved_quantity = reserved_quantity + $1 
            WHERE id = $2
            "#,
            item.quantity,
            item.product_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Product reservation update error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    let updated_order = sqlx::query_as!(
        Order,
        r#"
        UPDATE orders 
        SET total_amount = $1 
        WHERE id = $2
        RETURNING id, company_id, machine_id, status, total_amount, created_at
        "#,
        total_amount,
        order.id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        eprintln!("Order total update error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|e| {
        eprintln!("Transaction commit error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(updated_order)))
}
