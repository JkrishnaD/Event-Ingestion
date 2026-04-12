use tokio::net::TcpListener;

use crate::{handlers::events_router, utils::shutdown_signal};

mod batcher;
mod db;
mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    // Initialize the tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Bind the TCP listener to the address
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server started at 3000");

    // Create a channel for sending events to the batcher
    let (tx, rx) = tokio::sync::mpsc::channel(10000);

    // Fetch the database URL from environment variables
    let url = std::env::var("DATABASE_URL").expect("Failed to fetch url");
    tracing::info!("Server is good to start communication");
    // Create AppState with the database pool and event channel
    let state = db::AppState::new(&url, tx).await;

    // Create the Axum router with the AppState
    let app = events_router(state.clone());

    // Spawn batcher job task for processing events
    tokio::spawn(batcher::batch_jobs(rx, state.pool.clone()));

    // Start the Axum server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}
