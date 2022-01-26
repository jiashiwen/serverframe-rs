use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::{Key, Response, KV};
use crate::httpserver::service::{s_raw_get, s_raw_put};
use crate::resources::get_tikv_handler;
use axum::Json;
use axum_debug::debug_handler;
use serde_json::{json, Map, Value};

#[axum_debug::debug_handler]
pub async fn raw_put(Json(payload): Json<KV>) -> HandlerResult<()> {
    if let Err(e) = s_raw_put(payload).await {
        let err = AppError {
            message: Some(e.to_string()),
            cause: None,
            error_type: AppErrorType::UnknowErr,
        };
        return Err(err);
    }
    Ok(Json(Response::ok(())))
}

pub async fn raw_get(Json(payload): Json<Key>) -> HandlerResult<String> {
    // let result = s_raw_get(payload.Key).await.map_err(|e| {
    //     let err = AppError {
    //         message: Some(e.to_string()),
    //         cause: None,
    //         error_type: AppErrorType::UnknowErr,
    //     };
    //     return Err(err);
    // })?;

    // if let Err(e) = s_raw_get(payload.Key).await {
    //     let err = AppError {
    //         message: Some(e.to_string()),
    //         cause: None,
    //         error_type: AppErrorType::UnknowErr,
    //     };
    //     return Err(err);
    // }

    let result = s_raw_get(payload.Key).await;
    match result {
        Ok(str) => Ok(Json(Response::ok(str))),
        Err(e) => {
            let err = AppError {
                message: Some(e.to_string()),
                cause: None,
                error_type: AppErrorType::UnknowErr,
            };
            return Err(err);
        }
    }
    // Ok(Json(Response::ok(result)))
}
