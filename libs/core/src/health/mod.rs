use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use serde::Serialize;

use crate::{AppState, config::Config, error::CoreError};

#[derive(Serialize)]
struct Health {
    status: String,
}

pub fn health_router() -> Router<AppState> {
    async fn handler() -> impl IntoResponse {
        Json(Health {
            status: "up".to_string(),
        })
    }

    async fn config(State(config): State<Arc<Config>>) -> Result<impl IntoResponse, CoreError> {
        Ok(Json(Health {
            status: config.port.to_string(),
        }))
    }

    Router::new()
        .route("/health", get(handler))
        .route("/health/config", get(config))
}
