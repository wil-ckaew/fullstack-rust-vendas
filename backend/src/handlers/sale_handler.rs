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
use sqlx::PgPool;

#[post("/sales")]
async fn create_sale(body: Json<CreateSaleSchema>, db_pool: Data<PgPool>) -> impl Responder {
    let query = r#"
        INSERT INTO sales (product_id, quantity, total)
        VALUES ($1, $2, $3)
        RETURNING id, product_id, quantity, total, sale_date;
    "#;

    match sqlx::query_as::<_, SaleModel>(query)
        .bind(body.product_id)
        .bind(body.quantity)
        .bind(body.total)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(sale) => HttpResponse::Ok().json(json!({ "status": "success", "sale": sale })),
        Err(_) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": "Failed to create sale" })),
    }
}

#[get("/sales")]
pub async fn get_all_sales(
    opts: Query<FilterOptions>, 
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        SaleModel, 
        "SELECT * FROM sales ORDER BY id LIMIT $1 OFFSET $2", 
        limit as i32, 
        offset as i32
    )
        .fetch_all(&data.db)
        .await
    {
        Ok(sales) => {
            HttpResponse::Ok().json(json!({
                "status": "success", 
                "sales": sales
            }))
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get sales: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/sales/{id}")]
async fn get_sale_by_id(path: Path<Uuid>, db_pool: Data<PgPool>) -> impl Responder {
    let sale_id = path.into_inner();
    let query = r#"SELECT * FROM sales WHERE id = $1;"#;

    match sqlx::query_as::<_, SaleModel>(query)
        .bind(sale_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(sale) => HttpResponse::Ok().json(json!({ "status": "success", "sale": sale })),
        Err(_) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": "Sale not found" })),
    }
}

#[patch("/sales/{id}")]
async fn update_sale_by_id(
    path: Path<Uuid>,
    body: Json<UpdateSaleSchema>,
    db_pool: Data<PgPool>
) -> impl Responder {
    let sale_id = path.into_inner();
    let query = r#"
        UPDATE sales
        SET product_id = COALESCE($1, product_id), 
            quantity = COALESCE($2, quantity), 
            total = COALESCE($3, total)
        WHERE id = $4
        RETURNING *;
    "#;

    match sqlx::query_as::<_, SaleModel>(query)
        .bind(body.product_id)
        .bind(body.quantity)
        .bind(body.total)
        .bind(sale_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(updated_sale) => HttpResponse::Ok().json(json!({ "status": "success", "sale": updated_sale })),
        Err(_) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": "Failed to update sale" })),
    }
}

#[delete("/sales/{id}")]
async fn delete_sale_by_id(path: Path<Uuid>, db_pool: Data<PgPool>) -> impl Responder {
    let sale_id = path.into_inner();
    let query = r#"DELETE FROM sales WHERE id = $1;"#;

    match sqlx::query(query)
        .bind(sale_id)
        .execute(&**db_pool)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "success", "message": "Sale deleted" })),
        Err(_) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": "Failed to delete sale" })),
    }
}

pub fn config_sales(cfg: &mut ServiceConfig) {
    cfg.service(create_sale)
        .service(get_all_sales)
        .service(get_sale_by_id)
        .service(update_sale_by_id)
        .service(delete_sale_by_id);
}
