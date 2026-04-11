use tokio::net::TcpListener;

use crate::{handlers::events_router, utils::shutdown_signal};

mod batcher;
mod db;
mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server started at 3000");

    let (tx, rx) = tokio::sync::mpsc::channel(1000);

    let url = std::env::var("DATABASE_URL").expect("Failed to fetch url");
    tracing::info!("Server is good to start communication");
    let state = db::AppState::new(&url, tx).await;

    let app = events_router(state.clone());

    tokio::spawn(batcher::batch_jobs(rx, state.pool.clone()));

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}
