use actix_web::{post, web::Json, HttpRequest, HttpResponse};

use crate::{db_access::db_queries, dto_s::message_dto::MessageDTO, models::message::Message};

#[post("/send_message")]
async fn send_message(message: Json<MessageDTO>, _req: HttpRequest) -> HttpResponse {
    let msg: Message = message.0.into();

    let result = db_queries::insert_into_db(msg).await;
    match result {
        Ok(responce) => responce,
        Err(err) => todo!(),
    }
}

// #[get("/chat")]
// async fn chat(message: Json<Message>, _req: HttpRequest) -> Result<HttpResponse, Error> {
//     db_layer::insert_into_db(message.0).await
// }
