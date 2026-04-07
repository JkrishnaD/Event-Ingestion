use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    db::DbPool,
    handlers::{events::insert_event, health::health_handler},
};

mod events;
mod health;

pub fn events_router(pool: DbPool) -> Router {
    let pool_clone = pool.clone();
    Router::new()
        .route("/health", get(health_handler))
        .route("/event", post(insert_event))
        .with_state(pool_clone)
}
