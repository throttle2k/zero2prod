use axum::response::Response;

pub async fn health_check() -> Response {
    Response::default()
}
