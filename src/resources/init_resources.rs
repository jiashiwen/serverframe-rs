use crate::configure::get_config;
use crate::errors::{GlobalError, GlobalErrorType};
use crate::resources::tikv::TiKVHandler;
use anyhow::Result;
use async_once::AsyncOnce;
use log::info;
use std::borrow::{Borrow, BorrowMut};
use std::sync::{Mutex, MutexGuard};

lazy_static::lazy_static! {
    static ref GLOBAL_PD_ENDPOINT: Mutex<Vec<String>> = Mutex::new(vec![]);

    static ref GLOBAL_TiKV: AsyncOnce<TiKVHandler> = AsyncOnce::new(async {
        let endpoint= GLOBAL_PD_ENDPOINT.lock().unwrap().to_vec();
        let pd: Vec<&str> = endpoint.iter().map(|s| &**s).collect();
        let global_TiKV = TiKVHandler::new(pd).await;
        global_TiKV
    });
}

pub fn init_resources() -> Result<()> {
    let cfg = get_config()?;
    let pd: Vec<&str> = cfg.tikv.pdaddrs.iter().map(|s| &**s).collect();
    //配置tikv
    set_tikv(pd);
    //tikv连接初始化
    let tikvhandler = get_tikv_handler();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        tikvhandler.await.raw_get("t".to_string()).await;
    });
    // tikvhandler.raw_get(key).await.map_err(|e| {
    //     return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    // })?;
    Ok(())
}

pub fn set_tikv(endpoint: Vec<&str>) {
    if endpoint.is_empty() {
        GLOBAL_PD_ENDPOINT
            .lock()
            .unwrap()
            .push("127.0.0.1:2379".to_string());
        return;
    }

    for str in endpoint {
        GLOBAL_PD_ENDPOINT.lock().unwrap().push(String::from(str));
    }
}

pub async fn get_tikv_handler() -> &'static TiKVHandler {
    GLOBAL_TiKV.get().await
}
