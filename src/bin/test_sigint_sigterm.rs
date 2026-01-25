use tokio::signal;

#[tokio::main]
async fn main() {
    let ctrl_c = signal::ctrl_c();

    // SIGTERM is present only on unix systems
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Cannot create SIGTERM handler")
            .recv()
            .await;
    };

    // Mock for non unix systems
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // wait for SIGINT or SIGTERM
    let result = tokio::select! {
        _ = ctrl_c => "SIGINT",
        _ = terminate => "SIGTERM",
    };
    println!("Triggered: {}", result);
}
