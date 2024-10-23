// models/sale.rs
use sqlx::types::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//#[derive(Debug, sqlx::FromRow)]
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]  // Adicionando sqlx::FromRow
pub struct SaleModel {
    pub id: Uuid,
    pub client_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub total: f64,
    pub sale_date: Option<DateTime<Utc>>,  // Ajustado para Option<DateTime<Utc>> para lidar com valores nulos
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSaleSchema {
    pub client_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub total: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSaleSchema {
    pub client_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub quantity: Option<i32>,
    pub total: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub pagina: Option<usize>,
    pub limite: Option<usize>,
}
