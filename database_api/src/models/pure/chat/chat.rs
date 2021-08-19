use super::message::Message;
use crate::models::pure::person::person::Person;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: i32,
    pub people: Vec<Person>,
    pub messages: Vec<Message>,
}
