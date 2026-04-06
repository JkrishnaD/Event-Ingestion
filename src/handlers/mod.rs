use axum::{Router, routing::get};

use crate::{db::DbPool, handlers::health::health_handler};

mod health;

pub fn create_events(pool: DbPool) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .with_state(pool)
}
