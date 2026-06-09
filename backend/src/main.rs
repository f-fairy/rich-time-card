// Backend modules are kept small: routing, database setup, handlers, shared models, and app state.
mod app;
mod db;
mod handlers;
mod models;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool()
        .await
        .expect("failed to connect to database");
    println!("Connected to PostgreSQL database");

    let state = AppState::new(db_pool);
    let app = app::build_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server started on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
