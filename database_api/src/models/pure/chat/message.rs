use serde::{Deserialize, Serialize};
use std::convert::From;

use crate::models::{
    dto_s::chat::message_dto::MessageDTO,
    pure::person::person::{hash_pass, Person},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub issuer: Person,
    pub message: String,
}

impl From<MessageDTO> for Message {
    fn from(message_dto: MessageDTO) -> Self {
        todo!("Implement casting from Message to MEssageDTO")
    }
}
