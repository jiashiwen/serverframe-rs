use async_once::AsyncOnce;
use casbin::prelude::*;
use casbin::MemoryAdapter;
use casbin::Result;
use futures_locks::RwLock;

lazy_static::lazy_static! {
    static ref GLOBALE_CASBIN_ENFORCER: AsyncOnce<RwLock<CasbinEnforcer>> = AsyncOnce::new(async {
        let ce=CasbinEnforcer::default().await;
        RwLock::new(ce)
    });
}
pub struct CasbinEnforcer {
    enforcer: Enforcer,
}

impl CasbinEnforcer {
    pub async fn default() -> Self {
        let m = DefaultModel::from_file("./rbac_with_domains_model.conf")
            .await
            .unwrap();

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
        Self { enforcer }
    }

    pub async fn addpolice(&mut self, p: Vec<String>) -> Result<bool> {
        self.enforcer.add_policy(p).await
    }
}

pub async fn enfoce(args: Vec<String>) -> Result<bool> {
    GLOBALE_CASBIN_ENFORCER
        .get()
        .await
        .read()
        .await
        .enforcer
        .enforce(args)
}

pub async fn add_policy(p: Vec<String>) -> Result<bool> {
    GLOBALE_CASBIN_ENFORCER
        .get()
        .await
        .write()
        .await
        .addpolice(p)
        .await
}
