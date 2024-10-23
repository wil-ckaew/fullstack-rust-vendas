pub mod clients;
pub mod product_handler;
pub mod sales;

use actix_web::web::ServiceConfig;

pub fn config(conf: &mut ServiceConfig) {
    conf.service(
        actix_web::web::scope("/api")
            .configure(clients::config_clients)
            .configure(product_handler::config_products)
            .configure(sales::config_sales)
    );
}
