use std::borrow::Borrow;
use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;
use std::time::Duration;
use tokio::{select, spawn};

use tokio::task::JoinHandle;
// use tracing::Instrument;

// use tracing::instrument::WithSubscriber;
use crate::httpserver::routers::router_root;
use tokio::sync::oneshot::{channel, Receiver, Sender};

pub struct HttpServer {
    pub addr: SocketAddr,
    pub router: Router,
}

impl HttpServer {
    pub fn default() -> Self {
        let band_addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        // let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        // let app = Router::new()
        //     .route("/", get(root));
        let app = router_root();
        Self {
            addr: band_addr,
            router: app,
        }
    }

    pub async fn run(&mut self, rx: Receiver<()>) -> JoinHandle<()> {
        // pub async fn run(&mut self) -> JoinHandle<T> {
        // let mut handles = vec![];

        let server = axum::Server::bind(&self.addr)
            .serve(self.router.clone().into_make_service())
            .with_graceful_shutdown(async {
                rx.await.ok();
            });

        let handle = spawn(async {
            server.await.unwrap();
        });
        log::info!("httpserver start");
        return handle;
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}
