use async_trait::async_trait;

use crate::error::CoreError;

#[async_trait]
pub trait FindUserRepository: std::fmt::Debug + Send + Sync + 'static {
    fn find_user(self, id: &uuid::Uuid) -> Result<String, CoreError>;
}

#[derive(Debug)]
pub struct DummyFindUserRepository {}

impl FindUserRepository for DummyFindUserRepository {
    fn find_user(self, id: &uuid::Uuid) -> Result<String, CoreError> {
        todo!()
    }
}
