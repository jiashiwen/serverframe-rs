use anyhow::Result;
use base64;
use casbin::prelude::*;

use serde::de::Unexpected::Str;
use std::collections::HashMap;
use std::string::String;

#[tokio::main]
async fn main() -> Result<()> {
    let mut base = base64::encode("0");
    println!("{}", base);

    let decodestr = base64::decode(base).unwrap();

    let str = String::from_utf8(decodestr);
    println!("{:?}", str);
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
