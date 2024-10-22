use actix_web::{
    get, post, delete, patch, web::{Data, Json, scope, Query, Path, ServiceConfig}, HttpResponse, Responder
};
use serde_json::json;
use crate::{
    model::ClientModel,
    schema::{CreateClientSchema, UpdateClientSchema, FilterOptions},
    AppState
};
use sqlx::PgPool;
use uuid::Uuid;

#[post("/clients")]
async fn create_clients(
    body: Json<CreateClientchema>,
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
                "message": format!("Failed to create client: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/clients")]
pub async fn get_all_clients(
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
        Ok(clients) => {
            let response = json!({
                "status": "success",
                "clients": clients
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get clients: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/clients/{id}")]
async fn get_client_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let client_id = path.into_inner();

    match sqlx::query_as!(
        ClientModel,
        "SELECT * FROM clients WHERE id = $1",
        client_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(client) => {
            let response = json!({
                "status": "success",
                "client": client
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get client: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[patch("/clients/{id}")]
async fn update_client_by_id(
    path: Path<Uuid>,
    body: Json<UpdateClientchema>,
    data: Data<AppState>
) -> impl Responder {
    let client_id = path.into_inner();

    match sqlx::query_as!(ClientModel, "SELECT * FROM clients WHERE id = $1", client_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(client) => {
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
                Ok(updated_client) => {
                    let response = json!({
                        "status": "success",
                        "client": updated_client
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Failed to update client: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!({
                "status": "error",
                "message": format!("Client not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
    }
}

#[delete("/clients/{id}")]
async fn delete_client_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let client_id = path.into_inner();

    match sqlx::query!("DELETE FROM clients WHERE id = $1", client_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to delete client: {:?}", err)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Configuração das rotas para tarefas
pub fn config_client(conf: &mut ServiceConfig) {
    conf.service(create_clients)
       .service(get_all_clients)
       .service(get_client_by_id)
       .service(update_client_by_id)
       .service(delete_client_by_id);
}