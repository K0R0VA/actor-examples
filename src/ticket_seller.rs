use crate::models::{Ticket, Event};
use actix::{Actor, Context};

pub struct TicketSeller {
    pub(crate) event_name: String,
    pub(crate) tickets: Vec<Ticket>
}

impl Actor for TicketSeller {
    type Context = Context<Self>;
}

pub mod messages {
    use crate::models::{Ticket};
    use actix::{Message, Actor};
    use actix::dev::{MessageResponse, OneshotSender};

    pub struct Add {
        pub tickets: Vec<Ticket>
    }
    impl Message for Add {
        type Result = ();
    }
    pub struct Buy {
        pub tickets_count: usize
    }
    pub struct Tickets {
        pub event: String,
        pub entries: Option<Vec<Ticket>>
    }
    impl Message for Buy {
        type Result = Tickets;
    }

    impl<A, M> MessageResponse<A, M> for Tickets
        where
            A: Actor,
            M: Message<Result = Tickets>,
    {
        fn handle(self, ctx: &mut <A as Actor>::Context, tx: Option<OneshotSender<Tickets>>) {
            if let Some(tx) = tx {
                tx.send(self);
            }
        }
    }

    pub struct GetEvent;

    impl Message for GetEvent {
        type Result = String;
    }

    pub struct Cancel;
}

pub mod receive {
    use crate::ticket_seller::TicketSeller;
    use crate::ticket_seller::messages::{Add, Buy, Tickets, GetEvent};
    use actix::Handler;

    impl Handler<Add> for TicketSeller {
        type Result = ();

        fn handle(&mut self, mut msg: Add, _ctx: &mut Self::Context) -> Self::Result {
            self.tickets.append(&mut msg.tickets);
        }
    }
    impl Handler<Buy> for TicketSeller {

        type Result = Tickets;

        fn handle(&mut self, msg: Buy, _ctx: &mut Self::Context) -> Self::Result {
            if self.tickets.len() >= msg.tickets_count {
                let entries = Some(self.tickets.drain(.. msg.tickets_count).collect());
                Tickets {event: self.event_name.clone(), entries}
            }
            else {
                Tickets {event: self.event_name.clone(), entries: None}
            }
        }
    }
    impl Handler<GetEvent> for TicketSeller {
        type Result = String;

        fn handle(&mut self, msg: GetEvent, ctx: &mut Self::Context) -> Self::Result {
            self.event_name.clone()
        }
    }

}