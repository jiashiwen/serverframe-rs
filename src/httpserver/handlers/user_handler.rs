use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::{Response, User, UserName, ID};
use crate::httpserver::service::{s_get_user, s_remove_user, s_user_create};
use crate::privilege::User as PrivilegeUser;
use axum::Json;

pub async fn user_create(Json(u): Json<User>) -> HandlerResult<()> {
    let r = s_user_create(u);
    match r {
        Ok(_) => Ok(Json(Response::ok(()))),
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

pub async fn get_user(Json(u): Json<UserName>) -> HandlerResult<PrivilegeUser> {
    let user = s_get_user(u.name);
    match user {
        Ok(mut u) => {
            u.password = "".to_string();
            Ok(Json(Response::ok(u)))
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

pub async fn remove_user(Json(id): Json<ID>) -> HandlerResult<()> {
    let r = s_remove_user(id.id);
    match r {
        Ok(_) => Ok(Json(Response::ok(()))),
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
