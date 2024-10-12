use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};  // Importação de Utc para manipular datas
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SaleModel {
    pub id: Option<Uuid>,  // O campo id é opcional (Option)
    pub client_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub total: f64,
    pub sale_date: Option<chrono::NaiveDateTime>,  // Data opcional
}

// Exemplo de criação de uma nova venda com data
pub fn create_sample_sale() -> SaleModel {
    SaleModel {
        id: None,
        client_id: Uuid::new_v4(),
        product_id: Uuid::new_v4(),
        quantity: 10,
        total: 150.0,
        sale_date: Some(Utc::now().naive_utc()),  // Usando Utc para data e hora atual
    }
}

/// Estrutura para criar uma nova venda.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSaleSchema {
    pub client_id: Uuid,         // Campo client_id para criação
    pub product_id: Uuid,
    pub quantity: i32,
    pub total: f64,
}

/// Estrutura para atualização de uma venda.
#[derive(Deserialize)]
pub struct UpdateSaleSchema {
    pub product_id: Option<Uuid>,  // Opcional para atualização parcial
    pub quantity: Option<i32>,     // Opcional
    pub total: Option<f64>,        // Opcional
}

/// Estrutura para opções de filtro ao buscar vendas.
#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

// Implementação da função new para SaleModel
impl SaleModel {
    pub fn new(client_id: Uuid, product_id: Uuid, quantity: i32, total: f64) -> Self {
        SaleModel {
            id: Some(Uuid::new_v4()),  // Corrigido para envolver o UUID em Some()
            client_id,
            product_id,
            quantity,
            total,
            sale_date: Some(Utc::now().naive_utc()),  // Usando NaiveDateTime para o campo de data
        }
    }
}
