use actix::{Actor, Context, Addr};
use crate::models::Event;
use crate::ticket_seller::TicketSeller;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

pub struct BoxOffice {
    pub(crate) events: HashMap<String, Addr<TicketSeller>>
}

impl Default for BoxOffice {
    fn default() -> Self {
        BoxOffice {
            events: HashMap::new()
        }
    }
}

impl Actor for BoxOffice {
    type Context = Context<Self>;
}

pub mod messages {
    use crate::models::Event;
    use actix::{Message, MailboxError, Response};
    use crate::ticket_seller::messages::{Tickets, Buy};
    use actix::dev::Request;
    use crate::ticket_seller::TicketSeller;
    use serde::{Serialize, Deserialize};
    use crate::maybe::Maybe;

    #[derive(Serialize, Deserialize)]
    pub struct CreateEvent {
        pub name: String,
        pub tickets_count: usize,
    }

    impl Message for CreateEvent {
        type Result = Result<EventCreated, EventExists>;
    }

    pub struct EventCreated {
        pub event: String
    }

    pub struct EventExists;

    #[derive(Serialize, Deserialize)]
    pub struct GetEvent {
        pub name: String
    }

    pub struct GetEvents;

    #[derive(Serialize, Deserialize)]
    pub struct GetTickets {
        pub event_name: String,
        pub tickets_count: usize,
    }

    impl Message for GetTickets {
        type Result = Maybe<Result<Tickets, MailboxError>>;
    }

    pub struct CancelEvent {
        event_name: String
    }
}

pub mod receive {
    use crate::box_office::messages::{CreateEvent, EventCreated, EventExists, GetTickets};
    use crate::box_office::BoxOffice;
    use crate::ticket_seller::TicketSeller;
    use crate::models::Ticket;
    use crate::ticket_seller::messages::{Tickets, Buy};
    use actix::{Handler, Actor, Context, MailboxError, Response, Addr};
    use actix::dev::MessageResponse;
    use crate::maybe::Maybe;

    impl Handler<CreateEvent> for BoxOffice {
        type Result = Result<EventCreated, EventExists>;

        fn handle(&mut self, msg: CreateEvent, _ctx: &mut Self::Context) -> Self::Result {
            if self.events.contains_key(&*msg.name) {
                Err(EventExists)
            } else {
                let event_tickets = TicketSeller::create(|ctx: &mut Context<TicketSeller>| {
                    TicketSeller {
                        event_name: msg.name.clone(),
                        tickets: (0..msg.tickets_count).map(|i| Ticket { id: i }).collect(),
                    }
                });
                self.events.insert(msg.name.clone(), event_tickets);
                Ok(EventCreated { event: msg.name })
            }
        }
    }
    impl Handler<GetTickets> for BoxOffice {
        type Result = Response<Maybe<Result<Tickets, MailboxError>>>;

        fn handle(&mut self, msg: GetTickets, ctx: &mut Self::Context) -> Self::Result {
            match self.events.get(&*msg.event_name) {
                None => Response::reply(Maybe::None),
                Some(actor) => {
                    let request = actor.send(Buy { tickets_count: msg.tickets_count });
                    Response::fut(Maybe::Some(request))
                }
            }
        }
    }
}

