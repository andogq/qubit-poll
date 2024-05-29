use qubit::{handler, FromContext, Router};

use crate::{
    manager::{Client, PollOverview},
    Ctx,
};

#[derive(Clone)]
struct LoggingCtx {
    client: Client,
}

impl FromContext<Ctx> for LoggingCtx {
    fn from_app_ctx(ctx: Ctx) -> Result<Self, qubit::RpcError> {
        println!("processing request for user {}", ctx.user_id);

        Ok(LoggingCtx { client: ctx.client })
    }
}

#[handler]
async fn get_summaries(ctx: LoggingCtx) -> Vec<PollOverview> {
    ctx.client.get_summaries().await
}

#[handler]
async fn get_summary(ctx: LoggingCtx, id: usize) -> Option<PollOverview> {
    ctx.client.get_summary(id).await
}

#[handler]
async fn create(ctx: LoggingCtx, name: String, description: String, options: Vec<String>) {
    ctx.client.create(name, description, options).await;
}

#[handler]
async fn vote(ctx: LoggingCtx, poll: usize, option: usize) {
    ctx.client.vote(poll, option).await;
}

mod stream {
    use futures::Stream;

    use super::*;

    #[handler(subscription)]
    async fn poll(ctx: LoggingCtx, poll_id: usize) -> impl Stream<Item = Vec<usize>> {
        ctx.client.stream_poll(poll_id).await
    }

    #[handler(subscription)]
    async fn poll_total(ctx: LoggingCtx) -> impl Stream<Item = Vec<usize>> {
        ctx.client.stream_poll_total().await
    }

    #[handler(subscription)]
    async fn overview(ctx: LoggingCtx) -> impl Stream<Item = Vec<PollOverview>> {
        ctx.client.stream_overview().await
    }

    pub fn init() -> Router<Ctx> {
        Router::new()
            .handler(poll)
            .handler(poll_total)
            .handler(overview)
    }
}

pub fn init() -> Router<Ctx> {
    Router::new()
        .handler(get_summaries)
        .handler(create)
        .handler(get_summary)
        .handler(vote)
        .nest("stream", stream::init())
}
