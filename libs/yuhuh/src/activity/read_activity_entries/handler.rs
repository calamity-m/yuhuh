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
use serde_json::Value;
use tracing::{debug, error, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    activity::{
        self,
        model::{ActivityEntry, ActivityType},
        state::ActivityState,
    },
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
    pub activity_entries: Vec<FoundActivityRecord>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FoundActivityRecord {
    pub activity: String,
    pub activity_type: ActivityType,
    pub activity_info: Value,
    pub logged_at: DateTime<Utc>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl From<ActivityEntry> for FoundActivityRecord {
    fn from(record: ActivityEntry) -> Self {
        Self {
            activity: record.activity,
            activity_type: record.activity_type,
            activity_info: record.activity_info,
            logged_at: record.logged_at,
        }
    }
}

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

    let offset = request.offset.unwrap_or(0);
    let limit = request.limit.unwrap_or(10000);
    debug!(offset=?offset, limit=?limit, "calculated offset and limit");

    let activity_records = activity_state
        .read_activity_entries_repo
        .read_activity_entries(
            &request.user_id,
            request.logged_before_date,
            request.logged_after_date,
            limit.into(),
            offset.into(),
        )
        .await?;

    let mapped = activity_records.into_iter().map(FoundActivityRecord::from);

    Ok((
        StatusCode::OK,
        Json(ReadActivityEntriesResponse {
            found_activity_entries: mapped.len() as u32,
            activity_entries: mapped.collect(),
        }),
    ))
}
