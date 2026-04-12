use axum::{Json, extract::State, response::IntoResponse};
use serde::Serialize;

use crate::db::{AppState, PoolDetails};

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    pool_details: PoolDetails,
}

// Handler for the health check endpoint
pub async fn health_handler(State(db_pool): State<AppState>) -> impl IntoResponse {
    tracing::info!("Health check");
    let details = db_pool.get_pool_details().await;

    tracing::info!("Pool details: {:?}", details);
    Json(HealthResponse {
        status: "OK",
        pool_details: details,
    })
}
