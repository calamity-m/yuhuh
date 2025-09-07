#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };

    use tower::ServiceExt;

    #[tokio::test]
    async fn hello_world() {
        let (app, _db) = crate::common::setup().await;

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Not found as / hosts nothing
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
