use axum::http::StatusCode;

pub async fn health_check() -> Result<(), StatusCode> {
    Ok(())
}
