use crate::models::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDTO {
    pub id: i32,
    pub issuer: String,
    pub message: String,
}

impl From<Message> for MessageDTO {
    fn from(message: Message) -> Self {
        todo!()
    }
}
