use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub enum AttendanceStatus {
    BeforeWork,
    Working,
    Away,
    Finished,
}

impl AttendanceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BeforeWork => "BEFORE_WORK",
            Self::Working => "WORKING",
            Self::Away => "AWAY",
            Self::Finished => "FINISHED",
        }
    }
}

#[derive(Deserialize)]
pub struct AttendanceActionRequest {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct AttendanceActionResponse {
    pub result: &'static str,
    pub status: &'static str,
}

#[derive(Deserialize)]
pub struct AttendanceStatusQuery {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct AttendanceStatusResponse {
    pub user_id: String,
    pub work_date: &'static str,
    pub status: &'static str,
}

#[derive(Deserialize)]
pub struct AttendanceEventsQuery {
    pub user_id: String,
    pub work_date: String,
}

#[derive(Serialize)]
pub struct AttendanceEvent {
    pub event_id: u64,
    pub event_type: &'static str,
    pub event_at: &'static str,
}

#[derive(Serialize)]
pub struct AttendanceEventsResponse {
    pub user_id: String,
    pub work_date: String,
    pub events: Vec<AttendanceEvent>,
}
