pub mod client;
pub mod clients;
pub mod product;
pub mod sale;

use actix_web::web::ServiceConfig;

pub fn config(conf: &mut ServiceConfig) {
    conf.service(
        actix_web::web::scope("/api")
            .configure(health::config_health)
            .configure(client::config_client)
            .configure(clients::config_clients)
            .configure(product::config_products)
            .configure(sale::config_sales)
            .configure(logs::config_logs)
    );
}

