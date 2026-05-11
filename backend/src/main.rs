use axum::{
    Router,
    extract::Query,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

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
    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(health))
        .route("/api/attendance/checkin", post(checkin))
        .route("/api/attendance/checkout", post(checkout))
        .route("/api/attendance/break-start", post(break_start))
        .route("/api/attendance/break-end", post(break_end))
        .route("/api/attendance/status", get(attendance_status))
        .route("/api/attendance/events", get(attendance_events));

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

async fn checkin(Json(payload): Json<AttendanceActionRequest>) -> Json<AttendanceActionResponse> {
    let _user_id = payload.user_id;

    Json(AttendanceActionResponse {
        result: "success",
        status: "WORKING",
    })
}

async fn checkout(Json(payload): Json<AttendanceActionRequest>) -> Json<AttendanceActionResponse> {
    let _user_id = payload.user_id;

    Json(AttendanceActionResponse {
        result: "success",
        status: "FINISHED",
    })
}

async fn break_start(
    Json(payload): Json<AttendanceActionRequest>,
) -> Json<AttendanceActionResponse> {
    let _user_id = payload.user_id;

    Json(AttendanceActionResponse {
        result: "success",
        status: "AWAY",
    })
}

async fn break_end(Json(payload): Json<AttendanceActionRequest>) -> Json<AttendanceActionResponse> {
    let _user_id = payload.user_id;

    Json(AttendanceActionResponse {
        result: "success",
        status: "WORKING",
    })
}

async fn attendance_status(
    Query(params): Query<AttendanceStatusQuery>,
) -> Json<AttendanceStatusResponse> {
    Json(AttendanceStatusResponse {
        user_id: params.user_id,
        work_date: "2026-05-11",
        status: "BEFORE_WORK",
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
