use crate::errors::{GlobalError, GlobalErrorType};
use crate::resources::tikv::TiKVHandler;
use anyhow::Result;
use std::sync::{Mutex, MutexGuard};

lazy_static::lazy_static! {
    static ref GLOBAL_TiKV: Mutex<TiKVHandler> = {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let global_TiKV = TiKVHandler::default().await;
            Mutex::new(global_TiKV)
    })};
}

pub fn set_tikv(endpoint: Vec<&str>) {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let tikv = TiKVHandler::new(endpoint).await;
        GLOBAL_TiKV.lock().unwrap().set_self(tikv);
    });
    // let tikv = TiKVHandler::new(endpoint).await;
    // GLOBAL_TiKV.lock().unwrap().set_self(tikv);
}

pub fn get_tikv_handler() -> Result<MutexGuard<'static, TiKVHandler>> {
    let handler = GLOBAL_TiKV.lock().map_err(|e| {
        return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    })?;
    Ok(handler)
}
