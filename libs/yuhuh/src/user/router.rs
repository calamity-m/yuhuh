use axum::{Router, routing::get};

use crate::{state::AppState, user::find_user::handler::find_user};

pub fn user_router() -> Router<AppState> {
    Router::new().route("/user", get(find_user))
}
