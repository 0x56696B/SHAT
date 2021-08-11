const PAYLOAD_LIMIT: usize = 4096;
const IP_ADDR: &str = "127.0.0.1";
const PORT: &str = "8080";

use actix_web::{dev::Body, error::InternalError, web::JsonConfig, App, HttpResponse, HttpServer};
use database_api::{
    db_access::{
        db_resources::{self, DbResources},
        db_setup,
    },
    endpoints,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = db_setup::establish_connection().expect("Unable to make connection to database");
    let db_recourses = db_resources::DbResources::new(conn);

    let server = HttpServer::new(|| {
        let json_config = JsonConfig::default()
            .limit(PAYLOAD_LIMIT)
            .error_handler(|err, req| {
                InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            });

        App::new()
            // .app_data(db_recourses)
            .app_data(json_config)
    })
    .bind(format!("{}:{}", IP_ADDR, PORT))?
    .run()
    .await;

    match server {
        Ok(success) => Ok(()),
        Err(err) => Err(err),
    }
}
