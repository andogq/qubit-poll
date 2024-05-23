use futures::{stream::FuturesUnordered, StreamExt};
use tokio::sync::mpsc;

use super::{Poll, PollOverview};

#[derive(Default)]
pub(super) struct Subscriptions {
    poll: Vec<(usize, mpsc::Sender<Vec<usize>>)>,
    poll_total: Vec<mpsc::Sender<Vec<usize>>>,
    overview: Vec<mpsc::Sender<Vec<PollOverview>>>,
}

impl Subscriptions {
    /// Register a new `poll` subscription.
    pub async fn register_poll(&mut self, tx: mpsc::Sender<Vec<usize>>, poll: &Poll) {
        // Send the initial state
        tx.send(poll.votes.clone()).await.unwrap();

        self.poll.push((poll.id, tx));
    }

    /// Register a new `poll_total` subscription.
    pub async fn register_poll_total(&mut self, tx: mpsc::Sender<Vec<usize>>, polls: &[Poll]) {
        // Send the initial state
        tx.send(Self::poll_totals(polls)).await.unwrap();

        self.poll_total.push(tx);
    }

    /// Register a new `overview` subscription.
    pub async fn register_overview(&mut self, tx: mpsc::Sender<Vec<PollOverview>>, polls: &[Poll]) {
        // Send the initial state
        tx.send(polls.iter().map(|poll| poll.into()).collect())
            .await
            .unwrap();

        self.overview.push(tx);
    }

    /// Update all `poll` subscriptions.
    pub async fn update_poll(&self, poll: &Poll) {
        self.poll
            .iter()
            .filter(|(id, _)| *id == poll.id)
            .map(|(_, tx)| {
                let votes = poll.votes.clone();

                async move { tx.send(votes).await.unwrap() }
            })
            .collect::<FuturesUnordered<_>>()
            .collect::<()>()
            .await;
    }

    /// Update all `poll_total` subscriptions.
    pub async fn update_poll_total(&self, polls: &[Poll]) {
        let totals = Self::poll_totals(polls);

        self.poll_total
            .iter()
            .map(|tx: &mpsc::Sender<_>| {
                let totals = totals.clone();
                async move { tx.send(totals).await.unwrap() }
            })
            .collect::<FuturesUnordered<_>>()
            .collect::<()>()
            .await;
    }

    /// Update all `overview` subscriptions.
    pub async fn update_overview(&self, polls: &[Poll]) {
        let overview = polls.iter().map(|p| p.into()).collect::<Vec<_>>();

        self.overview
            .iter()
            .map(|tx: &mpsc::Sender<_>| {
                let overview = overview.clone();

                async move { tx.send(overview).await.unwrap() }
            })
            .collect::<FuturesUnordered<_>>()
            .collect::<()>()
            .await;
    }

    fn poll_totals(polls: &[Poll]) -> Vec<usize> {
        polls
            .iter()
            .map(|poll| poll.votes.iter().sum())
            .collect::<Vec<_>>()
    }
}
