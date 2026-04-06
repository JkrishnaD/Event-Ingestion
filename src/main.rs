use tokio::net::TcpListener;

use crate::{handlers::create_events, utils::shutdown_signal};

mod db;
mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server started at 3000");

    let url = std::env::var("DATABASE_URL").expect("Failed to fetch url");
    let pool = db::DbPool::new(&url).await;

    let app = create_events(pool);

    tracing::info!("Server is good to start communication");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}
