use actix_web::{
    get, post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use diesel::Connection;

use crate::{
    app_state::AppState,
    db_access::{db_queries, db_resources::DbResources},
    dto_s::message_dto::MessageDTO,
    models::message::Message,
};

#[post("/send_message")]
pub async fn send_message(
    message: Json<MessageDTO>,
    _req: HttpRequest,
    data: web::Data<AppState>,
) -> HttpResponse {
    let msg: Message = message.0.into();

    //TODO: Query person info and populate Message struct

    //Database query
    let result = db_queries::insert_message(msg, data.db_recourses.db_conn).await;
    match result {
        Ok(responce) => responce,
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[get("/chat")]
async fn get_chat(user_one: String, user_two: String, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(format!("{} messages to {}", user_one, user_two))

    // db_layer::query_chat(message.0).await
}
