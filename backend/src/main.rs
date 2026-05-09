use axum::{
    Router,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Deserialize)]
struct CheckinRequest {
    user_id: String,
}

#[derive(Serialize)]
struct CheckinResponse {
    result: &'static str,
    status: &'static str,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(health))
        .route("/api/attendance/checkin", post(checkin));

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

async fn checkin(Json(payload): Json<CheckinRequest>) -> Json<CheckinResponse> {
    let _user_id = payload.user_id;

    Json(CheckinResponse {
        result: "success",
        status: "WORKING",
    })
}
