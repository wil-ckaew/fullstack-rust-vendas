//models/schema.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow; // Make sure to import FromRow


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClientSchema {
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClientSchema {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSaleSchema {
    pub client_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSaleSchema {
    pub client_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub quantity: Option<i32>,
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