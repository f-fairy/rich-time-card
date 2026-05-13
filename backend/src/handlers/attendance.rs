use crate::{
    models::attendance::{
        AttendanceActionRequest, AttendanceActionResponse, AttendanceEvent, AttendanceEventsQuery,
        AttendanceEventsResponse, AttendanceStatus, AttendanceStatusQuery,
        AttendanceStatusResponse,
    },
    state::{AppState, AttendanceStatusStorage},
};
use axum::{
    extract::{Query, State},
    response::Json,
};

pub async fn checkin(
    State(state): State<AppState>,
    Json(payload): Json<AttendanceActionRequest>,
) -> Json<AttendanceActionResponse> {
    set_attendance_status(
        &state.attendance_statuses,
        payload.user_id,
        AttendanceStatus::Working,
    )
    .await
}

pub async fn checkout(
    State(state): State<AppState>,
    Json(payload): Json<AttendanceActionRequest>,
) -> Json<AttendanceActionResponse> {
    set_attendance_status(
        &state.attendance_statuses,
        payload.user_id,
        AttendanceStatus::Finished,
    )
    .await
}

pub async fn break_start(
    State(state): State<AppState>,
    Json(payload): Json<AttendanceActionRequest>,
) -> Json<AttendanceActionResponse> {
    set_attendance_status(
        &state.attendance_statuses,
        payload.user_id,
        AttendanceStatus::Away,
    )
    .await
}

pub async fn break_end(
    State(state): State<AppState>,
    Json(payload): Json<AttendanceActionRequest>,
) -> Json<AttendanceActionResponse> {
    set_attendance_status(
        &state.attendance_statuses,
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

pub async fn attendance_status(
    State(state): State<AppState>,
    Query(params): Query<AttendanceStatusQuery>,
) -> Json<AttendanceStatusResponse> {
    let status = state
        .attendance_statuses
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

pub async fn attendance_events(
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
