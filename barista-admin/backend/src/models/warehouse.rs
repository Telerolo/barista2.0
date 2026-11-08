use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

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

#[derive(Deserialize)]
pub struct UpdateStockDto {
    pub quantity_change: i32, // Положительное (приход) или отрицательное (списание)
}
