use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;

use crate::{
    mood::{
        create_mood_entry::{self},
        read_mood_entries,
    },
    state::AppState,
};

// =============================================================================
// API Docs
// =============================================================================

#[derive(OpenApi)]
#[openapi(paths(
    create_mood_entry::create_mood_entry,
    read_mood_entries::read_mood_entries
))]
pub struct MoodApi;

// =============================================================================
// Router
// =============================================================================

pub fn mood_router() -> Router<AppState> {
    Router::new()
        .route("/mood", post(create_mood_entry::create_mood_entry))
        .route("/mood", get(read_mood_entries::read_mood_entries))
}
