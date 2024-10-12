// models/sale.rs
use sqlx::types::BigDecimal;
use chrono::{NaiveDateTime, Utc};
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
    // any other fields
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
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}
