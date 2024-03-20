use axum::{
    extract::Form,
    response::Response,
    routing::{get, post},
    Router,
};

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn health_check() -> Response {
    Response::default()
}

async fn subscribe(Form(_form): Form<FormData>) -> Response {
    Response::default()
}

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub async fn run(listener: tokio::net::TcpListener) -> Result<(), std::io::Error> {
    axum::serve(listener, app()).await
}
