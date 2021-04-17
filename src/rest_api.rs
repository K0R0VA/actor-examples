use actix_web::web::ServiceConfig;
use actix::{System, Actor};
use crate::box_office::BoxOffice;
use crate::rest_api::routes::create_event;

pub fn configuration(config: &mut ServiceConfig) {
    let box_office = BoxOffice::default().start();
    config
        .data(box_office)
        .service(create_event);
}

pub mod routes {
    use actix::Addr;
    use actix_web::web::{Data, Path};
    use crate::box_office::BoxOffice;
    use actix_web::{Responder, HttpResponse};
    use crate::box_office::messages::{CreateEvent, GetTickets};
    use actix_web::post;

    #[post("event/{name}/tickets/{count}")]
    pub async fn create_event(box_office: Data<Addr<BoxOffice>>, Path((name, count)): Path<(String, usize)>) -> impl Responder {
        let response = box_office.send(CreateEvent {name, tickets_count: count}).await;
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

    #[post("buy/{event}/tickets/{count}")]
    pub async fn buy_tickets(box_office: Data<Addr<BoxOffice>>, Path((event, count)): Path<(String, usize)>) -> impl Responder {
        let response = box_office.send(GetTickets { event_name: event, tickets_count: count }).await;
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
}