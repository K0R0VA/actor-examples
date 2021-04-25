mod ticket_seller;
mod models;
mod box_office;
mod rest_api;
mod maybe;

use actix_web::{get, web, App, HttpServer, Responder};
use crate::rest_api::configuration;
use crate::box_office::BoxOffice;
use std::collections::HashMap;
use actix::{Actor, System};
use crate::ticket_seller::TicketSeller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let box_office= BoxOffice { events: Default::default() }.start();
    HttpServer::new(move || App::new().data(box_office.clone()).configure(configuration))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
