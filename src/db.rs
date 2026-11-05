use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

/// Establish and verify the shared pool before the HTTP server starts.
pub async fn connect(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}
