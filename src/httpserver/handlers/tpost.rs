use axum::http::StatusCode;
use axum::response::IntoResponse;
// use axum::response::Response;
use axum::Json;

use crate::httpserver::exception::{AppError, AppErrorType, Error_msg, Response_Error};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::{CreateUser, Response, User};
use serde_json::{json, Value};

// async fn root() -> &'static str {
//     "OK!"
// }

pub async fn root() -> Json<Value> {
    Json(json!({"health":"ok"}))
}

// pub async fn tpost(Json(payload): Json<CreateUser>) -> impl IntoResponse {
pub async fn tpost(Json(payload): Json<CreateUser>) -> HandlerResult<User> {
    if !payload.username.eq("abc") {
        // let mut err_msg = Error_msg::new();
        // err_msg.set_error_code(5);
        // err_msg.set_msg("555 not ok".to_string());
        let mut err = AppError {
            message: Some("555 not ok".to_string()),
            cause: None,
            error_type: AppErrorType::unknowErr,
        };
        // let re = Response_Error::new(err_msg);
        // return Json(json!(re));
        return Err(err);
    }
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // return Json(json!(user));
    Ok(Json(Response::ok(user)))
}
