use super::{message::Message, person::Person};
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Chat {
    pub id: i32,
    pub people: Vec<Person>,
    pub messages: Vec<Message>,
}
