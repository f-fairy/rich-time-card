use crate::{
    models::attendance::{
        AttendanceActionRequest, AttendanceActionResponse, AttendanceEvent, AttendanceEventsQuery,
        AttendanceEventsResponse, AttendanceStatus, AttendanceStatusQuery,
        AttendanceStatusResponse,
    },
    state::{AppState, AttendanceStatusStorage},
};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub(crate) struct AppError {
    status: StatusCode,
    message: String,
}

impl AppError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }

    fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorResponse {
                error: self.message,
            }),
        )
            .into_response()
    }
}

pub async fn checkin(
    State(state): State<AppState>,
    Json(payload): Json<AttendanceActionRequest>,
) -> Result<Json<AttendanceActionResponse>, AppError> {
    println!("checkin request received: user_id={}", payload.user_id);

    validate_user_exists(&state.db_pool, &payload.user_id).await?;

    let mut tx = state.db_pool.begin().await.map_err(|error| {
        println!("database error starting checkin transaction: {error}");
        AppError::internal("failed to start database transaction")
    })?;

    let event_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO attendance_events (user_id, work_date, occurred_at, event_type)
        VALUES ($1::uuid, CURRENT_DATE, NOW(), $2::attendance_event_type)
        RETURNING event_id
        "#,
    )
    .bind(&payload.user_id)
    .bind("CLOCK_IN")
    .fetch_one(&mut *tx)
    .await
    .map_err(map_sqlx_error)?;

    println!("inserted attendance_event: event_id={event_id}");

    sqlx::query(
        r#"
        INSERT INTO current_attendance_status (
            user_id,
            work_date,
            current_status,
            last_event_id,
            updated_at
        )
        VALUES ($1::uuid, CURRENT_DATE, $2::attendance_status_type, $3, NOW())
        ON CONFLICT (user_id)
        DO UPDATE SET
            work_date = EXCLUDED.work_date,
            current_status = EXCLUDED.current_status,
            last_event_id = EXCLUDED.last_event_id,
            updated_at = NOW()
        "#,
    )
    .bind(&payload.user_id)
    .bind(AttendanceStatus::Working.as_str())
    .bind(event_id)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx_error)?;

    println!(
        "current_attendance_status upsert success: user_id={}, event_id={event_id}",
        payload.user_id
    );

    tx.commit().await.map_err(|error| {
        println!("database error committing checkin transaction: {error}");
        AppError::internal("failed to commit database transaction")
    })?;

    Ok(Json(AttendanceActionResponse {
        result: "success",
        status: AttendanceStatus::Working.as_str(),
    }))
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
) -> Result<Json<AttendanceStatusResponse>, AppError> {
    println!("status request received: user_id={}", params.user_id);

    validate_user_exists(&state.db_pool, &params.user_id).await?;

    let status = sqlx::query_as::<_, (String, String, String)>(
        r#"
        SELECT CURRENT_DATE::text, work_date::text, current_status::text
        FROM current_attendance_status
        WHERE user_id = $1::uuid
        "#,
    )
    .bind(&params.user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(map_sqlx_error)?;

    let (work_date, status) = match status {
        Some((current_date, stored_work_date, stored_status))
            if stored_work_date == current_date =>
        {
            println!(
                "status found for today: user_id={}, work_date={}, status={}",
                params.user_id, stored_work_date, stored_status
            );
            (stored_work_date, stored_status)
        }
        Some((current_date, stored_work_date, _)) => {
            println!(
                "previous-day status ignored: user_id={}, stored_work_date={}, current_date={}",
                params.user_id, stored_work_date, current_date
            );
            (
                current_date,
                AttendanceStatus::BeforeWork.as_str().to_owned(),
            )
        }
        None => {
            let work_date: String = sqlx::query_scalar("SELECT CURRENT_DATE::text")
                .fetch_one(&state.db_pool)
                .await
                .map_err(map_sqlx_error)?;

            println!(
                "status not found: user_id={}, current_date={}",
                params.user_id, work_date
            );

            (work_date, AttendanceStatus::BeforeWork.as_str().to_owned())
        }
    };

    Ok(Json(AttendanceStatusResponse {
        user_id: params.user_id,
        work_date,
        status,
    }))
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

async fn validate_user_exists(db_pool: &PgPool, user_id: &str) -> Result<(), AppError> {
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE user_id = $1::uuid)")
            .bind(user_id)
            .fetch_one(db_pool)
            .await
            .map_err(map_sqlx_error)?;

    if exists {
        Ok(())
    } else {
        Err(AppError::not_found("user_id does not exist"))
    }
}

fn map_sqlx_error(error: sqlx::Error) -> AppError {
    println!("database error: {error}");

    if let sqlx::Error::Database(database_error) = &error {
        if database_error.code().as_deref() == Some("22P02") {
            return AppError::bad_request("user_id must be a valid UUID string");
        }
    }

    AppError::internal("database operation failed")
}
