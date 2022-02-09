mod config;
mod rawkv_handler;
mod tpost;

use axum::Json;
pub use config::current_config;
pub use rawkv_handler::raw_flushall;
pub use rawkv_handler::raw_get;
pub use rawkv_handler::raw_put;
pub use rawkv_handler::raw_scan;
pub use tpost::root;
pub use tpost::tpost;

use crate::httpserver::module::Response;

type HandlerResult<T> = crate::httpserver::module::Result<Json<Response<T>>>;
