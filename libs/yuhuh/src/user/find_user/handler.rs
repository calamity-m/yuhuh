use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use tracing::{debug, info, info_span, instrument};
use uuid::Uuid;

use crate::{error::CoreError, user::state::UserState};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {}

#[derive(Debug, Deserialize)]
pub struct FindUserRequest {
    id: Option<Uuid>,
    username: Option<String>,
}

#[instrument]
pub async fn find_user(
    Query(request): Query<FindUserRequest>,
    State(user_state): State<Arc<UserState>>,
) -> Result<Json<User>, CoreError> {
    debug!("entered find_user - request: {:?}", request);

    Ok(Json(User {}))
}
