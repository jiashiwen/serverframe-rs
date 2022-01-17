use crate::httpserver::handlers::{root, tpost};

use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{BoxError, Router};
use std::time::Duration;
use tower::{ServiceBuilder, ServiceExt};
// use tower_http::auth::RequireAuthorizationLayer;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

pub fn router_root() -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(HandleErrorLayer::new(handle_timeout_error))
        .layer(tower::timeout::TimeoutLayer::new(Duration::from_secs(2)))
        // .timeout(Duration::from_secs(2))
        // .layer(RequireAuthorizationLayer::basic("test", "passwd"))
        .into_inner();

    let root = Router::new()
        .route("/health", get(root))
        .route("/health", post(root));

    let api = Router::new()
        .route("/v1/tpost", post(tpost))
        .layer(middleware_stack);
    return root.nest("/api", api);
    // let router = root.merge(api);
    // return router;
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}
