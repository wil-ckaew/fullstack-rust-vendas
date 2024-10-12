use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;
mod ai_service;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Carrega variáveis de ambiente
    dotenv().ok();

    // Recupera a URL do banco de dados
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Configura o pool de conexões do banco de dados
    let db_pool = PgPoolOptions::new()
        .max_connections(5)  // Define o limite de conexões conforme necessário
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Inicia o servidor HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db_pool.clone() })) // Compartilha o pool de banco de dados
            .configure(handlers::config) // Registra rotas e serviços
            .wrap(Logger::default()) // Habilita log de requisições
            .wrap(
                Cors::default()
                    .allow_any_origin() // Permite requisições de qualquer origem (dev)
                    .allow_any_method() // Permite qualquer método HTTP
                    .allow_any_header() // Permite quaisquer cabeçalhos
            )
            // Servir arquivos estáticos, se necessário
            .service(actix_files::Files::new("/uploads", "./uploads").show_files_listing()) // Servir diretório de uploads
            .service(actix_files::Files::new("/static", "./static").show_files_listing()) // Servir diretório de arquivos estáticos
    })
    .bind("127.0.0.1:8080")? // Bind do servidor na porta 8080
    .run()
    .await
}
