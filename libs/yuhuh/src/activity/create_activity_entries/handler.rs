// ============================================================================
// HTTP Request Types
// ============================================================================

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    activity::{
        model::{ActivityEntry, ActivityType},
        state::ActivityState,
    },
    error::YuhuhError,
    user::state::UserState,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateActivityEntryRequest {
    pub user_id: Uuid,
    pub mood_entries: Vec<NewActivityEntry>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewActivityEntry {
    pub activity: String,
    pub activity_type: ActivityType,
    #[schema(value_type = Object)]
    pub activity_info: serde_json::Value,
    pub logged_at: Option<DateTime<Utc>>,
}

// ============================================================================
// Implementations
// ============================================================================

impl NewActivityEntry {
    pub fn into(self, user_id: Uuid) -> ActivityEntry {
        ActivityEntry {
            activity_record_id: None,
            user_id,
            created_at: self.logged_at,
            updated_at: None,
            activity: self.activity,
            activity_type: self.activity_type,
            activity_info: self.activity_info,
            logged_at: self.logged_at.unwrap_or(Utc::now()),
        }
    }
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Create activity entries for a user
#[utoipa::path(
        post,
        path = "activity",
        tag = "activity",
        responses(
            (status = 201, description = "activity entries created successfully"),
        )
    )]
#[instrument]
pub async fn create_activity_entries(
    State(activity_state): State<Arc<ActivityState>>,
    State(user_state): State<Arc<UserState>>,
    Json(request): Json<CreateActivityEntryRequest>,
) -> Result<StatusCode, YuhuhError> {
    if (user_state
        .find_user_repo
        .find_user_by_id(&request.user_id)
        .await?)
        .is_none()
    {
        return Err(YuhuhError::NotFound("user not found".to_string()));
    }

    activity_state
        .create_activity_entries_repo
        .create_activity_entries(
            request
                .mood_entries
                .into_iter()
                .map(|f| f.into(request.user_id))
                .collect(),
        )
        .await?;

    Ok(StatusCode::CREATED)
}
