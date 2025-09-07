use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    user::{model::User, state::UserState},
};

#[derive(Debug, Deserialize)]
pub struct FindUserRequest {
    id: Option<Uuid>,
    discord_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct FindUserResponse {
    pub id: Uuid,
    pub personalisation: Option<String>,
    pub contact_email: Option<String>,
    pub contact_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub discord_id: Option<i64>,
    pub discord_username: Option<String>,
}

impl From<User> for FindUserResponse {
    fn from(user: User) -> Self {
        let (discord_id, discord_username) = user
            .discord_user
            .map(|d| (Some(d.discord_id), Some(d.username)))
            .unwrap_or((None, None));

        FindUserResponse {
            id: user.user_id,
            personalisation: user.personalisation.to_owned(),
            contact_email: user.contact_email,
            contact_name: user.contact_name,
            created_at: user.created_at,
            updated_at: user.updated_at,
            timezone: user.timezone,
            discord_id,
            discord_username,
        }
    }
}

pub struct DiscordUser {}

/// Handles a request to find a user by ID.
///
/// This endpoint extracts query parameters and application state,
/// attempts to look up the user in the repository, and returns
/// the user as JSON if found.
#[instrument]
pub async fn find_user(
    Query(request): Query<FindUserRequest>,
    State(user_state): State<Arc<UserState>>,
) -> Result<Json<FindUserResponse>, YuhuhError> {
    // Log at debug level that we've entered the function, along with the request contents
    debug!("entered find_user - request: {:?}", request);

    if let Some(id) = request.id {
        // Query the repository to find the user by ID
        let user = user_state.find_user_repo.find_user(&id).await?;

        info!(user = ?user, "found user");

        // Return the user as JSON in the response
        return Ok(Json(FindUserResponse::from(user)));
    }

    if let Some(discord_id) = request.discord_id {
        return Err(YuhuhError::NotImplemented);
    }

    Err(YuhuhError::BadRequest("missing query".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::user::find_user::repository::FindUserRepository;
    use async_trait::async_trait;
    use testutil::assert_variant;

    #[derive(Debug)]
    pub struct TestFindUserRepository {}

    #[async_trait]
    impl FindUserRepository for TestFindUserRepository {
        async fn find_user(&self, _id: &uuid::Uuid) -> Result<User, YuhuhError> {
            Err(YuhuhError::NotImplemented)
        }
    }

    #[tokio::test]
    async fn test_find_user_missing_query() {
        let _query = Query(FindUserRequest {
            id: None,
            discord_id: None,
        });
        let _user_state = State(Arc::new(UserState {
            find_user_repo: Arc::new(TestFindUserRepository {}),
        }));

        let result = find_user(_query, _user_state).await;

        assert!(result.is_err());

        assert_variant!(result.err().unwrap(), YuhuhError::BadRequest(_));
    }
}
