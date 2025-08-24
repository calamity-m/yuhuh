use axum::{
    Router,
    routing::{get, post},
};

use crate::{AppState, user::find_user::find_user};

pub mod find_user;
pub mod model {
    pub mod user;
}

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/user/{id}", get(find_user::find_user))
        .route("/user/test/get", post(find_user::test))
}
