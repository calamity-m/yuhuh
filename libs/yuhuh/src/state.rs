use std::sync::Arc;

use axum::extract::FromRef;

use crate::{config::Config, user::state::*};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user: Arc<UserState>,
}

impl FromRef<AppState> for Arc<Config> {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}

impl FromRef<AppState> for Arc<UserState> {
    fn from_ref(input: &AppState) -> Self {
        input.user.clone()
    }
}
