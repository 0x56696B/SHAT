use actix_web::{Error, HttpResponse};
use diesel::PgConnection;

use crate::models::message::Message;

pub async fn insert_message(
    _message_struct: Message,
    connection: PgConnection,
) -> Result<HttpResponse, Error> {
    todo!("Implement insert_into_db")
}

// pub async fn query_chat(chat: Chat) -> Result<HttpResponse, Error> {
//     todo!("Implement query_from_db")
// }
