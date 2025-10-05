//! Package for common test code that applies to yuhuh

#[allow(dead_code)]
pub mod common {
    use std::sync::Arc;

    use axum::Router;
    use sqlx::PgPool;
    use tracing::Span;

    pub async fn setup() -> (Router, PgPool) {
        let db = testutil::get_test_db_instance().await;
        let app = crate::api::new_app(
            &crate::config::Config::default(),
            db.clone(),
            Arc::new(Span::current()),
        );

        crate::migrations::run_migrations_with_db(db.clone())
            .await
            .expect("db migrations successful");

        (app, db)
    }
}
