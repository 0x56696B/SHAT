use serde::{Deserialize, Serialize};

use crate::models::pure::chat::message::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatDTO {
    pub id: i32,
    pub people: Vec<String>,
    pub messages: Vec<Message>,
}
