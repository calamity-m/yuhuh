use std::{fmt::Debug, sync::Arc};

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
