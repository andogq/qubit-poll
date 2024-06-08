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
use qubit::Router;

use crate::manager::Manager;

#[derive(Clone)]
pub struct Ctx {
    pub client: Client,
    pub user_id: u32,
}

fn setup_router() -> Router<Ctx> {
    polls::init()
}

#[tokio::main]
async fn main() {
    let app = setup_router();

    #[cfg(debug_assertions)]
    {
        println!("Generating types");
        app.write_bindings_to_dir("./app/src/lib/bindings");
    }

    let client = Manager::start();
    let next_user_id = Arc::new(AtomicU32::new(0));

    let (app_service, app_handle) = app.to_service(
        move |_req| {
            let ctx = Ctx {
                client: client.clone(),
                user_id: next_user_id.fetch_add(1, Ordering::Relaxed),
            };

            async { ctx }
        },
        |_| async {},
    );

    let router = axum::Router::<()>::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest_service("/api", app_service);

    println!("Starting server");

    axum::serve(
        tokio::net::TcpListener::bind(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3030)))
            .await
            .unwrap(),
        router.into_make_service(),
    )
    .await
    .unwrap();

    println!("Shutting down server");

    app_handle.stop().unwrap();
}
