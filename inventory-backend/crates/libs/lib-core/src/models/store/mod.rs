use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::core_config;

pub(in crate::models) mod dbx;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> sqlx::Result<Db> {
    let max_connections = if cfg!(test) { 1 } else { 5 };

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&core_config().DB_URL)
        .await
}
