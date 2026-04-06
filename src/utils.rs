use tokio::signal;
use tracing::info;

pub async fn shutdown_signal() {
    let ctrl_c = signal::ctrl_c();

    #[cfg(unix)]
    let mut terminate =
        signal::unix::signal(signal::unix::SignalKind::terminate()).expect("Unable to terminate");

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Ctrl-C received"),
        _ = terminate.recv() => info!("Recieved SIGTERM")
    }

    info!("shutdown signal received, draining connections...");
}
