use crate::httpserver::handlers::{root, tpost};

use axum::routing::{get, post};
use axum::Router;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

pub fn router_root() -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .into_inner();

    let root = Router::new()
        .route("/health", get(root))
        .route("/health", post(root));
    let api = Router::new().route("/api/tpost", post(tpost));
    let router = root.layer(middleware_stack).merge(api);
    return router;
}

// async fn root() -> &'static str {
//     "OK!"
// }
