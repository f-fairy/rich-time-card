use crate::{
    handlers::{attendance, health},
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn build_router(state: AppState) -> Router {
    // Central route registration; handlers decide whether data is DB-backed or still mocked.
    Router::new()
        .route("/", get(health::root))
        .route("/api/health", get(health::health))
        .route("/api/attendance/checkin", post(attendance::checkin))
        .route("/api/attendance/checkout", post(attendance::checkout))
        .route("/api/attendance/break-start", post(attendance::break_start))
        .route("/api/attendance/break-end", post(attendance::break_end))
        .route("/api/attendance/status", get(attendance::attendance_status))
        .route("/api/attendance/events", get(attendance::attendance_events))
        .with_state(state)
}
