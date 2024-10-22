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
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
