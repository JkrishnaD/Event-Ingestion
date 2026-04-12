use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::mpsc;

use crate::handlers::events::IncomingEvent;

// Struct to hold the App state, including the database pool and event sender
#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub tx: mpsc::Sender<IncomingEvent>,
}

// Struct to hold pool details (size, idle connections, is closed)
#[derive(Serialize, Debug)]
pub struct PoolDetails {
    size: u32,
    idle_connections: usize,
    is_closed: bool,
}

impl AppState {
    // Method to create a new AppState instance with a database pool and event sender
    pub async fn new(db_url: &str, tx: mpsc::Sender<IncomingEvent>) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .connect(db_url)
            .await
            .expect("Failed to connect");

        tracing::info!("Database Connection Established");

        Self { pool, tx }
    }

    // Method to get pool details (size, idle connections, is closed)
    pub async fn get_pool_details(&self) -> PoolDetails {
        PoolDetails {
            size: self.pool.size(),
            idle_connections: self.pool.num_idle(),
            is_closed: self.pool.is_closed(),
        }
    }
}
