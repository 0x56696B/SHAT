use tokio_postgres::{tls::NoTlsStream, Client, Connection, NoTls, Socket};

//After establishing connection, it's required to spawn it, in
//order to use it.
pub async fn establish_db_connection() -> (Client, Connection<Socket, NoTlsStream>) {
    let db_conn_str = dotenv::var("DB_CONN_STR_ENV_VAR")
        .expect("Unable to get DB_CONN_STR_ENV_VAR environment variable!");

    tokio_postgres::connect(db_conn_str.as_str(), NoTls)
        .await
        .expect("Unable to connect to database")
}

pub async fn spawn_connection(connection: Connection<Socket, NoTlsStream>) {
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
}
