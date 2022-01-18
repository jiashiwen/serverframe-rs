use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::thread::sleep;
use std::time::Duration;

use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::{CreateUser, Response, User};
use serde_json::{json, Value};

// async fn root() -> &'static str {
//     "OK!"
// }

pub async fn root() -> HandlerResult<Value> {
    // Json(json!({"health":"ok"}))
    Ok(Json(Response::ok(json!({"health":"ok"}))))
}

// pub async fn tpost(Json(payload): Json<CreateUser>) -> impl IntoResponse {
pub async fn tpost(Json(payload): Json<CreateUser>) -> HandlerResult<User> {
    if !payload.username.eq("abc") {
        let err = AppError {
            message: Some("555 not ok".to_string()),
            cause: None,
            error_type: AppErrorType::unknowErr,
        };
        log::error!("{}", err);

        return Err(err);
    }
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // return Json(json!(user));
    Ok(Json(Response::ok(user)))
}
