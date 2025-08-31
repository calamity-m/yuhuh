use async_trait::async_trait;
use tracing::info;

use crate::{error::YuhuhError, user::model::User};

#[async_trait]
pub trait FindUserRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn find_user(&self, id: &uuid::Uuid) -> Result<User, YuhuhError>;
}

#[derive(Debug)]
pub struct DummyFindUserRepository {}

#[async_trait]
impl FindUserRepository for DummyFindUserRepository {
    async fn find_user(&self, id: &uuid::Uuid) -> Result<User, YuhuhError> {
        info!(id = ?id, "called with id");
        todo!()
    }
}
