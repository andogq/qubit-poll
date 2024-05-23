use futures::Stream;
use qubit::{handler, Router};

use crate::{poll_manager::PollSummary, Ctx};

#[handler]
async fn list(ctx: Ctx) -> Vec<PollSummary> {
    ctx.poll_manager.list_polls().await
}

#[handler]
async fn create(ctx: Ctx, name: String, description: String, options: Vec<String>) {
    ctx.poll_manager
        .create_poll(name, description, options)
        .await;
}

#[handler]
async fn get(ctx: Ctx, id: u32) -> Option<PollSummary> {
    ctx.poll_manager.get_poll(id).await
}

#[handler]
async fn vote(ctx: Ctx, poll: u32, option: u32) {
    ctx.poll_manager.vote(poll, option).await;
}

#[handler(subscription)]
async fn poll_votes(ctx: Ctx, poll: u32) -> impl Stream<Item = Vec<usize>> {
    ctx.poll_manager.poll_votes(poll).await
}

pub fn init() -> Router<Ctx> {
    Router::new()
        .handler(list)
        .handler(create)
        .handler(get)
        .handler(vote)
        .handler(poll_votes)
}
