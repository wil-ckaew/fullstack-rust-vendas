use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct PredictionInput {
    pub product_id: uuid::Uuid, // ID do produto
    pub historical_sales: Vec<f64>, // Vendas históricas do produto
    pub current_stock: i32, // Estoque atual do produto
    pub promotional_factor: f64, // Fator de promoção (se houver)
    pub seasonality_factor: f64, // Fator de sazonalidade
}

#[derive(Serialize, Deserialize)]
pub struct PredictionOutput {
    pub predicted_sales: f64, // Vendas previstas
    pub predicted_stock: i32, // Estoque previsto após vendas
    pub confidence: f64, // Confiança na previsão
}

pub async fn make_prediction(input: PredictionInput) -> PredictionOutput {
    let average_sales = input.historical_sales.iter().copied().sum::<f64>() / input.historical_sales.len() as f64;

    let predicted_sales = (average_sales * input.promotional_factor * input.seasonality_factor).min(input.current_stock as f64);

    let predicted_stock = (input.current_stock as f64 - predicted_sales).max(0.0) as i32;

    let confidence = 0.85;

    PredictionOutput {
        predicted_sales,
        predicted_stock,
        confidence,
    }
}

pub async fn save_prediction_to_db(pool: &PgPool, product_id: uuid::Uuid, predicted_sales: f64, predicted_stock: i32) {
    let query = r#"
        INSERT INTO sales_predictions (product_id, predicted_sales, predicted_stock)
        VALUES ($1, $2, $3)
    "#;

    match sqlx::query(query)
        .bind(product_id)
        .bind(predicted_sales)
        .bind(predicted_stock)
        .execute(pool)
        .await
    {
        Ok(_) => println!("Prediction saved successfully."),
        Err(e) => eprintln!("Failed to save prediction: {:?}", e),
    }
}
