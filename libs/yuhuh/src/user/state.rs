use std::{fmt::Debug, sync::Arc};

use sqlx::PgPool;

use super::find_user::repository::*;

#[derive(Debug)]
pub struct UserState {
    pub find_user_repo: Arc<dyn FindUserRepository>,
}

impl UserState {
    pub fn new(find_user_repo: Arc<dyn FindUserRepository>) -> Self {
        UserState { find_user_repo }
    }
}

pub fn create_user_state(db: PgPool) -> UserState {
    UserState {
        find_user_repo: Arc::new(FindUserRepositoryImpl { db: db.clone() }),
    }
}
