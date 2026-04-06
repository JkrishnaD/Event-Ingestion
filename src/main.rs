use tokio::net::TcpListener;

use crate::{handlers::create_events, utils::shutdown_signal};

mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = create_events();
    tracing::info!("Server started at 3000");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}
