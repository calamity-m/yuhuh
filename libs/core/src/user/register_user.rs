use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

use crate::{error::CoreError, user::model::user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    username: String,
}

#[instrument]
pub async fn register_user(Json(user): Json<RegisterUser>) -> Result<Json<User>, CoreError> {
    debug!("entered register_user - {:?}", user);

    Err(CoreError::NotImplemented)
}
