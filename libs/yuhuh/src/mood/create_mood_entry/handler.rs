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
use validator::Validate;

use crate::{
    error::YuhuhError,
    mood::{
        model::{MoodEntry, Rating},
        state::MoodState,
    },
    user::state::UserState,
};

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateAssignmentsRequest {
    pub user_id: Uuid,
    pub notes: Option<String>,
    #[validate(range(min = 0, max = 10))]
    pub mood: Option<u32>,
    #[validate(range(min = 0, max = 10))]
    pub energy: Option<u32>,
    #[validate(range(min = 0, max = 10))]
    pub sleep: Option<u32>,
}

// ============================================================================
// HTTP Request types
// ============================================================================

/// Create assignments for a user
#[utoipa::path(
    post,
    path = "mood/assignments", 
    tag = "mood", 
    responses(
        (status = 201, description = "assignments created successfully")
))]
#[instrument]
pub async fn create_mood_entry(
    State(mood_state): State<Arc<MoodState>>,
    State(user_state): State<Arc<UserState>>,
    Json(request): Json<CreateAssignmentsRequest>,
) -> Result<StatusCode, YuhuhError> {
    request.validate()?;

    if (user_state
        .find_user_repo
        .find_user_by_id(&request.user_id)
        .await?)
        .is_none()
    {
        error!(user_id = ?request.user_id, "failed to find user");
        return Err(YuhuhError::BadRequest("user not found".to_string()));
    }

    let mood = match request.mood {
        Some(x) => Rating::new(x),
        None => None,
    };

    let energy = match request.energy {
        Some(x) => Rating::new(x),
        None => None,
    };

    let sleep = match request.sleep {
        Some(x) => Rating::new(x),
        None => None,
    };

    let m = MoodEntry {
        mood_record_id: None,
        user_id: request.user_id,
        created_at: None,
        updated_at: None,
        mood,
        energy,
        sleep,
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
