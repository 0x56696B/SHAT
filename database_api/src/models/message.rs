use diesel::Queryable;
use serde::{Deserialize, Serialize};
use std::convert::From;

use super::person::Person;
use crate::dto_s::message_dto::MessageDTO;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Message {
    pub id: i32,
    pub issuer: Person,
    pub message: String,
}

impl From<MessageDTO> for Message {
    fn from(messageDTO: MessageDTO) -> Self {
        todo!()
    }
}
