use axum::http::{header, Request, Response, StatusCode};
use axum::{routing::get, Router};
use http_body::Body;
use std::marker::PhantomData;
use tower_http::auth::{AuthorizeRequest, RequireAuthorizationLayer};

struct MyAuth<ResBody> {
    _ty: PhantomData<fn() -> ResBody>,
}

impl<ResBody> Clone for MyAuth<ResBody> {
    fn clone(&self) -> Self {
        Self { _ty: PhantomData }
    }
}

impl<B, ResBody> AuthorizeRequest<B> for MyAuth<ResBody>
where
    ResBody: Body + Default,
{
    type ResponseBody = ResBody;

    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        let i = request
            .headers()
            .get(header::AUTHORIZATION)
            .map(|header| {
                header
                    .to_str()
                    .ok()
                    .map(|s| s.parse::<i32>().ok())
                    .flatten()
            })
            .flatten();
        match i {
            None => {
                let body = ResBody::default();
                let mut res = Response::new(body);
                *res.status_mut() = StatusCode::UNAUTHORIZED;
                return Err(res);
            }
            Some(i) => {
                if (i % 2) == 0 {
                    return Ok(());
                } else {
                    let body = ResBody::default();
                    let mut res = Response::new(body);
                    *res.status_mut() = StatusCode::UNAUTHORIZED;
                    return Err(res);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app =
        Router::new()
            .route("/", get(|| async { "Ok" }))
            .layer(RequireAuthorizationLayer::custom(MyAuth {
                _ty: PhantomData,
            }));
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
