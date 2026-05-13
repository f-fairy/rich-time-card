use crate::models::attendance::AttendanceStatus;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub type AttendanceStatusStorage = Arc<Mutex<HashMap<String, AttendanceStatus>>>;

#[derive(Clone, Default)]
pub struct AppState {
    pub attendance_statuses: AttendanceStatusStorage,
}
