use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: usize
}

#[derive(Clone)]
pub struct Event {
    pub name: String
}