pub mod clients;
pub mod product_handler;
pub mod sale_handler;

use actix_web::web::ServiceConfig;

pub fn config(conf: &mut ServiceConfig) {
    conf.service(
        actix_web::web::scope("/api")
            .configure(clients::config_clients)
            .configure(product_handler::config_products)
            .configure(sale_handler::config_sales)
    );
}
