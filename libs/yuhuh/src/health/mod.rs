use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;
use tracing::instrument;
use utoipa::OpenApi;

use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(paths(handler))]
pub struct HealthApi;

/// Health check
#[utoipa::path(get, tag = "health", path = "health")]
#[instrument]
pub(crate) async fn handler() -> impl IntoResponse {
    Json(json!({"status": "ok"}))
}

pub fn health_router() -> Router<AppState> {
    Router::new().route("/health", get(handler))
}
