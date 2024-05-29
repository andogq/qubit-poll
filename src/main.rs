mod manager;
mod polls;

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use axum::routing::get;
use manager::Client;
use qubit::{handler, Router};

use crate::manager::Manager;

#[derive(Clone)]
pub struct Ctx {
    pub client: Client,
    pub user_id: u32,
}

#[handler]
async fn hello_world(_ctx: Ctx) -> String {
    "Hello world!".to_string()
}

fn setup_router() -> Router<Ctx> {
    polls::init().handler(hello_world)
}

#[tokio::main]
async fn main() {
    let app = setup_router();

    println!("Generating types");
    app.write_type_to_file("./app/src/lib/server.ts");

    let client = Manager::start();
    let next_user_id = Arc::new(AtomicU32::new(0));

    let (app_service, app_handle) = app.to_service(move |_req| Ctx {
        client: client.clone(),
        user_id: next_user_id.fetch_add(1, Ordering::Relaxed),
    });

    let router = axum::Router::<()>::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest_service("/api", app_service);

    println!("Starting server");

    hyper::Server::bind(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3030)))
        .serve(router.into_make_service())
        .await
        .unwrap();

    println!("Shutting down server");

    app_handle.stop().unwrap();
}
