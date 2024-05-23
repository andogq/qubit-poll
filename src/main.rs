mod poll_manager;
mod polls;

use std::net::{Ipv4Addr, SocketAddr};

use axum::routing::get;
use poll_manager::PollManager;
use qubit::{handler, Router};

#[derive(Clone)]
pub struct Ctx {
    pub poll_manager: PollManager,
}

#[handler]
async fn hello_world(_ctx: Ctx) -> String {
    "Hello world!".to_string()
}

fn setup_router() -> Router<Ctx> {
    Router::new()
        .handler(hello_world)
        .nest("polls", polls::init())
}

#[tokio::main]
async fn main() {
    let app = setup_router();

    println!("Generating types");
    app.write_type_to_file("./app/src/lib/api.ts");

    let ctx = Ctx {
        poll_manager: PollManager::new(),
    };

    let (app_service, app_handle) = app.to_service(move |_| ctx.clone());

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
