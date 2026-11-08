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
use uuid::Uuid;

// ==================== МОДЕЛИ ДАННЫХ ====================

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
