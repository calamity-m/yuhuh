use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;

use crate::{
    mood::create_assignments::{self},
    state::AppState,
};

// =============================================================================
// API Docs
// =============================================================================

#[derive(OpenApi)]
#[openapi(paths(create_assignments::create_assignments))]
pub struct MoodApi;

// =============================================================================
// Router
// =============================================================================

pub fn mood_router() -> Router<AppState> {
    Router::new().route(
        "/mood/assignments",
        post(create_assignments::create_assignments),
    )
}
