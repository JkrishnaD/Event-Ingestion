use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone, Debug)]
pub struct DbPool {
    pub pool: PgPool,
}

#[derive(Serialize, Debug)]
pub struct PoolDetails {
    size: u32,
    idle_connections: usize,
    is_closed: bool,
}

impl DbPool {
    pub async fn new(db_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .connect(db_url)
            .await
            .expect("Failed to connect");

        tracing::info!("Database Connection Established");

        Self { pool }
    }

    pub async fn get_pool_details(&self) -> PoolDetails {
        PoolDetails {
            size: self.pool.size(),
            idle_connections: self.pool.num_idle(),
            is_closed: self.pool.is_closed(),
        }
    }
}
