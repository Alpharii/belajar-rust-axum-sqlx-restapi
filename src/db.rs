use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn create_pg_pool() -> Result<PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL harus diset");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
