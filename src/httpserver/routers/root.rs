use crate::httpserver::service::{root, tpost};
use axum::routing::{get, post};
use axum::Router;

pub fn router_root() -> Router {
    let root = Router::new()
        .route("/health", get(root))
        .route("/health", post(root));
    let api = Router::new().route("/api/tpost", post(tpost));
    return root.merge(api);
}

// async fn root() -> &'static str {
//     "OK!"
// }
