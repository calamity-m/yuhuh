//! read mood entries HTTP handler
//!
//! This module provides HTTP endpoints for reading mood entries.

// ============================================================================
// Request/Response Types
// ============================================================================

use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    mood::{model::MoodEntry, state::MoodState},
    user::state::UserState,
};

// ============================================================================
// HTTP Request types
// ============================================================================

/// Request parameters for finding mood entries.
#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct ReadMoodEntriesRequest {
    /// user ID to search by.
    pub user_id: Uuid,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub logged_before_date: Option<DateTime<Utc>>,
    pub logged_after_date: Option<DateTime<Utc>>,
}

// ============================================================================
// HTTP Responsed types
// ============================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReadMoodEntriesResponse {
    pub found_entries: Vec<MoodEntry>,
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Find mood entries for a user
#[utoipa::path(
    get,
    path = "mood", 
    tag = "mood", 
    params(ReadMoodEntriesRequest),
    responses(
        (status = 200, description = "Found mood entries", body = ReadMoodEntriesResponse)
))]
#[instrument]
pub async fn read_mood_entries(
    State(mood_state): State<Arc<MoodState>>,
    State(user_state): State<Arc<UserState>>,
    Query(request): Query<ReadMoodEntriesRequest>,
) -> Result<(StatusCode, Json<ReadMoodEntriesResponse>), YuhuhError> {
    debug!("entering read_mood_entries");

    let offset = request.offset.unwrap_or(0);
    let limit = request.limit.unwrap_or(10000);
    debug!(offset=?offset, limit=?limit, "calculated offset and limit");

    if (user_state
        .find_user_repo
        .find_user_by_id(&request.user_id)
        .await?)
        .is_none()
    {
        error!(user_id = ?request.user_id, "failed to find user");
        return Err(YuhuhError::BadRequest("user not found".to_string()));
    }

    let records = mood_state
        .read_mood_entries_repo
        .find_mood_entries(
            &request.user_id,
            request.logged_after_date,
            request.logged_after_date,
            limit.into(),
            offset.into(),
        )
        .await?;

    Ok((
        StatusCode::OK,
        Json(ReadMoodEntriesResponse {
            found_entries: records,
        }),
    ))
}
