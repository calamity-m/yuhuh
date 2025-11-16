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
    activity::state::ActivityState,
    error::YuhuhError,
    food::{model::FoodEntry, state::FoodState},
    user::state::UserState,
};

// ============================================================================
// HTTP Request types
// ============================================================================

/// Request parameters for finding a user.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ReadActivityEntriesRequest {
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
pub struct ReadActivityEntriesResponse {
    pub found_activity_entries: u32,
}

// ============================================================================
// Trait Implementations
// ============================================================================

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Find activity entries for a user
#[utoipa::path(
    get,
    path = "activity", 
    tag = "activity", 
    params(ReadActivityEntriesRequest),
    responses(
        (status = 200, description = "Found activity entries", body = ReadActivityEntriesResponse)
))]
#[instrument]
pub async fn read_activity_entries(
    State(activity_state): State<Arc<ActivityState>>,
    State(user_state): State<Arc<UserState>>,

    Query(request): Query<ReadActivityEntriesRequest>,
) -> Result<(StatusCode, Json<ReadActivityEntriesResponse>), YuhuhError> {
    debug!("entering read_activity_entries");

    if (user_state
        .find_user_repo
        .find_user_by_id(&request.user_id)
        .await?)
        .is_none()
    {
        return Err(YuhuhError::NotFound("user not found".to_string()));
    }

    Err(YuhuhError::NotImplemented)
}
