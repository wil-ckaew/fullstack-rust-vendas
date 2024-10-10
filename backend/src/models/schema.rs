//models/schema.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
}

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

// Remova esta definição duplicada de ProductModel
#[derive(Debug, Serialize, Deserialize)]
pub struct SaleModel {
    pub id: Uuid,
    pub client_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSaleSchema {
    pub client_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
