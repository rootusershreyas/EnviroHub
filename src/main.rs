use envirohub::{app, config::Config, db, state::AppState, telemetry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    telemetry::init().map_err(|error| -> Box<dyn std::error::Error> { error })?;
    let config = Config::from_env()?;
    let pool = db::connect(&config.database_url, config.database_max_connections).await?;
    tracing::info!("PostgreSQL connection established");
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    tracing::info!(address = %listener.local_addr()?, "EnviroHub listening");
    let result = axum::serve(listener, app::router(AppState { pool: pool.clone() }))
        .with_graceful_shutdown(shutdown_signal())
        .await;
    pool.close().await;
    result?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    tracing::info!("Shutdown requested");
}
