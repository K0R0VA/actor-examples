mod box_office;
mod database;
mod maybe;
mod models;
mod rest_api;
mod ticket_seller;
use crate::box_office::BoxOffice;
use crate::rest_api::configuration;
use actix::Actor;
use actix_web::{App, HttpServer};
use database::{get_config, Database};
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let box_office = BoxOffice {
        events: Default::default(),
    }
    .start();
    let pool = get_config().create_pool(NoTls).unwrap();
    let database = Database::new(pool);
    HttpServer::new(move || {
        App::new()
            .data(box_office.clone())
            .data(database.clone())
            .configure(configuration)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
