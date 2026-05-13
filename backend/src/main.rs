mod app;
mod handlers;
mod models;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::default();
    let app = app::build_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server started on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
