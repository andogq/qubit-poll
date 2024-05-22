use qubit::{handler, Router};

use crate::Ctx;

#[handler]
async fn list(ctx: Ctx) -> Vec<String> {
    ctx.poll_manager.list_polls().await
}

#[handler]
async fn create(ctx: Ctx, name: String, options: Vec<String>) {
    ctx.poll_manager.create_poll(name, options).await;
}

pub fn init() -> Router<Ctx> {
    Router::new().handler(list).handler(create)
}
