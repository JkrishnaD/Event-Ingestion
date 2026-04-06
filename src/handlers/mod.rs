use axum::{Router, routing::get};

use crate::handlers::health::health_handler;

mod health;

pub fn create_events() -> Router {
    Router::new().route("/health", get(health_handler))
}
