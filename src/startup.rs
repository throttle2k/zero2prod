use crate::routes;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub fn app(state: AppState) -> Router {
    let middleware = ServiceBuilder::new().layer(TraceLayer::new_for_http().on_failure(()));
    Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
        .with_state(state)
        .layer(middleware)
}

pub async fn run(listener: tokio::net::TcpListener, db_pool: PgPool) -> Result<(), std::io::Error> {
    let state = AppState { db_pool };
    axum::serve(listener, app(state)).await
}
