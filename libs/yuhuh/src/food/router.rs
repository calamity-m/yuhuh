use crate::{
    food::{
        create_food_entries::{self},
        find_food_entry::{self},
    },
    state::AppState,
};

use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;

// =============================================================================
// API Docs
// =============================================================================

#[derive(OpenApi)]
#[openapi(paths(
    find_food_entry::find_food_entry,
    create_food_entries::create_food_entry
))]
pub struct FoodApi;

// =============================================================================
// Router
// =============================================================================

pub fn food_router() -> Router<AppState> {
    Router::new()
        .route("/food", get(find_food_entry::find_food_entry))
        .route("/food/create", post(create_food_entries::create_food_entry))
}
