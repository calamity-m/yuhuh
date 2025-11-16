use crate::{
    food::{
        create_food_entries::{self},
        read_food_entries::{self},
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
    read_food_entries::read_food_entries,
    create_food_entries::create_food_entries
))]
pub struct FoodApi;

// =============================================================================
// Router
// =============================================================================

pub fn food_router() -> Router<AppState> {
    Router::new()
        .route("/food", get(read_food_entries::read_food_entries))
        .route(
            "/food/create",
            post(create_food_entries::create_food_entries),
        )
}
