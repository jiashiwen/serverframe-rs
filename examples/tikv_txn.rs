use tikv_client::{Key, RawClient, Value};

fn main() {
    let pdaddr = vec!["114.67.120.120:2379"];
    let rt = tokio::runtime::Runtime::new().unwrap();

    let future = async {
        // let tikv_handler = tikv_client: RawClient::new(pdaddr, None).await.unwrap();

        let key1: Key = b"key1".to_vec().into();
        let value1: Value = b"value1".to_vec();
        let key2: Key = b"key2".to_vec().into();
        let value2: Value = b"value2".to_vec();

        let client = tikv_client::TransactionClient::new(pdaddr, None)
            .await
            .unwrap();

        let mut txn = client
            .begin_optimistic()
            .await
            .expect("Could not begin a transaction");
        txn.delete(key1.clone())
            .await
            .expect("Could not del key value");
        txn.delete(key2.clone())
            .await
            .expect("Could not del key value");
        // let res = txn.get(key1.clone()).await.expect("Could not get value");
        txn.put(key1.clone(), value1)
            .await
            .expect("Could not set key value");
        let res = txn.get(key1.clone()).await.expect("Could not get value");

        if let Some(res) = res.clone() {
            txn.put(key2.clone(), res)
                .await
                .expect("Could not set key value");
        }

        let res1 = txn.get(key2.clone()).await.expect("Could not get value");

        txn.commit()
            .await
            .expect("Committing read-only transaction should not fail");
        if let Some(res) = res {
            println!("result is: {:?}", String::from_utf8(res));
        }

        if let Some(res1) = res1 {
            println!("result is: {:?}", String::from_utf8(res1));
        }
    };
    rt.block_on(future);
}
