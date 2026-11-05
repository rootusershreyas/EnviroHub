use sqlx::PgPool;

/// Cloning PgPool shares its underlying connections; it does not create a new pool.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
