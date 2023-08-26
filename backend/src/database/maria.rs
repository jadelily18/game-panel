use std::env;
use sqlx::{
    pool::Pool,
    mysql::MySqlPool,
    MySql, Error
};

pub async fn create_pool() -> Result<Pool<MySql>, Error> {
    let conn_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(err) => panic!("Failed to get environment variable `DATABASE_URL`: {}", err)
    };

    let conn = MySqlPool::connect(&*conn_url).await;

    info!("Successfully connected to MariaDB database!");

    conn
}
