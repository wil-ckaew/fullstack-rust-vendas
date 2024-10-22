use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProductModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: f64, // Keep price as f64
    pub stock_quantity: i32,
}


#[derive(Deserialize, Serialize)]
pub struct CreateProductSchema {
    pub name: String,
    pub price: f64, // This stays the same
    pub description: Option<String>,
    pub stock_quantity: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateProductSchema {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub description: Option<String>,
    pub stock_quantity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
  