mod ticket_seller;
mod models;
mod box_office;
mod rest_api;

use actix_web::{get, web, App, HttpServer, Responder};
use crate::rest_api::configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configuration))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
