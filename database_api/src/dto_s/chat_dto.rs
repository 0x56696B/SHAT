use crate::models::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatDTO {
    pub id: i32,
    pub people: Vec<String>,
    pub messages: Vec<Message>,
}
