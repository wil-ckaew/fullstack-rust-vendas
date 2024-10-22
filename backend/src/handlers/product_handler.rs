// src/handlers/product_handler.rs
use actix_web::{
    get, post, delete, patch,
    web::{Data, Json, Path, ServiceConfig, Query},
    HttpResponse, Responder,
};
use serde_json::json;
use crate::{
    models::product::{CreateProductSchema, UpdateProductSchema, ProductModel, FilterOptions}, 
    AppState
};
use uuid::Uuid;
use sqlx::PgPool;

#[post("/products")]
async fn create_product(
    body: Json<CreateProductSchema>,
    db_pool: Data<PgPool>,
) -> impl Responder {
    let query = r#"
        INSERT INTO products (name, price, quantity, description, stock)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;
    "#;

    match sqlx::query_as::<_, ProductModel>(query)
        .bind(&body.name)
        .bind(body.price)
        .bind(body.quantity)
        .bind(&body.description) // Agora deve funcionar
        .bind(body.stock)         // Agora deve funcionar
        .fetch_one(&**db_pool)
        .await
    {
        Ok(new_product) => HttpResponse::Created().json(new_product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/products")]
pub async fn get_all_products(
    opts: Query<FilterOptions>, 
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        ProductModel, 
        "SELECT * FROM products ORDER BY id LIMIT $1 OFFSET $2", 
        limit as i32, 
        offset as i32
    )
        .fetch_all(&data.db)
        .await
    {
        Ok(products) => {
            HttpResponse::Ok().json(json!({
                "status": "success", 
                "products": products
            }))
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get products: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

/*
#[get("/products")]
async fn get_all_products(db_pool: Data<PgPool>) -> impl Responder {
    let query = "SELECT * FROM products";
    
    match sqlx::query_as::<_, ProductModel>(query).fetch_all(&**db_pool).await {
        Ok(products) => HttpResponse::Ok().json(json!({"status": "success", "products": products})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch products"})),
    }
}
*/

#[get("/products/{id}")]
async fn get_product_by_id(path: Path<Uuid>, db_pool: Data<PgPool>) -> impl Responder {
    let product_id = path.into_inner();
    let query = "SELECT * FROM products WHERE id = $1";
    
    match sqlx::query_as::<_, ProductModel>(query)
        .bind(product_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(product) => HttpResponse::Ok().json(json!({"status": "success", "product": product})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Product not found"})),
    }
}

#[patch("/products/{id}")]
async fn update_product_by_id(
    path: Path<Uuid>,
    body: Json<UpdateProductSchema>,
    db_pool: Data<PgPool>,
) -> impl Responder {
    let product_id = path.into_inner();

    // Lógica de atualização
    let query = r#"
        UPDATE products
        SET name = COALESCE($1, name),
            price = COALESCE($2, price),
            quantity = COALESCE($3, quantity),
            description = COALESCE($4, description),
            stock = COALESCE($5, stock)
        WHERE id = $6
        RETURNING *;
    "#;

    match sqlx::query_as::<_, ProductModel>(query)
        .bind(body.name.as_ref())
        .bind(body.price)
        .bind(body.quantity)
        .bind(body.description.as_ref())
        .bind(body.stock)
        .bind(product_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(updated_product) => HttpResponse::Ok().json(updated_product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/products/{id}")]
async fn delete_product_by_id(path: Path<Uuid>, db_pool: Data<PgPool>) -> impl Responder {
    let product_id = path.into_inner();
    let query = "DELETE FROM products WHERE id = $1";

    match sqlx::query(query)
        .bind(product_id)
        .execute(&**db_pool)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Product deleted"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete product"})),
    }
}

pub fn config_products(cfg: &mut ServiceConfig) {
    cfg.service(create_product)
        .service(get_all_products)
        .service(get_product_by_id)
        .service(update_product_by_id)
        .service(delete_product_by_id);
}
