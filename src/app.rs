use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use tower_http::trace::TraceLayer;

use crate::{error::AppError, state::AppState};

/// Feature routers can be nested under `/api/v1` here as they are implemented.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .fallback(not_found)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn ready(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        sqlx::query_scalar::<_, i32>("SELECT 1").fetch_one(&state.pool),
    )
    .await;

    match result {
        Ok(Ok(_)) => Ok(Json(HealthResponse { status: "ready" })),
        error => {
            tracing::warn!(error = ?error, "PostgreSQL readiness check failed");
            Err(AppError::Unavailable)
        }
    }
}

async fn not_found() -> AppError {
    AppError::NotFound
}
