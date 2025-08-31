use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;
use tracing::{debug, info, instrument};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    user::{model::User, state::UserState},
};

#[derive(Debug, Deserialize)]
pub struct FindUserRequest {
    id: Option<Uuid>,
}

/// Handles a request to find a user by ID.
///
/// This endpoint extracts query parameters and application state,
/// attempts to look up the user in the repository, and returns
/// the user as JSON if found.
#[instrument]
pub async fn find_user(
    Query(request): Query<FindUserRequest>,
    State(user_state): State<Arc<UserState>>,
) -> Result<Json<User>, YuhuhError> {
    // Log at debug level that we've entered the function, along with the request contents
    debug!("entered find_user - request: {:?}", request);

    // Otherwise, match on the ID
    match request.id {
        Some(id) => {
            // Query the repository to find the user by ID
            let user = user_state.find_user_repo.find_user(&id).await?;

            info!(user = ?user, "found user");

            // Return the user as JSON in the response
            Ok(Json(user))
        }
        // If the request didn't contain an ID, return a BadRequest error
        None => return Err(YuhuhError::BadRequest("missing query".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    mod unit_tests {
        use super::*;

        use crate::user::find_user::repository::FindUserRepository;
        use async_trait::async_trait;

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
            let _query = Query(FindUserRequest { id: None });
            let _user_state = State(Arc::new(UserState {
                find_user_repo: Arc::new(TestFindUserRepository {}),
            }));

            let result = find_user(_query, _user_state).await;

            assert!(result.is_err());

            self::assert_eq!(
                result.err(),
                Some(YuhuhError::BadRequest("missing query".to_string()))
            );
        }
    }

    mod integration_tests {
        #[tokio::test]
        async fn test_find_user() {
            let db = testutil::get_test_db_static();
        }
    }
}
