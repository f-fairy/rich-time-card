use crate::models::attendance::AttendanceStatus;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub type AttendanceStatusStorage = Arc<Mutex<HashMap<String, AttendanceStatus>>>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub attendance_statuses: AttendanceStatusStorage,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool,
            attendance_statuses: AttendanceStatusStorage::default(),
        }
    }
}
