use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

use crate::httpserver::exception::{Error_msg, Response_Error};
use crate::httpserver::module::{CreateUser, User};
use serde_json::{json, Value};

pub async fn tpost(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here

    if !payload.username.eq("abc") {
        let mut err_msg = Error_msg::new();
        err_msg.set_error_code(5);
        err_msg.set_msg("555 not ok".to_string());
        let re = Response_Error::new(err_msg);

        return Json(json!(re));
    }
    let user = User {
        id: 1337,
        username: payload.username,
    };

    return Json(json!(user));
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    // (StatusCode::OK, Json(user))
}
