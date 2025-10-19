use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;

use crate::{
    state::AppState,
    user::{create_user, find_user},
};

// =============================================================================
// API Docs
// =============================================================================

#[derive(OpenApi)]
#[openapi(paths(find_user::find_user, create_user::create_discord_user))]
pub struct UserApi;

// =============================================================================
// Router
// =============================================================================

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(find_user::find_user))
        .route(
            "/users/create/discord",
            post(create_user::create_discord_user),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{config::Config, state::create_app_state};

    use pretty_assertions::assert_eq;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };

    use tower::ServiceExt;

    #[tokio::test]
    async fn hello_world() {
        let app = user_router();

        let db = testutil::get_test_db_instance().await;

        crate::migrations::run_migrations_with_db(db.clone())
            .await
            .expect("db migrations successful");

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .with_state(create_app_state(&Config::default(), db))
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Not found as / hosts nothing
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
