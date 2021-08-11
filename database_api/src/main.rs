const PAYLOAD_LIMIT: usize = 4096;
const IP_ADDR: &str = "127.0.0.1";
const PORT: &str = "8080";

use actix_web::{error::InternalError, web::JsonConfig, App, HttpResponse, HttpServer};
use database_api::db_access::{db_resources, db_setup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        let json_config = JsonConfig::default()
            .limit(PAYLOAD_LIMIT)
            .error_handler(|err, _req| {
                InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            });

        App::new()
            //Wtf... But nothing else works. since Copy/Clone ain't implemented
            .app_data(db_resources::DbResources::new(
                db_setup::establish_connection().expect("Unable to make connection to database"),
            ))
            .app_data(json_config)
    })
    .bind(format!("{}:{}", IP_ADDR, PORT))?
    .run()
    .await;

    match server {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
