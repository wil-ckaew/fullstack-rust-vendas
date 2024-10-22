use actix_web::{
    get, post, delete, patch, web::{Data, Json, scope, Query, Path, ServiceConfig}, HttpResponse, Responder
};
use serde_json::json;
use crate::{
    models::client::ClientModel, 
    models::schema::{CreateClientSchema, UpdateClientSchema, FilterOptions},
    AppState
};
use sqlx::PgPool;
use uuid::Uuid;

#[post("/clientes")]
async fn create_client(
    body: Json<CreateClientSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO clients (name, email, phone)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, phone
    "#;

    match sqlx::query_as::<_, ClientModel>(query)
        .bind(&body.name)
        .bind(&body.email)
        .bind(&body.phone)
        .fetch_one(&data.db)
        .await
    {
        Ok(client) => {
            let response = json!({
                "status": "success",
                "client": {
                    "id": client.id,
                    "name": client.name,
                    "email": client.email,
                    "phone": client.phone,
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create clientes: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/clientes")]
async fn get_all_clients(
    opts: Query<FilterOptions>, 
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        ClientModel, 
        "SELECT * FROM clients ORDER BY id LIMIT $1 OFFSET $2", 
        limit as i32, 
        offset as i32
    )
        .fetch_all(&data.db)
        .await
    {
        Ok(clients) => HttpResponse::Ok().json(json!({
            "status": "success", 
            "clients": clients
        })),
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to get clients: {:?}", error)
        })),
    }
}

#[get("/clientes/{id}")]
async fn get_client_by_id(
    path: Path<Uuid>, 
    data: Data<AppState>
) -> impl Responder {
    let client_id = path.into_inner();
    let query = "SELECT * FROM clients WHERE id = $1";
    
    match sqlx::query_as::<_, ClientModel>(query)
        .bind(client_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(client) => HttpResponse::Ok().json(json!({
            "status": "success",
            "client": client
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Client not found"
        })),
    }
}

#[patch("/clientes/{id}")]
async fn update_client_by_id(
    path: Path<Uuid>,
    body: Json<UpdateClientSchema>,
    data: Data<AppState>
) -> impl Responder {
    let client_id = path.into_inner();

    match sqlx::query_as!(ClientModel, "SELECT * FROM clients WHERE id = $1", client_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(_) => {
            let update_result = sqlx::query_as!(
                ClientModel,
                "UPDATE clients SET name = COALESCE($1, name), email = COALESCE($2, email), phone = COALESCE($3, phone) WHERE id = $4 RETURNING *",
                body.name.as_ref(),
                body.email.as_ref(),
                body.phone.as_ref(),
                client_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_client) => HttpResponse::Ok().json(json!({
                    "status": "success",
                    "client": updated_client
                })),
                Err(error) => HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Failed to update client: {:?}", error)
                })),
            }
        }
        Err(fetch_error) => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": format!("Client not found: {:?}", fetch_error)
        })),
    }
}

#[delete("/clientes/{id}")]
async fn delete_client_by_id(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let client_id = path.into_inner();
    let query = "DELETE FROM clients WHERE id = $1";
    
    match sqlx::query(query)
        .bind(client_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Client deleted"
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to delete client"
        })),
    }
}

pub fn config_clients(conf: &mut ServiceConfig) {
    conf.service(create_client)
        .service(get_all_clients)
        .service(get_client_by_id)
        .service(update_client_by_id)
        .service(delete_client_by_id);
}
