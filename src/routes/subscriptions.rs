use crate::startup::AppState;
use axum::{
    extract::{Form, State},
    http::StatusCode,
};
use axum_macros::debug_handler;
use chrono::Utc;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[debug_handler]
pub async fn subscribe(
    State(state): State<AppState>,
    Form(form): Form<FormData>,
) -> Result<(), StatusCode> {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
