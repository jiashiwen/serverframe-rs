use anyhow::Result;
use casbin::prelude::*;

use dashmap::DashMap;
use serde::de::Unexpected::Str;
use std::collections::HashMap;
use std::string::String;

#[tokio::main]
async fn main() -> Result<()> {
    let map = DashMap::new();
    map.insert("a", "b");
    map.insert("c", "b");
    map.insert("c", "c");
    let a = *map.get("a").unwrap();

    println!("{:?}", map);
    println!("a is {}", a);
    let mut e = Enforcer::new(
        "examples/rbac_with_domains_model.conf",
        "examples/rbac_with_domains_policy.csv",
    )
    .await?;
    e.enable_log(true);

    let allowed = e.enforce(("alice", "domain1", "data1", "read"))?;
    println!("{}", allowed);
    Ok(())
}
