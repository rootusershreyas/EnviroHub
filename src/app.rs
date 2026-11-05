use axum::{Json, Router, routing::get};
use serde::Serialize;
use tower_http::trace::TraceLayer;

use crate::error::AppError;

/// Feature routers can be nested under `/api/v1` here as they are implemented.
pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn not_found() -> AppError {
    AppError::NotFound
}
