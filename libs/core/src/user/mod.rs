use std::sync::Arc;

use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::{AppState, user::find_user::service::FindUserService};

pub mod find_user;
pub mod register_user;
pub mod model {
    pub mod user;
}

#[derive(Clone, Debug)]
pub struct UserState {
    find_user: FindUserService,
}

impl UserState {
    pub fn new(pool: PgPool) -> Self {
        UserState {
            find_user: FindUserService::new(pool),
        }
    }
}

impl FromRef<UserState> for FindUserService {
    fn from_ref(input: &UserState) -> Self {
        input.find_user.clone()
    }
}

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/user/id/{id}", get(find_user::handler::find_user_id))
        .route(
            "/user/name/{username}",
            get(find_user::handler::find_user_uname),
        )
        .route("/user/register", post(register_user::register_user))
}
