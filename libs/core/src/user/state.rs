use std::sync::Arc;

use async_trait::async_trait;

use crate::error::CoreError;

pub struct UserState {
    find_user_repo: Arc<dyn FindUserRepository + Send>,
}

impl UserState {
    pub fn new(find_user_repo: Arc<dyn FindUserRepository + Send>) -> Self {
        UserState { find_user_repo }
    }
}

#[async_trait]
pub trait FindUserRepository: Send + Sync + 'static {
    fn find_user(self, id: &uuid::Uuid) -> Result<String, CoreError>;
}

pub struct DummyFindUserRepository {}

impl FindUserRepository for DummyFindUserRepository {
    fn find_user(self, id: &uuid::Uuid) -> Result<String, CoreError> {
        todo!()
    }
}
