use tokio_postgres::{tls::NoTlsStream, Client, Connection, NoTls, Socket};

use crate::database::db_setup::establish_db_connection;

pub struct DbResources {
    pub client: Client,
    pub connection: Connection<Socket, NoTlsStream>,
}

impl DbResources {
    pub async fn new() -> Self {
        let db_conn = establish_db_connection().await;

        DbResources {
            client: db_conn.0,
            connection: db_conn.1,
        }
    }
}
