use actix_web::{
    get, post, delete, patch,
    web::{Data, Json, Path, ServiceConfig, Query},
    HttpResponse, Responder,
};
use serde_json::json;
use crate::{
    models::client::{CreateClientSchema, UpdateClientSchema, ClientModel, FilterOptions}, 
    AppState
};
use uuid::Uuid;
use sqlx::PgPool; // Supondo que isso seja necessário

#[post("/clientes")]
async fn create_client(
    body: Json<CreateClientSchema>,
    db_pool: Data<PgPool>,
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
        .fetch_one(&**db_pool)
        .await
    {
        Ok(cliente) => HttpResponse::Ok().json(json!({
            "status": "sucesso",
            "cliente": cliente
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "erro",
            "mensagem": "Falha ao criar cliente"
        })),
    }
}

#[get("/clientes")]
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
            HttpResponse::Ok().json(json!({
                "status": "success", 
                "clients": clients
            }))
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

/*
#[get("/clientes")]
async fn get_all_clients(db_pool: Data<PgPool>) -> impl Responder {
    let query = "SELECT * FROM clients";
    
    match sqlx::query_as::<_, ClientModel>(query).fetch_all(&**db_pool).await {
        Ok(clientes) => HttpResponse::Ok().json(json!({
            "status": "sucesso",
            "clientes": clientes
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "erro",
            "mensagem": "Falha ao buscar clientes"
        })),
    }
}
*/
#[get("/clientes/{id}")]
async fn get_client_by_id(
    path: Path<Uuid>, 
    db_pool: Data<PgPool>
) -> impl Responder {
    let client_id = path.into_inner();
    let query = "SELECT * FROM clients WHERE id = $1";
    
    match sqlx::query_as::<_, ClientModel>(query).bind(client_id).fetch_one(&**db_pool).await {
        Ok(cliente) => HttpResponse::Ok().json(json!({
            "status": "sucesso",
            "cliente": cliente
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "erro",
            "mensagem": "Cliente não encontrado"
        })),
    }
}

#[patch("/clients/{id}")]
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
                        "parent": updated_client
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
/*
#[patch("/clientes/{id}")]
async fn update_client_by_id(
    path: Path<Uuid>,
    body: Json<UpdateClientSchema>,
    db_pool: Data<PgPool>,
) -> impl Responder {
    let client_id = path.into_inner();
    let query = r#"
        UPDATE clients SET 
            name = COALESCE($1, name), 
            email = COALESCE($2, email), 
            phone = COALESCE($3, phone)
        WHERE id = $4
        RETURNING *
    "#;

    match sqlx::query_as::<_, ClientModel>(query)
        .bind(&body.name)
        .bind(&body.email)
        .bind(&body.phone)
        .bind(client_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(updated_client) => HttpResponse::Ok().json(json!({
            "status": "sucesso",
            "cliente": updated_client
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "erro",
            "mensagem": "Falha ao atualizar o cliente"
        })),
    }
}
*/
#[delete("/clientes/{id}")]
async fn delete_client_by_id(path: Path<Uuid>, db_pool: Data<PgPool>) -> impl Responder {
    let client_id = path.into_inner();
    let query = "DELETE FROM clients WHERE id = $1";
    
    match sqlx::query(query).bind(client_id).execute(&**db_pool).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "sucesso",
            "mensagem": "Cliente excluído"
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "erro",
            "mensagem": "Falha ao excluir o cliente"
        })),
    }
}

pub fn config_clients(cfg: &mut ServiceConfig) {
    cfg.service(create_client)
        .service(get_all_clients)
        .service(get_client_by_id)
        .service(update_client_by_id)
        .service(delete_client_by_id);
}
