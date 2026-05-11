use axum::{
    Router,
    extract::{Query, State},
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type AttendanceStatusStorage = Arc<Mutex<HashMap<String, AttendanceStatus>>>;

#[derive(Clone, Copy)]
enum AttendanceStatus {
    BeforeWork,
    Working,
    Away,
    Finished,
}

impl AttendanceStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::BeforeWork => "BEFORE_WORK",
            Self::Working => "WORKING",
            Self::Away => "AWAY",
            Self::Finished => "FINISHED",
        }
    }
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Deserialize)]
struct AttendanceActionRequest {
    user_id: String,
}

#[derive(Serialize)]
struct AttendanceActionResponse {
    result: &'static str,
    status: &'static str,
}

#[derive(Deserialize)]
struct AttendanceStatusQuery {
    user_id: String,
}

#[derive(Serialize)]
struct AttendanceStatusResponse {
    user_id: String,
    work_date: &'static str,
    status: &'static str,
}

#[derive(Deserialize)]
struct AttendanceEventsQuery {
    user_id: String,
    work_date: String,
}

#[derive(Serialize)]
struct AttendanceEvent {
    event_id: u64,
    event_type: &'static str,
    event_at: &'static str,
}

#[derive(Serialize)]
struct AttendanceEventsResponse {
    user_id: String,
    work_date: String,
    events: Vec<AttendanceEvent>,
}

#[tokio::main]
async fn main() {
    let attendance_statuses = AttendanceStatusStorage::default();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(health))
        .route("/api/attendance/checkin", post(checkin))
        .route("/api/attendance/checkout", post(checkout))
        .route("/api/attendance/break-start", post(break_start))
        .route("/api/attendance/break-end", post(break_end))
        .route("/api/attendance/status", get(attendance_status))
        .route("/api/attendance/events", get(attendance_events))
        .with_state(attendance_statuses);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server started on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Rich Time Card API"
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn checkin(
    State(attendance_statuses): State<AttendanceStatusStorage>,
    Json(payload): Json<AttendanceActionRequest>,
) -> Json<AttendanceActionResponse> {
    set_attendance_status(
        &attendance_statuses,
        payload.user_id,
        AttendanceStatus::Working,
    )
    .await
}

async fn checkout(
    State(attendance_statuses): State<AttendanceStatusStorage>,
    Json(payload): Json<AttendanceActionRequest>,
) -> Json<AttendanceActionResponse> {
    set_attendance_status(
        &attendance_statuses,
        payload.user_id,
        AttendanceStatus::Finished,
    )
    .await
}

async fn break_start(
    State(attendance_statuses): State<AttendanceStatusStorage>,
    Json(payload): Json<AttendanceActionRequest>,
) -> Json<AttendanceActionResponse> {
    set_attendance_status(
        &attendance_statuses,
        payload.user_id,
        AttendanceStatus::Away,
    )
    .await
}

async fn break_end(
    State(attendance_statuses): State<AttendanceStatusStorage>,
    Json(payload): Json<AttendanceActionRequest>,
) -> Json<AttendanceActionResponse> {
    set_attendance_status(
        &attendance_statuses,
        payload.user_id,
        AttendanceStatus::Working,
    )
    .await
}

async fn set_attendance_status(
    attendance_statuses: &AttendanceStatusStorage,
    user_id: String,
    status: AttendanceStatus,
) -> Json<AttendanceActionResponse> {
    attendance_statuses.lock().await.insert(user_id, status);

    Json(AttendanceActionResponse {
        result: "success",
        status: status.as_str(),
    })
}

async fn attendance_status(
    State(attendance_statuses): State<AttendanceStatusStorage>,
    Query(params): Query<AttendanceStatusQuery>,
) -> Json<AttendanceStatusResponse> {
    let status = attendance_statuses
        .lock()
        .await
        .get(&params.user_id)
        .copied()
        .unwrap_or(AttendanceStatus::BeforeWork);

    Json(AttendanceStatusResponse {
        user_id: params.user_id,
        work_date: "2026-05-11",
        status: status.as_str(),
    })
}

async fn attendance_events(
    Query(params): Query<AttendanceEventsQuery>,
) -> Json<AttendanceEventsResponse> {
    Json(AttendanceEventsResponse {
        user_id: params.user_id,
        work_date: params.work_date,
        events: vec![
            AttendanceEvent {
                event_id: 1,
                event_type: "CLOCK_IN",
                event_at: "2026-05-11T09:00:00Z",
            },
            AttendanceEvent {
                event_id: 2,
                event_type: "GO_OUT",
                event_at: "2026-05-11T12:00:00Z",
            },
            AttendanceEvent {
                event_id: 3,
                event_type: "RETURN",
                event_at: "2026-05-11T13:00:00Z",
            },
            AttendanceEvent {
                event_id: 4,
                event_type: "CLOCK_OUT",
                event_at: "2026-05-11T18:00:00Z",
            },
        ],
    })
}
