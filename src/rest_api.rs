use actix_web::web::ServiceConfig;
use actix::{System, Actor};
use crate::box_office::BoxOffice;
use crate::rest_api::routes::{create_event, buy_tickets};
use std::collections::HashMap;

pub fn configuration(config: &mut ServiceConfig) {
    config
        .service(create_event)
        .service(buy_tickets);
}

pub mod routes {
    use actix::{Addr, MailboxError};
    use actix_web::web::{Data, Path, Json};
    use crate::box_office::BoxOffice;
    use actix_web::{Responder, HttpResponse};
    use crate::box_office::messages::{CreateEvent, GetTickets};
    use actix_web::post;
    use serde::Serialize;
    use actix_web::http::StatusCode;
    use crate::maybe::Maybe;
    use crate::ticket_seller::messages::Tickets;

    #[post("event")]
    pub async fn create_event(box_office: Data<Addr<BoxOffice>>, msg: Json<CreateEvent>) -> impl Responder {
        let response = box_office.send(msg.into_inner()).await;
        match response {
            Ok(result) => {
                match result {
                    Ok(event_created) => HttpResponse::Created().body(format!("Event {} was created", event_created.event)),
                    Err(exist) => HttpResponse::BadRequest().body("Event already exist")
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("something crash")
        }
    }

    #[post("buy")]
    pub async fn buy_tickets(box_office: Data<Addr<BoxOffice>>, msg: Json<GetTickets>) -> impl Responder {
        let response = box_office.send(msg.into_inner()).await;
        match response {
            Err(err) => HttpResponse::InternalServerError().finish(),
            Ok(maybe) => match maybe {
                Maybe::None => HttpResponse::BadRequest().finish(),
                Maybe::Some(result) => match result {
                    Ok(tickets) => HttpResponse::Ok().json(tickets),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
        }
    }
}