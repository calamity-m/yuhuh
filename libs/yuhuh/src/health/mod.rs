use axum::{Json, Router, response::IntoResponse, routing::get};
use serde::Serialize;

use crate::state::AppState;

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

    Router::new().route("/health", get(handler))
}
