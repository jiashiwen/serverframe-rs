use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::Response;
use axum::Json;
use serde_json::{json, Value};

pub async fn root() -> HandlerResult<Value> {
    Ok(Json(Response::ok(json!({"health":"ok"}))))
}

// pub async fn tpost(Json(payload): Json<CreateUser>) -> HandlerResult<User> {
//     if !payload.username.eq("abc") {
//         let err = AppError {
//             message: Some("555 not ok".to_string()),
//             cause: None,
//             error_type: AppErrorType::UnknowErr,
//         };
//         log::error!("{}", err);
//
//         return Err(err);
//     }
//     let user = User {
//         username: payload.username,
//         password: "".to_string(),
//     };
//
//     Ok(Json(Response::ok(user)))
// }
