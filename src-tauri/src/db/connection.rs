use sqlx::postgres::PgPool;
use std::env;
use tokio::sync::Mutex;
pub struct DbConnectionPool {
    pub connection: Mutex<PgPool>,
}

pub async fn establish_connection_pool() -> Result<DbConnectionPool, String> {
    let _db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect("postgres://gofvuhxy:N0Kn9CqG1LeJCWDMfYdLFddTaNg_qGaY@trumpet.db.elephantsql.com/gofvuhxy")//&db_url)
        .await
        .map_err(|e| format!("Error connecting to db: {}", e))?;

    Ok(DbConnectionPool {
        connection: Mutex::new(pool),
    })
}
