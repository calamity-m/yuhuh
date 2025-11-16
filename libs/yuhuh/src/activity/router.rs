use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;

use crate::{activity::create_activity_entries, state::AppState};

// =============================================================================
// API Docs
// =============================================================================

#[derive(OpenApi)]
#[openapi(paths(create_activity_entries::create_activity_entries))]
pub struct ActivityApi;

// =============================================================================
// Router
// =============================================================================

pub fn activity_router() -> Router<AppState> {
    Router::new().route(
        "/activity",
        post(create_activity_entries::create_activity_entries),
    )
}
