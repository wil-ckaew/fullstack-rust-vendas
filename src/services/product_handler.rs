// src/handlers/product_handler.rs
use actix_web::{
    get, post, delete, patch,
    web::{Data, Json, Path, ServiceConfig, Query},
    HttpResponse, Responder,
};
use serde_json::json;
use crate::{
    //models::product::ProductModel,
    models::product::{ProductModel, CreateProductSchema, UpdateProductSchema, FilterOptions},
    AppState
};
use uuid::Uuid;
use sqlx::PgPool; // Supondo que isso seja necessário
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive; // Import FromPrimitive for from_f64

#[post("/products")]
async fn create_product(
    body: Json<CreateProductSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO products (name, description, price, stock_quantity)
        VALUES ($1, $2, $3, $4)
        RETURNING *;
    "#;

    match sqlx::query_as::<_, ProductModel>(query)
        .bind(&body.name)
        .bind(&body.description)
        .bind(body.price) // Usa f64 diretamente
        .bind(body.stock_quantity.unwrap_or(0)) // Usa stock_quantity corretamente
        .fetch_one(&data.db)
        .await
    {
        Ok(product) => {
            let response = json!( {
                "status": "success",
                "product": {
                    "id": product.id,
                    "name": product.name,
                    "description": product.description,
                    "price": product.price, // Isso agora é f64
                    "stock_quantity": product.stock_quantity
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!( {
                "status": "error",
                "message": format!("Failed to create product: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/products")]
pub async fn get_all_products(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // Replace `stock_quantity` with `stock`
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
            let response = json!({
                "status": "success",
                "products": products
            });
            HttpResponse::Ok().json(response)
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
    data: Data<AppState>, // Adicione o `data` para acessar o banco de dados
) -> impl Responder {
    let product_id = path.into_inner(); // Extrai o `product_id` da URL

    match sqlx::query_as!(ProductModel, "SELECT * FROM products WHERE id = $1", product_id) // Corrigido para `ProductModel`
        .fetch_one(&data.db) // Usa `data.db` para acessar o banco de dados
        .await
    {
        Ok(product) => {
            let update_result = sqlx::query_as!(
                ProductModel,
                "UPDATE products SET name = COALESCE($1, name), price = COALESCE($2, price), description = COALESCE($3, description), stock_quantity = COALESCE($4, stock_quantity) WHERE id = $5 RETURNING *",
                body.name.as_ref(),
                body.price.as_ref(),
                body.description.as_ref(),
                body.stock_quantity.as_ref(),
                product_id
            )
            .fetch_one(&data.db) // Usa `data.db` para acessar o banco de dados
            .await;

            match update_result {
                Ok(updated_product) => {
                    let response = json!({
                        "status": "success",
                        "product": updated_product
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Failed to update product: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!({
                "status": "error",
                "message": format!("Product not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
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

pub fn config_products(conf: &mut ServiceConfig) {
    conf.service(create_product)
        .service(get_all_products)
        .service(get_product_by_id)
        .service(update_product_by_id)
        .service(delete_product_by_id);
}
