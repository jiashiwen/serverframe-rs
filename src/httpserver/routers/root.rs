use crate::httpserver::service::{root, tpost};
use axum::routing::{get, post};
use axum::Router;
use std::time::Duration;

pub fn router_root() -> Router {
    let middleware_stack = ServiceBuilder::new()
        // timeout all requests after 10 seconds
        .timeout(Duration::from_secs(10))
        // add high level tracing of requests and responses
        .layer(TraceLayer::new_for_http())
        // compression responses
        .layer(CompressionLayer::new())
        // convert the `ServiceBuilder` into a `tower::Layer`
        .into_inner();

    let root = Router::new()
        .layer(middleware_stack)
        .route("/health", get(root))
        .route("/health", post(root));
    let api = Router::new().route("/api/tpost", post(tpost));
    return root.merge(api);
}

// async fn root() -> &'static str {
//     "OK!"
// }
