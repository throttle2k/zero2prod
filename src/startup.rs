use crate::routes;
use axum::{
    routing::{get, post},
    Router,
};

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
}

pub async fn run(listener: tokio::net::TcpListener) -> Result<(), std::io::Error> {
    axum::serve(listener, app()).await
}
