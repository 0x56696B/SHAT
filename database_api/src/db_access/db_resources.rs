use diesel::pg::PgConnection;

pub struct DbResources {
    pub db_conn: PgConnection,
}

impl DbResources {
    pub fn new(conn: PgConnection) -> Self {
        DbResources { db_conn: conn }
    }
}
