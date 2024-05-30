use std::collections::BTreeMap;

use futures::{stream, Stream};
use tokio::sync::{mpsc, oneshot};

use crate::manager::{Message, PollOverview};

use super::Uuid;

/// Helper macro to insert a oneshot channel into a [`Message`], and return the awaited response
/// from the channel.
macro_rules! send_message {
    ($self:ident, $ty:ident { $($key:ident: $val:tt),* }) => {
        {
            let (tx, rx) = oneshot::channel();
            $self.tx
                .send(Message::$ty {
                    tx,
                    $($key: $val),*
                })
                .await
                .unwrap();

            rx.await.unwrap()
        }
    };
}

/// Service to manage the state of the polls. For simplicity, this is just an in-memory store
/// running on another thread, with communication to the main thread via channels.
#[derive(Clone)]
pub struct Client {
    tx: mpsc::Sender<Message>,
}

impl Client {
    pub fn new(tx: mpsc::Sender<Message>) -> Self {
        Self { tx }
    }

    /// Get a poll summary with the provided ID.
    pub async fn get_summary(&self, poll: Uuid) -> Option<PollOverview> {
        send_message!(self, GetSummary { poll: poll })
    }

    /// Retrieve a list of all available polls.
    pub async fn get_summaries(&self) -> Vec<PollOverview> {
        send_message!(self, GetSummaries {})
    }

    /// Create a new poll with the provided name.
    pub async fn create(
        &self,
        name: String,
        description: String,
        private: bool,
        options: Vec<String>,
    ) -> Uuid {
        send_message!(
            self,
            Create {
                name: name,
                description: description,
                private: private,
                options: options
            }
        )
    }

    /// Vote for the given option on a poll.
    pub async fn vote(&self, poll: Uuid, option: usize) -> bool {
        send_message!(
            self,
            Vote {
                poll: poll,
                option: option
            }
        )
    }

    /// Stream poll votes for the given poll.
    pub async fn stream_poll(&self, poll: Uuid) -> impl Stream<Item = Vec<usize>> {
        // Setup the channel
        let (tx, rx) = mpsc::channel(10);

        // Send it to the manager
        self.tx
            .send(Message::RegisterPoll { poll, tx })
            .await
            .unwrap();

        // Turn the resulting channel into a stream
        stream::unfold(rx, |mut rx| async move { Some((rx.recv().await?, rx)) })
    }

    /// Stream poll totals for all polls.
    pub async fn stream_poll_total(&self) -> impl Stream<Item = BTreeMap<Uuid, usize>> {
        let (tx, rx) = mpsc::channel(10);

        self.tx
            .send(Message::RegisterPollTotal { tx })
            .await
            .unwrap();

        // Stream back the overview results
        stream::unfold(rx, |mut rx| async move { Some((rx.recv().await?, rx)) })
    }

    /// Stream poll overviews for all polls.
    pub async fn stream_overview(&self) -> impl Stream<Item = BTreeMap<Uuid, PollOverview>> {
        let (tx, rx) = mpsc::channel(10);

        self.tx
            .send(Message::RegisterOverview { tx })
            .await
            .unwrap();

        // Stream back the overview results
        stream::unfold(rx, |mut rx| async move { Some((rx.recv().await?, rx)) })
    }
}
