use crate::errors::{GlobalError, GlobalErrorType};
use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::module::KV;
use crate::resources::get_tikv_handler;
use anyhow::Error;
use anyhow::Result;
use futures::TryFutureExt;
use tikv_client::Value;

pub async fn s_raw_put(put: KV) -> Result<()> {
    let tikvhandler = get_tikv_handler().await;
    tikvhandler
        .raw_put(put.Key, put.Value)
        .map_err(|e| {
            return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
        })
        .await?;
    Ok(())
}

pub async fn s_raw_get(key: String) -> Result<String> {
    let tikvhandler = get_tikv_handler().await;
    let result = tikvhandler.raw_get(key).await.map_err(|e| {
        return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    })?;

    match result {
        None => {
            return Err(Error::from(GlobalError::from_err(
                "no reault".to_string(),
                GlobalErrorType::UnknowErr,
            )));
        }
        Some(val) => {
            let str = String::from_utf8(val).map_err(|e| {
                return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
            })?;
            Ok(str)
        }
    }
    // if let Some(val) = result.unwrap() {
    //     println!("get key:{},value is:{:?}", key, String::from_utf8(val));
    //     let str = String::from_utf8(val).map_err(|e| {
    //         return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    //     })?;
    //     Ok(str)
    // }
    // let result = tikvhandler
    //     .raw_get(key)
    //     .map_err(|e| {
    //         return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    //     })
    //     .await?;

    // return Err(Error::from(GlobalError::from_err(
    //     "".to_string(),
    //     GlobalErrorType::UnknowErr,
    // )));
}
