use std::net::{Ipv4Addr, SocketAddr};

use axum::routing::get;
use qubit::{handler, Router};

#[handler]
async fn hello_world(_ctx: ()) -> String {
    "Hello world!".to_string()
}

fn setup_router() -> Router<()> {
    Router::new().handler(hello_world)
}

#[tokio::main]
async fn main() {
    let app = setup_router();

    println!("Generating types");
    app.write_type_to_file("./app/src/lib/api.ts");

    let (app_service, app_handle) = app.to_service(|_| ());

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
