use tikv_client::{IntoOwnedRange, Key, KvPair, RawClient, TransactionClient, Value};

pub struct TiKVHandler {
    client_raw: RawClient,
    client_txn: TransactionClient,
}

impl TiKVHandler {
    pub async fn default() -> Self {
        let endpoint = vec!["172.0.0.1:2379"];
        Self {
            client_raw: RawClient::new(endpoint.clone(), None).await.unwrap(),
            client_txn: TransactionClient::new(endpoint, None).await.unwrap(),
        }
    }
    pub async fn set_self(&mut self, tikv_handler: TiKVHandler) {
        self.client_raw = tikv_handler.client_raw;
        self.client_txn = tikv_handler.client_txn;
    }
    pub async fn new(pd_endpoints: Vec<&str>) -> Self {
        Self {
            client_raw: RawClient::new(pd_endpoints.clone(), None).await.unwrap(),
            client_txn: TransactionClient::new(pd_endpoints, None).await.unwrap(),
        }
    }
    // pub fn get_tikvhandler(&self) -> &TiKVHandler {
    //     self
    // }
    pub async fn raw_put(&self, key: String, val: String) -> tikv_client::Result<()> {
        println!("invoke put");
        self.client_raw.put(Key::from(key), Value::from(val)).await
    }

    pub async fn raw_remove(&self, key: String) -> tikv_client::Result<()> {
        self.client_raw.delete(Key::from(key)).await
    }

    pub async fn raw_remove_all(&self) -> tikv_client::Result<()> {
        let range = "".."";
        self.client_raw.delete_range(range.into_owned()).await
    }

    pub async fn raw_get(&self, key: String) -> tikv_client::Result<Option<Value>> {
        self.client_raw.get(key.to_owned()).await
    }

    pub async fn raw_get_ttl_sec(&self, key: String) -> tikv_client::Result<Option<u64>> {
        self.client_raw.get_key_ttl_secs(key.to_owned()).await
    }

    pub async fn raw_put_with_ttl(
        &self,
        key: String,
        val: String,
        ttl: u64,
    ) -> tikv_client::Result<()> {
        self.client_raw
            .put_with_ttl(Key::from(key), Value::from(val.as_str()), ttl)
            .await
    }

    pub async fn prefix_scan(
        &self,
        start: String,
        end: String,
        limited: u32,
    ) -> tikv_client::Result<Vec<KvPair>> {
        let range = start..end;
        self.client_raw.scan(range, limited).await
        // self.client_raw.scan(range, limited).await.map_err(|e| {
        //     return KvPairError::OptionError(e.to_string());
        //     // e
        // })?
    }
}

// pub async fn set_global_tikv() {}
