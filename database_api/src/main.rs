const PAYLOAD_LIMIT: usize = 4096;
const IP_ADDR: &str = "127.0.0.1";
const PORT: i32 = 8080;

use actix_web::{error::InternalError, web::JsonConfig, App, HttpResponse, HttpServer};

use database_api::endpoints::messages_endpoints::send_message;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = dotenv::dotenv();
    if env.is_err() {
        panic!("Cannot read env variables!");
    }

    let server = HttpServer::new(|| {
        let json_config = JsonConfig::default()
            .limit(PAYLOAD_LIMIT)
            .error_handler(|err, _req| {
                InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            });

        App::new()
            //Json config
            .app_data(json_config)
            //Add all endpoints as services
            .service(send_message)
    })
    .bind(format!("{}:{}", IP_ADDR, PORT))?
    .run()
    .await;

    match server {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
