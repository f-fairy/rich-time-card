use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn root() -> &'static str {
    "Rich Time Card API"
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
