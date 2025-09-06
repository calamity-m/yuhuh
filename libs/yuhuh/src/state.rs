use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;
use tracing::debug;

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

pub fn create_app_state(config: &Config, db: PgPool) -> AppState {
    let app_state = AppState {
        config: Arc::new(config.clone()),
        user: Arc::new(create_user_state(db)),
    };

    debug!("created app state");

    app_state
}
