use crate::routes;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
        .with_state(state)
}

pub async fn run(listener: tokio::net::TcpListener, db_pool: PgPool) -> Result<(), std::io::Error> {
    let state = AppState { db_pool };
    axum::serve(listener, app(state)).await
}
