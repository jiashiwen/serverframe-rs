use anyhow::Result;
use anyhow::Error;
use hyper::{Body, Method, Request};
use serde::Deserialize;
use crate::httpquerry::globalhttpclient::{GLOBAL_HTTP_CLIENT};
use crate::httpserver::module::Token;

#[derive(Deserialize, Debug)]
pub struct QueryResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}


pub async fn query_login() -> Result<Option<Token>> {
    let req = Request::builder()
        .method(Method::POST).header("Content-Type", "application/json")
        .uri("http://127.0.0.1:3000/login")
        .body(Body::from(r#"{"username": "root",	"password": "123456"}"#))
        .expect("request builder");


    let resp = GLOBAL_HTTP_CLIENT.http.request(req).await?;
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    // let str = String::from_utf8(body_bytes.to_vec()).unwrap();
    // let str = serde_json::from_slice::<Value>(&*body_bytes);
    let data = serde_json::from_slice::<QueryResult<Token>>(&*body_bytes).map_err(|e| {
        return Error::new(e);
    })?;
    Ok(data.data)
}

pub async fn query_baidu() -> Result<String> {
    let req = Request::builder()
        .method(Method::GET)
        .uri("https://www.baidu.com")
        .body(Body::empty())
        .expect("request builder");

    let resp = GLOBAL_HTTP_CLIENT.https.request(req).await?;
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let str = String::from_utf8(body_bytes.to_vec())?;

    Ok(str)
}

// pub async fn query_baidu() -> Result<Token> {
//     let ak = "4107B314B15BCE99A1C781DFCF119F59";
//     let sk = "8877CD432EB5738EFF0FA01F630201C9";
//     let credential = Credential::new(ak, sk);
//     let signer = Signer::new(credential, "vm".to_string(), "cn-north-1".to_string());
//     // let mut reqh = httprequest::builder();
//     //
//     // let mut reqh = req.method("GET")
//     //     .uri("https://vm.jdcloud-api.com/v1/regions/cn-north-1/instances")
//     //     .body("".to_string()).unwrap();
//     // signer.sign_request(&mut reqh).unwrap();
//     // let req = Request::builder()
//     //     .method(Method::POST).header("Content-Type", "application/json")
//     //     .header(reqh.)
//     //     .uri("https://vm.jdcloud-api.com/v1/regions/cn-north-1/instances")
//     //     .expect("request builder");
//
//
//     let req = Request::builder()
//         .method(Method::POST).header("Content-Type", "application/json")
//         .uri("http://127.0.0.1:3000/login")
//         .body(Body::from(r#"{"username": "root",	"password": "123456"}"#))
//         .expect("request builder");
//
//
//     let resp = GLOBAL_HYPER_CLIENT.request(req).await?;
//     let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
//     // let str = String::from_utf8(body_bytes.to_vec()).unwrap();
//     // let str = serde_json::from_slice::<Value>(&*body_bytes);
//     let data = serde_json::from_slice::<QueryResult<Token>>(&*body_bytes).map_err(|e| {
//         return Error::new(e);
//     })?;
//     Ok(data.data.unwrap())
// // println!("token is {:?}", data.unwrap().data.unwrap().token);
// }