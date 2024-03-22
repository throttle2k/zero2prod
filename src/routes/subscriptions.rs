use axum::{extract::Form, response::Response};

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(Form(_form): Form<FormData>) -> Response {
    Response::default()
}
