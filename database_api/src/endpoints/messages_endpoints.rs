use actix_web::{get, post, web::Json, HttpRequest, HttpResponse};

use crate::{
    database::db_queries,
    models::{dto_s::chat::message_dto::MessageDTO, pure::chat::message::Message},
};

#[post("/send_message")]
pub async fn send_message(message: Json<MessageDTO>, req: HttpRequest) -> HttpResponse {
    let msg: Message = message.0.into();

    //TODO: Query person info and populate Message struct

    //Database query
    let result = db_queries::insert_message(msg, req).await;

    match result {
        Ok(success) => success,
        Err(err) => err,
    }
}

#[get("/chat")]
async fn get_chat(user_one: String, user_two: String, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(format!("{} messages to {}", user_one, user_two))

    // db_layer::query_chat(message.0).await
}
