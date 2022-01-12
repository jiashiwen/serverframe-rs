// async fn tpost() -> &'static str {
//     "tpost!"
// }

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::httpserver::module::{CreateUser, User};

pub async fn tpost(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
