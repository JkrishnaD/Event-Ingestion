use crate::db::AppState;
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct IncomingEvent {
    pub app_id: i32,
    pub user_id: i32,
    pub event_type: String,
    pub data: serde_json::Value,
}

#[instrument(skip(pool, event))]
#[axum::debug_handler]
pub async fn insert_event(
    State(pool): State<AppState>,
    Json(event): Json<IncomingEvent>,
) -> StatusCode {
    tracing::info!("Sending event to batching");
    let state = pool.tx;
    state.send(event).await.ok();

    tracing::info!("{}", StatusCode::ACCEPTED);
    StatusCode::ACCEPTED
}
