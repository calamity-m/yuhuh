use std::fmt::Debug;

use sqlx::PgPool;

#[derive(Debug)]
pub struct UserState {
    pub db: PgPool,
}

impl UserState {
    pub fn new(db: PgPool) -> Self {
        UserState { db }
    }
}

pub fn create_user_state(db: PgPool) -> UserState {
    UserState { db: db.clone() }
}
