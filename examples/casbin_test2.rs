use anyhow::Result;
use casbin::prelude::*;
use casbin::MemoryAdapter;
use std::borrow::{Borrow, BorrowMut};

use futures::lock::Mutex;
use futures::AsyncWriteExt;
use futures_locks::RwLock;
use http_body::Body;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::sync::Arc;

// static GLOBALE_CASBIN_ENFORCER: OnceCell<Mutex<CasbinEnforcer>> = OnceCell::new();
static GLOBALE_CASBIN_ENFORCER: OnceCell<RwLock<CasbinEnforcer>> = OnceCell::new();
// static GLOBALE_CASBIN_ENFORCER: Lazy<RwLock<CasbinEnforcer>> = Lazy::new(|| {
//     let enforcer = CasbinEnforcer::default().await;
//     RwLock::new(enforcer)
// });

pub struct CasbinEnforcer {
    // enforcer: Arc<RwLock<Enforcer>>,
    enforcer: Enforcer,
}

impl CasbinEnforcer {
    pub async fn default() -> Self {
        // let rt = tokio::runtime::Runtime::new().unwrap();
        // let enforcer = rt.block_on(async {
        /*加载模型文件*/
        let m = DefaultModel::from_file("./rbac_with_domains_model.conf")
            .await
            .unwrap()
            /*初始化适配器*/;
        // let a = CICIAdapter::new(init_rbatis().await);
        let a = MemoryAdapter::default();
        // let enforcer = Enforcer::new(m, "./rbac_with_domains_policy.csv")
        let mut enforcer = Enforcer::new(m, a).await.unwrap();
        let p = vec![
            "jsw".to_string(),
            "domain1".to_string(),
            "data2".to_string(),
            "read".to_string(),
        ];

        enforcer.add_policy(p).await.unwrap();
        /* 添加自定义验证方法 */
        // cached_enforcer.add_function("ciciMatch", cici_match);
        //     enforcer
        // });
        Self {
            // enforcer: Arc::new(RwLock::new(enforcer)),
            enforcer,
        }
    }

    pub async fn addpolice(&mut self, p: Vec<String>) -> casbin::Result<bool> {
        self.enforcer.add_policy(p).await
    }
}

pub async fn init_casbin() {
    let enforcer = CasbinEnforcer::default().await;
    // GLOBALE_CASBIN_ENFORCER.get_or_init(&enforcer).await;
    // GLOBALE_CASBIN_ENFORCER.set(Mutex::new(enforcer)).ok();

    GLOBALE_CASBIN_ENFORCER.set(RwLock::new(enforcer)).ok();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_casbin().await;

    let args = vec![
        "jsw".to_string(),
        "domain3".to_string(),
        "data2".to_string(),
        "read".to_string(),
    ];

    // let r = GLOBALE_CASBIN_ENFORCER
    //     .get()
    //     .unwrap()
    //     .borrow_mut()
    //     .lock()
    //     .await
    //     .addpolice(args.clone())
    //     .await;
    // println!("{:?}", r);
    // let en = GLOBALE_CASBIN_ENFORCER
    //     .get()
    //     .unwrap()
    //     .lock()
    //     .await
    //     .enforcer
    //     .enforce(args);
    // println!("{:?}", en);
    let r = GLOBALE_CASBIN_ENFORCER
        .get()
        .unwrap()
        .write()
        .await
        .addpolice(args.clone())
        .await;

    println!("{:?}", r);
    // let en = GLOBALE_CASBIN_ENFORCER
    //     .get()
    //     .unwrap()
    //     .lock()
    //     .await
    //     .enforcer
    //     .enforce(args);
    // println!("{:?}", en);
    Ok(())
}
