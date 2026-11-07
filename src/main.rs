use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Home"
}
