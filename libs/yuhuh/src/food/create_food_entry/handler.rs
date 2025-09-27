//! create food entry HTTP handler
//!
//! This module provides HTTP endpoints for creating food entries.

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::instrument;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{error::YuhuhError, food::state::FoodState};

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateFoodEntryRequest {
    pub user_id: Uuid,
    pub description: String,
    pub calories: Option<f32>,
    pub carbs: Option<f32>,
    pub protein: Option<f32>,
    pub fats: Option<f32>,
    pub micronutrients: Option<serde_json::Value>,
    pub logged_at: Option<DateTime<Utc>>,
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Creates a new food entry with associated user data.
#[utoipa::path(
        post,
        path = "food/create",
        tag = "create food entry",
        responses(
            (status = 201, description = "food entry created successfully"),
        )
    )]
#[axum::debug_handler]
#[instrument]
pub async fn create_food_entry(
    State(food_state): State<Arc<FoodState>>,
    Json(request): Json<CreateFoodEntryRequest>,
) -> Result<StatusCode, YuhuhError> {
    Err(YuhuhError::NotImplemented)
}
