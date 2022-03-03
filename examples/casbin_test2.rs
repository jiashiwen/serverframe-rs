use anyhow::Result;
use casbin::prelude::*;
use casbin::MemoryAdapter;
use futures_locks::RwLock;
use once_cell::sync::OnceCell;

static GLOBALE_CASBIN_ENFORCER: OnceCell<RwLock<CasbinEnforcer>> = OnceCell::new();

pub struct CasbinEnforcer {
    enforcer: Enforcer,
}

impl CasbinEnforcer {
    pub async fn default() -> Self {
        /*加载模型文件*/
        let m = DefaultModel::from_file("./rbac_with_domains_model.conf")
            .await
            .unwrap()
            /*初始化适配器*/;

        let a = MemoryAdapter::default();

        let mut enforcer = Enforcer::new(m, a).await.unwrap();
        let p = vec![
            "jsw".to_string(),
            "domain1".to_string(),
            "data2".to_string(),
            "read".to_string(),
        ];

        enforcer.add_policy(p).await.unwrap();
        Self { enforcer }
    }

    pub async fn addpolice(&mut self, p: Vec<String>) -> casbin::Result<bool> {
        self.enforcer.add_policy(p).await
    }
}

pub async fn init_casbin() {
    let enforcer = CasbinEnforcer::default().await;

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

    let r = GLOBALE_CASBIN_ENFORCER
        .get()
        .unwrap()
        .write()
        .await
        .addpolice(args.clone())
        .await;

    println!("{:?}", r);

    Ok(())
}
