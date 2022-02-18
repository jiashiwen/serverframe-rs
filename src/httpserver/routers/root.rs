use crate::httpserver::handlers::{
    current_config, login, raw_flush, raw_get, raw_put, raw_scan, root, tpost,
};
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{BoxError, Router};
use std::thread::current;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::auth::RequireAuthorizationLayer;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

pub fn router_root() -> Router {
    // let onreq = DefaultOnRequest::new().level(Level::INFO);
    let tracer = TraceLayer::new_for_http();
    // let tracer = tracer.on_request(onreq);
    let middleware_stack = ServiceBuilder::new()
        .layer(tracer)
        .layer(CompressionLayer::new())
        .layer(HandleErrorLayer::new(handle_timeout_error))
        .layer(tower::timeout::TimeoutLayer::new(Duration::from_secs(2)))
        // .layer(TraceLayer::new_for_http())
        // .timeout(Duration::from_secs(2))
        .layer(RequireAuthorizationLayer::basic("test", "passwd"))
        .into_inner();

    let root = Router::new()
        .route("/login", post(login))
        .route("/health", get(root))
        .route("/health", post(root));

    let api = Router::new()
        .route("/v1/tpost", post(tpost))
        .route("/v1/raw/put", post(raw_put))
        .route("/v1/raw/get", post(raw_get))
        .route("/v1/raw/flushall", post(raw_flush))
        .route("/v1/raw/scan", post(raw_scan))
        .route("/v1/currentconfig", post(current_config))
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
