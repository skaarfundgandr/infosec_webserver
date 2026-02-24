use std::env;
use diesel_async::AsyncMysqlConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::{Object, Pool, PoolError};
use once_cell::sync::Lazy;
use dotenvy::dotenv;

pub struct Database {
    pool: Pool<AsyncMysqlConnection>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            pool: DB_POOL.clone(),
        }
    }

    pub async fn get_connection(&self) -> Result<Object<AsyncMysqlConnection>, PoolError> {
        self.pool.get().await
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

static DB_POOL: Lazy<Pool<AsyncMysqlConnection>> = Lazy::new(|| {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(database_url);
    let pool = Pool::builder(config)
        .build()
        .expect("Failed to create database connection pool");

    pool
});
