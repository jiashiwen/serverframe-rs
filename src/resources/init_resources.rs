use crate::errors::{GlobalError, GlobalErrorType};
use crate::resources::tikv::TiKVHandler;
use anyhow::Result;
use async_once::AsyncOnce;
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
    // static ref GLOBAL_TiKV: Mutex<TiKVHandler> = {
    //    let handler =tokio::runtime::Runtime::new().unwrap().block_on(async {
    //         // let global_TiKV = TiKVHandler::default().await;
    //         let endpoint= GLOBAL_PD_ENDPOINT.lock().unwrap().to_vec();
    //         let pd: Vec<&str> = endpoint.iter().map(|s| &**s).collect();
    //         let global_TiKV = TiKVHandler::new(pd).await;
    //         global_TiKV
    // });
    // Mutex::new(handler)
    // };
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

// pub fn set_tikv(endpoint: Vec<&str>) {
//     let handler = tokio::runtime::Runtime::new().unwrap().block_on(async {
//         let tikv = TiKVHandler::new(endpoint).await;
//         tikv
//     });
//     GLOBAL_TiKV.lock().unwrap().set_self(handler);
// }

pub async fn get_tikv_handler() -> &'static TiKVHandler {
    GLOBAL_TiKV.get().await
}
