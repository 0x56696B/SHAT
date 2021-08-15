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
    fn from(message_dto: MessageDTO) -> Self {
        Message {
            id: message_dto.id,
            issuer: Person {
                id: message_dto.issuer_id,
                username: message_dto.issuer,
            },
            message: message_dto.message,
        }
    }
}
