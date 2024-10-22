// models/product.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProductModel {
    pub id: Uuid, // Alterado para Uuid
    pub name: String,
    pub description: Option<String>, // Agora é opcional
    pub price: f64,
    pub stock: i32,
}

#[derive(Deserialize, Serialize)]
pub struct CreateProductSchema {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub description: Option<String>, // Mantido como opcional
    pub stock: Option<i32>,           // Mantido como opcional
}

#[derive(Deserialize, Serialize)]
pub struct UpdateProductSchema {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub quantity: Option<i32>,
    pub description: Option<String>, // Mantido como opcional
    pub stock: Option<i32>,           // Mantido como opcional
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
