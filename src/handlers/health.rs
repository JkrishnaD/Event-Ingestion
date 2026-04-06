use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn health_handler() -> impl IntoResponse {
    tracing::info!("Health check");
    Json(HealthResponse { status: "OK" })
}
