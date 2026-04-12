use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    db::AppState,
    handlers::{events::insert_event, health::health_handler},
};

pub mod events;
mod health;

// Router for the events and health endpoints
pub fn events_router(pool: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/event", post(insert_event))
        .with_state(pool)
}
