use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct DbPool {
    pub pool: PgPool,
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
}
