use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow; // Make sure to import FromRow

/// Estrutura que representa um cliente no banco de dados.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ClientModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
}

/// Estrutura para criar um novo cliente.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClientSchema {
    pub name: String,
    pub email: String,
    pub phone: String,
}

/// Estrutura para atualizar um cliente existente.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClientSchema {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
