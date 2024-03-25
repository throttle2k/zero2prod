use crate::{
    domain::{NewSubscriber, SubscriberEmail, SubscriberName},
    startup::AppState,
};
use axum::{
    extract::{Form, State},
    http::StatusCode,
};
use axum_macros::debug_handler;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[debug_handler]
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, state),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    State(state): State<AppState>,
    Form(form): Form<FormData>,
) -> Result<(), StatusCode> {
    let name = match SubscriberName::parse(form.name) {
        Ok(name) => name,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    let email = match SubscriberEmail::parse(form.email) {
        Ok(email) => email,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let new_subscriber = NewSubscriber { email, name };
    match insert_subscriber(&state.db_pool, &new_subscriber).await {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber in the database",
    skip(new_subscriber, db_pool)
)]
pub async fn insert_subscriber(
    db_pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
