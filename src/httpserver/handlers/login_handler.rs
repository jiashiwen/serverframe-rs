use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::{Response, Token, User};
use crate::privilege::get_user_by_name;
use axum::Json;

pub async fn login(Json(payload): Json<User>) -> HandlerResult<Token> {
    let user = get_user_by_name(payload.username.clone());

    match user {
        Ok(u) => {
            if payload.password.eq(u.password.as_str()) {
                let t = u.gen_token();
                return match t {
                    Ok(str) => {
                        let token = Token { token: str };
                        Ok(Json(Response::ok(token)))
                    }
                    Err(e) => {
                        let err = AppError {
                            message: Some(e.to_string()),
                            cause: None,
                            error_type: AppErrorType::UnknowErr,
                        };
                        Err(err)
                    }
                };
            }
            Ok(Json(Response::err(123, "password error".to_string())))
        }
        Err(e) => {
            let err = AppError {
                message: Some(e.to_string()),
                cause: None,
                error_type: AppErrorType::UnknowErr,
            };
            return Err(err);
        }
    }
}
