mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use routes::*;

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub async fn run(listener: tokio::net::TcpListener) -> Result<(), std::io::Error> {
    axum::serve(listener, app()).await
}
