use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;

use crate::{
    mood::create_mood_entry::{self},
    state::AppState,
};

// =============================================================================
// API Docs
// =============================================================================

#[derive(OpenApi)]
#[openapi(paths(create_mood_entry::create_mood_entry))]
pub struct MoodApi;

// =============================================================================
// Router
// =============================================================================

pub fn mood_router() -> Router<AppState> {
    Router::new().route(
        "/mood/assignments",
        post(create_mood_entry::create_mood_entry),
    )
}
