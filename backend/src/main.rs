use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));

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
