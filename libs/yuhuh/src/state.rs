use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;
use tracing::debug;

use crate::{
    activity::state::ActivityState, config::Config, food::state::FoodState, mood::state::MoodState,
    user::state::*,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user: Arc<UserState>,
    pub food: Arc<FoodState>,
    pub mood: Arc<MoodState>,
    pub activity: Arc<ActivityState>,
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

impl FromRef<AppState> for Arc<FoodState> {
    fn from_ref(input: &AppState) -> Self {
        input.food.clone()
    }
}

impl FromRef<AppState> for Arc<MoodState> {
    fn from_ref(input: &AppState) -> Self {
        input.mood.clone()
    }
}

impl FromRef<AppState> for Arc<ActivityState> {
    fn from_ref(input: &AppState) -> Self {
        input.activity.clone()
    }
}

pub fn create_app_state(config: &Config, db: PgPool) -> AppState {
    let app_state = AppState {
        config: Arc::new(config.clone()),
        user: Arc::new(UserState::new(db.clone())),
        food: Arc::new(FoodState::new(db.clone())),
        mood: Arc::new(MoodState::new(db.clone())),
        activity: Arc::new(ActivityState::new(db.clone())),
    };

    debug!("created app state");

    app_state
}
