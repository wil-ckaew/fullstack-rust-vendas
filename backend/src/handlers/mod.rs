pub mod client_handler;
pub mod product_handler;
pub mod sale_handler;
pub mod clients;

use actix_web::web::ServiceConfig;

pub fn config(conf: &mut ServiceConfig) {
    conf.service(
        actix_web::web::scope("/api")
            .configure(clientes::config_client)
            .configure(client_handler::config_clients)
            .configure(product_handler::config_products)
            .configure(sale_handler::config_sales)
    );
}
