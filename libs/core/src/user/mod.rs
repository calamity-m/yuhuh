use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

pub mod find_user;
pub mod register_user;
pub mod model {
    pub mod user;
}

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/user/id/{id}", get(find_user::find_user_id))
        .route("/user/name/{username}", get(find_user::find_user_uname))
        .route("/user/register", post(register_user::register_user))
}
