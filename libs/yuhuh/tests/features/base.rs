//! Base tests that cover the yuhuh server rather than
//! any specific functionality.

#[cfg(test)]
mod tests {

    use http_body_util::BodyExt;
    use pretty_assertions::assert_eq;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };

    use serde_json::{Value, json};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_not_found_handler() {
        let (app, _db) = crate::common::setup().await;

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Not found as / hosts nothing
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response
            .into_body()
            .into_data_stream()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            json!({ "error": "404", "reason": "no matching route found" })
        );
    }
}
