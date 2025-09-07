use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;
use tracing::Span;

pub async fn setup() -> (Router, PgPool) {
    let db = testutil::get_test_db_instance().await;
    let app = yuhuh::api::new_app(
        &yuhuh::config::Config::default(),
        db.clone(),
        Arc::new(Span::current()),
    );

    yuhuh::migrations::run_migrations_with_db(db.clone())
        .await
        .expect("db migrations successful");

    (app, db)
}
