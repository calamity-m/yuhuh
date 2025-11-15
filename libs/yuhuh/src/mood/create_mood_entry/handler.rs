//! create mood entries HTTP handler
//!
//! This module provides HTTP endpoints for creating mood entries.

// ============================================================================
// Request/Response Types
// ============================================================================

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use serde::{Deserialize, Serialize};
use tracing::{debug, error, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    mood::{model::MoodEntry, rating::Rating, state::MoodState},
    user::state::UserState,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateMoodEntryRequest {
    pub user_id: Uuid,
    pub notes: Option<String>,
    pub mood: Option<Rating>,
    pub energy: Option<Rating>,
    pub sleep: Option<Rating>,
}

// ============================================================================
// HTTP Request types
// ============================================================================

/// Create mood entry for a user
#[utoipa::path(
    post,
    path = "mood", 
    tag = "mood", 
    responses(
        (status = 201, description = "mood created successfully")
))]
#[instrument]
pub async fn create_mood_entry(
    State(mood_state): State<Arc<MoodState>>,
    State(user_state): State<Arc<UserState>>,
    Json(request): Json<CreateMoodEntryRequest>,
) -> Result<StatusCode, YuhuhError> {
    if (user_state
        .find_user_repo
        .find_user_by_id(&request.user_id)
        .await?)
        .is_none()
    {
        error!(user_id = ?request.user_id, "failed to find user");
        return Err(YuhuhError::BadRequest("user not found".to_string()));
    }

    let m = MoodEntry {
        mood_record_id: None,
        user_id: request.user_id,
        created_at: None,
        updated_at: None,
        mood: request.mood,
        energy: request.energy,
        sleep: request.sleep,
        notes: request.notes,
    };

    debug!(entry=?m, "creating mood entry");

    mood_state
        .create_mood_entry_repo
        .create_mood_entry(m)
        .await?;

    debug!("successfully created mood entry");

    Ok(StatusCode::CREATED)
}
