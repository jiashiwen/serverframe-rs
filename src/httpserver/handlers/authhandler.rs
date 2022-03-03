use crate::privilege::{get_user_by_id, get_user_id_from_token, ObjType, Policy};
use anyhow::{anyhow, Error, Result};
use axum::headers::HeaderMap;

pub async fn auth(mut p: Policy, hm: HeaderMap) -> Result<bool> {
    let token = hm.get("authorization");
    if let Some(t) = token {
        let ts = t.to_str().map_err(|e| Error::msg(anyhow!(e.to_string())))?;
        let uid = get_user_id_from_token(ts.to_string())
            .map_err(|e| Error::msg(anyhow!(e.to_string())))?;
        let user = get_user_by_id(uid)?;
        p.set_sub(user.name.clone());

        let ok = p
            .enforce()
            .await
            .map_err(|_e| Error::msg(anyhow!("casbin enforce error!")))?;
        return Ok(ok);
    }

    Err(anyhow!("header authorization not found"))
}
