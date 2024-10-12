// src/handlers/sale_handler.rs
use actix_web::{
    get, post, delete, patch,
    web::{Data, Json, Path, ServiceConfig, Query},
    HttpResponse, Responder,
};
use serde_json::json;
use crate::{
    models::sale::{CreateSaleSchema, UpdateSaleSchema, SaleModel, FilterOptions}, 
    AppState
};
use uuid::Uuid;
use sqlx::PgPool; // Supondo que isso seja necessário


#[post("/sales")]
async fn create_sale(
    body: Json<CreateSaleSchema>,  // Agora com o campo client_id se necessário
    db_pool: Data<PgPool>,
) -> impl Responder {
    let query = r#"
        INSERT INTO sales (client_id, product_id, quantity, total)
        VALUES ($1, $2, $3, $4)
        RETURNING id, client_id, product_id, quantity, total
    "#;

    match sqlx::query_as::<_, SaleModel>(query)
        .bind(&body.client_id)  // Remova isso se client_id não for relevante
        .bind(&body.product_id)
        .bind(&body.quantity)
        .bind(&body.total)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(sale) => HttpResponse::Ok().json(json!({
            "status": "sucesso",
            "sale": sale
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "erro",
            "mensagem": "Falha ao criar sale"
        })),
    }
}

#[get("/sales")]
async fn get_all_sales(db_pool: Data<PgPool>) -> impl Responder {
    let query = "SELECT * FROM sales";
    
    match sqlx::query_as::<_, SaleModel>(query).fetch_all(&**db_pool).await {
        Ok(sales) => HttpResponse::Ok().json(json!({"status": "success", "sales": sales})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch sales"})),
    }
}

#[get("/sales/{id}")]
async fn get_sale_by_id(path: Path<Uuid>, db_pool: Data<PgPool>) -> impl Responder {
    let sale_id = path.into_inner();
    let query = "SELECT * FROM sales WHERE id = $1";
    
    match sqlx::query_as::<_, SaleModel>(query)
        .bind(sale_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(sale) => HttpResponse::Ok().json(json!({"status": "success", "sale": sale})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Sale not found"})),
    }
}

#[patch("/sales/{id}")]
async fn update_sale_by_id(
    path: Path<Uuid>,
    body: Json<UpdateSaleSchema>,
    db_pool: Data<PgPool>,
) -> impl Responder {
    let sale_id = path.into_inner();

    let query = r#"
        UPDATE sales
        SET client_id = COALESCE($1, client_id),
            product_id = COALESCE($2, product_id),
            quantity = COALESCE($3, quantity),
            total = COALESCE($4, total)
        WHERE id = $5
        RETURNING *;
    "#;

    match sqlx::query_as::<_, SaleModel>(query)
        .bind(body.client_id.as_ref())  // Usa `Option<Uuid>`
        .bind(body.product_id)
        .bind(body.quantity)
        .bind(body.total.as_ref())  // Usa `Option<f64>`
        .bind(sale_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(updated_sale) => HttpResponse::Ok().json(updated_sale),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/sales/{id}")]
async fn delete_sale_by_id(path: Path<Uuid>, db_pool: Data<PgPool>) -> impl Responder {
    let sale_id = path.into_inner();
    let query = "DELETE FROM sales WHERE id = $1";

    match sqlx::query(query)
        .bind(sale_id)
        .execute(&**db_pool)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "sale deleted"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete sale"})),
    }
}

pub fn config_sales(cfg: &mut ServiceConfig) {
    cfg.service(create_sale)
        .service(get_all_sales)
        .service(get_sale_by_id)
        .service(update_sale_by_id)
        .service(delete_sale_by_id);
}
