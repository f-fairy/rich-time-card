use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, time::Duration};

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Shared PostgreSQL pool used by DB-backed attendance endpoints.
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
}
