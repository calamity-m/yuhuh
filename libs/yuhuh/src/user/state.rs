use std::{fmt::Debug, sync::Arc};

use sqlx::PgPool;

use crate::user::features::{
    create_user::{CreateUserRepository, CreateUserRepositoryImpl},
    find_user::{FindUserRepository, FindUserRepositoryImpl},
};

#[derive(Debug)]
pub struct UserState {
    pub db: PgPool,
    pub create_user_repo: Arc<dyn CreateUserRepository>,
    pub find_user_repo: Arc<dyn FindUserRepository>,
}

impl UserState {
    pub fn new(db: PgPool) -> Self {
        UserState {
            db: db.clone(),
            create_user_repo: Arc::new(CreateUserRepositoryImpl { db: db.clone() }),
            find_user_repo: Arc::new(FindUserRepositoryImpl { db: db.clone() }),
        }
    }
}
