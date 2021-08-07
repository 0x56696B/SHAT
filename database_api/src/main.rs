use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use database_api::Message;

mod lib;

#[get("/")]
async fn send_message(message: Message) -> HttpResponse {
    format!("aaaaa")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(send_message))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
