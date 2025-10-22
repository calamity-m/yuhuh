//! create assignemnts HTTP handler
//!
//! This module provides HTTP endpoints for create assignemnts.

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
        model::{EnergyAssignment, MoodAssignment, Rating, SleepAssignment},
        state::MoodState,
    },
    user::state::UserState,
};

// ============================================================================
// HTTP Request types
// ============================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct NewMoodAssignment {
    pub value: String,
    #[validate(range(min = 0, max = 10))]
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct NewEnergyAssignment {
    pub value: String,
    #[validate(range(min = 0, max = 10))]
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct NewSleepAssignemnt {
    pub value: String,
    #[validate(range(min = 0, max = 10))]
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateAssignmentsRequest {
    pub user_id: Uuid,
    pub mood_assignments: Vec<NewMoodAssignment>,
    pub energy_assignemnts: Vec<NewEnergyAssignment>,
    pub sleep_assignemnts: Vec<NewSleepAssignemnt>,
}

// ============================================================================
// HTTP Responsed types
// ============================================================================

// ============================================================================
// Trait Implementations
// ============================================================================

// =============================================================================
// Helpers
// =============================================================================
async fn validate_user(user_state: Arc<UserState>, user_id: &Uuid) -> Result<(), YuhuhError> {
    if (user_state.find_user_repo.find_user_by_id(user_id).await?).is_none() {
        error!(user_id = ?user_id, "failed to find user");
        return Err(YuhuhError::BadRequest("user not found".to_string()));
    }

    Ok(())
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Create assignments for a user
#[utoipa::path(
    post,
    path = "mood/assignments", 
    tag = "mood", 
    responses(
        (status = 201, description = "assignments created successfully")
))]
#[instrument]
pub async fn create_assignments(
    State(mood_state): State<Arc<MoodState>>,
    State(user_state): State<Arc<UserState>>,
    Json(request): Json<CreateAssignmentsRequest>,
) -> Result<StatusCode, YuhuhError> {
    // Pass request validation
    request.validate()?;

    // Ensure user exists
    validate_user(user_state, &request.user_id).await?;

    // have to search and see if our assignments' indexes already exist. If so, we'll
    // be updating them rather than creating new ones. For those in particular, anyway.
    //
    // If we fail here, well shit. Recovery isn't fun nor easy, but at least with this
    // design we can just re-run ad infinium until it does work, or we handle it, or
    // some shit.

    // Handle moods
    for requested in request.mood_assignments {
        let rating = Rating::new(requested.index)
            .ok_or_else(|| YuhuhError::BadRequest("mood rating out of range".to_string()))?;
        let mut m = MoodAssignment {
            mood_assignment_id: None,
            user_id: request.user_id,
            val: requested.value,
            idx: rating,
        };

        debug!(mood_assignment=?m, user_id=?request.user_id, "started processing mood assignment");

        let found = mood_state
            .read_assignments_repo
            .read_mood_assigment_index(&request.user_id, rating)
            .await?;

        if let Some(existing) = found {
            m.mood_assignment_id = existing.mood_assignment_id;
        }

        mood_state
            .create_assignemnts_repo
            .upsert_mood_assignment(m)
            .await
            .map_err(|e| {
                error!(error = ?e, "failed to upsert mood assignment");

                YuhuhError::ContextError {
                    context: "failed processing mood assignment, please try again with all"
                        .to_string(),
                    error: Box::new(e),
                }
            })?;

        debug!("finished processing mood assignment");
    }

    // Handle energies
    for requested in request.energy_assignemnts {
        let rating = Rating::new(requested.index)
            .ok_or_else(|| YuhuhError::BadRequest("energy rating out of range".to_string()))?;

        let mut e = EnergyAssignment {
            energy_assignment_id: None,
            user_id: request.user_id,
            val: requested.value,
            idx: rating,
        };

        debug!(mood_assignment=?e, user_id=?request.user_id, "started processing energy assignment");

        let found = mood_state
            .read_assignments_repo
            .read_energy_assigment_index(&request.user_id, rating)
            .await?;

        if let Some(existing) = found {
            e.energy_assignment_id = existing.energy_assignment_id;
        }

        mood_state
            .create_assignemnts_repo
            .upsert_energy_assignment(e)
            .await
            .map_err(|e| {
                error!(error = ?e, "failed to upsert energy assignment");

                YuhuhError::ContextError {
                    context: "failed processing energy assignment, please try again with all"
                        .to_string(),
                    error: Box::new(e),
                }
            })?;

        debug!("finished processing energy assignment");
    }

    // Handle sleeps
    for requested in request.sleep_assignemnts {
        let rating = Rating::new(requested.index)
            .ok_or_else(|| YuhuhError::BadRequest("energy rating out of range".to_string()))?;

        let mut s = SleepAssignment {
            sleep_assignment_id: None,
            user_id: request.user_id,
            val: requested.value,
            idx: rating,
        };

        debug!(mood_assignment=?s, user_id=?request.user_id, "started processing sleep assignment");

        let found = mood_state
            .read_assignments_repo
            .read_sleep_assigment_index(&request.user_id, rating)
            .await?;

        if let Some(existing) = found {
            s.sleep_assignment_id = existing.sleep_assignment_id;
        }

        mood_state
            .create_assignemnts_repo
            .upsert_sleep_assignment(s)
            .await
            .map_err(|e| {
                error!(error = ?e, "failed to upsert sleep assignment");

                YuhuhError::ContextError {
                    context: "failed processing sleep assignment, please try again with all"
                        .to_string(),
                    error: Box::new(e),
                }
            })?;

        debug!("finished processing sleep assignment");
    }

    Ok(StatusCode::CREATED)
}
