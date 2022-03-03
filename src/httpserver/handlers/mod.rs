mod config;
mod login_handler;
mod rawkv_handler;

mod authhandler;
mod tpost;
mod user_handler;

use axum::Json;
pub use config::current_config;
pub use login_handler::login;
pub use rawkv_handler::raw_flush;
pub use rawkv_handler::raw_get;
pub use rawkv_handler::raw_put;
pub use rawkv_handler::raw_scan;
pub use tpost::root;
pub use user_handler::get_headers;
pub use user_handler::get_user;
pub use user_handler::remove_user;
pub use user_handler::user_create;

use crate::httpserver::module::Response;

type HandlerResult<T> = crate::httpserver::module::Result<Json<Response<T>>>;
