use actix_web::{get, post, web::Json, HttpRequest, HttpResponse};

use crate::{dto_s::message_dto::MessageDTO, models::message::Message};

#[post("/send_message")]
pub async fn send_message(message: Json<MessageDTO>, _req: HttpRequest) -> HttpResponse {
    let msg: Message = message.0.into();

    HttpResponse::Ok().json(msg.message)

    // let result = db_queries::insert_message(msg).await;
    // match result {
    //     Ok(responce) => responce,
    //     Err(err) => todo!(),
    // }
}

#[get("/chat")]
async fn get_chat(user_one: String, user_two: String, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(format!("{} messages to {}", user_one, user_two))

    // db_layer::query_chat(message.0).await
}
