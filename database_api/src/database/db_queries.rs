use actix_web::{web::Data, Error, HttpRequest, HttpResponse};

use crate::{app_state::AppState, models::pure::chat::message::Message};

pub async fn insert_message(
    _message_struct: Message,
    req: HttpRequest,
) -> Result<HttpResponse, HttpResponse> {
    let data_opt = req.app_data::<Data<AppState>>();
    if data_opt.is_none() {
        return Err(HttpResponse::InternalServerError().body("Unable to get application data!"));
    }

    let data = &data_opt.unwrap();
    let client = &data.db_recourses.client;

    // client.prepare()

    todo!("Implement insert_into_db")
}

// pub async fn query_chat(chat: Chat) -> Result<HttpResponse, Error> {
//     todo!("Implement query_from_db")
// }
