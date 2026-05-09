use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server started on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Rich Time Card API"
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
