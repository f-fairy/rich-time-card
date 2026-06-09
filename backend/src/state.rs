use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    // Attendance endpoints use PostgreSQL for persistent event and status data.
    pub db_pool: PgPool,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}
