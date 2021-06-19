use crate::box_office::BoxOffice;
use crate::rest_api::routes::{buy_tickets, create_event};
use actix::{Actor, System};
use actix_web::web::ServiceConfig;
use std::collections::HashMap;

use self::routes::hello_world;

pub fn configuration(config: &mut ServiceConfig) {
    config
        .service(create_event)
        .service(buy_tickets)
        .service(hello_world);
}

pub mod routes {
    use crate::box_office::messages::{CreateEvent, GetTickets};
    use crate::box_office::BoxOffice;
    use crate::database::{Database, Hello};
    use crate::maybe::Maybe;
    use crate::ticket_seller::messages::Tickets;
    use actix::{Addr, MailboxError};
    use actix_web::http::StatusCode;
    use actix_web::web::{Data, Json, Path};
    use actix_web::{get, post};
    use actix_web::{HttpResponse, Responder};
    use serde::Serialize;

    #[get("hello-world")]
    pub async fn hello_world(database: Data<Addr<Database>>) -> impl Responder {
        database.send(Hello).await.unwrap().unwrap()
    }

    #[post("event")]
    pub async fn create_event(
        box_office: Data<Addr<BoxOffice>>,
        msg: Json<CreateEvent>,
    ) -> impl Responder {
        let response = box_office.send(msg.into_inner()).await;
        match response {
            Ok(result) => match result {
                Ok(event_created) => HttpResponse::Created()
                    .body(format!("Event {} was created", event_created.event)),
                Err(exist) => HttpResponse::BadRequest().body("Event already exist"),
            },
            Err(_) => HttpResponse::InternalServerError().body("something crash"),
        }
    }

    #[post("buy")]
    pub async fn buy_tickets(
        box_office: Data<Addr<BoxOffice>>,
        msg: Json<GetTickets>,
    ) -> impl Responder {
        let response = box_office.send(msg.into_inner()).await;
        match response {
            Err(err) => HttpResponse::InternalServerError().finish(),
            Ok(maybe) => match maybe {
                Maybe::None => HttpResponse::BadRequest().finish(),
                Maybe::Some(result) => match result {
                    Ok(tickets) => HttpResponse::Ok().json(tickets),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                },
            },
        }
    }
}
