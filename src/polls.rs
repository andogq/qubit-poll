use qubit::{handler, Router};

use crate::{poll_manager::Poll, Ctx};

#[handler]
async fn list(ctx: Ctx) -> Vec<Poll> {
    ctx.poll_manager.list_polls().await
}

#[handler]
async fn create(ctx: Ctx, name: String, description: String, options: Vec<String>) {
    ctx.poll_manager
        .create_poll(name, description, options)
        .await;
}

pub fn init() -> Router<Ctx> {
    Router::new().handler(list).handler(create)
}
