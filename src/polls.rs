use qubit::{handler, Router};

use crate::{manager::PollOverview, Ctx};

#[handler]
async fn get_summaries(ctx: Ctx) -> Vec<PollOverview> {
    ctx.client.get_summaries().await
}

#[handler]
async fn get_summary(ctx: Ctx, id: usize) -> Option<PollOverview> {
    ctx.client.get_summary(id).await
}

#[handler]
async fn create(ctx: Ctx, name: String, description: String, options: Vec<String>) {
    ctx.client.create(name, description, options).await;
}

#[handler]
async fn vote(ctx: Ctx, poll: usize, option: usize) {
    ctx.client.vote(poll, option).await;
}

mod stream {
    use futures::Stream;
    use qubit::{handler, Router};

    use crate::{manager::PollOverview, Ctx};

    #[handler(subscription)]
    async fn poll(ctx: Ctx, poll_id: usize) -> impl Stream<Item = Vec<usize>> {
        ctx.client.stream_poll(poll_id).await
    }

    #[handler(subscription)]
    async fn poll_total(ctx: Ctx) -> impl Stream<Item = Vec<usize>> {
        ctx.client.stream_poll_total().await
    }

    #[handler(subscription)]
    async fn overview(ctx: Ctx) -> impl Stream<Item = Vec<PollOverview>> {
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
