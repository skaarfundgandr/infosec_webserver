use diesel_async::AsyncMysqlConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::{Object, Pool, PoolError};
use dotenvy::dotenv;
use mysql_async::Conn;
use once_cell::sync::Lazy;
use std::env;

pub struct Database {
    pool: Pool<AsyncMysqlConnection>,
    mysql_pool: mysql_async::Pool,
}

impl Database {
    pub fn new() -> Self {
        Database {
            pool: DB_POOL.clone(),
            mysql_pool: MYSQL_DB_POOL.clone(),
        }
    }

    pub async fn get_connection(&self) -> Result<Object<AsyncMysqlConnection>, PoolError> {
        self.pool.get().await
    }

    pub async fn get_mysql_connection(&self) -> mysql_async::Result<Conn> {
        self.mysql_pool.get_conn().await
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

    Pool::builder(config)
        .build()
        .expect("Failed to create database connection pool")
});

static MYSQL_DB_POOL: Lazy<mysql_async::Pool> = Lazy::new(|| {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    mysql_async::Pool::new(database_url.as_str())
});
