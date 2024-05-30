use std::collections::BTreeMap;

use qubit::ExportType;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use ts_rs::TS;

mod client;
mod message;
mod subscriptions;

pub use client::Client;
pub use message::Message;
use subscriptions::Subscriptions;

// TODO: Remove once features are added
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Uuid(uuid::Uuid);

impl TS for Uuid {
    type WithoutGenerics = <String as TS>::WithoutGenerics;

    fn decl() -> String {
        <String as TS>::decl()
    }

    fn decl_concrete() -> String {
        <String as TS>::decl_concrete()
    }

    fn name() -> String {
        <String as TS>::name()
    }

    fn inline() -> String {
        <String as TS>::inline()
    }

    fn inline_flattened() -> String {
        <String as TS>::inline_flattened()
    }
}

impl ExportType for Uuid {}

impl std::ops::Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Uuid {
    pub fn new_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Clone, Debug)]
pub struct Poll {
    id: Uuid,
    name: String,
    description: String,
    private: bool,
    options: Vec<String>,
    votes: Vec<usize>,
}

impl Poll {
    pub fn new(
        id: Uuid,
        name: String,
        description: String,
        private: bool,
        options: Vec<String>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            private,
            votes: vec![0; options.len()],
            options,
        }
    }
}

#[derive(Clone, Debug, TS, Serialize, ExportType)]
pub struct PollOverview {
    id: Uuid,
    name: String,
    description: String,
    options: Vec<String>,
}

impl From<&Poll> for PollOverview {
    fn from(poll: &Poll) -> Self {
        Self {
            id: poll.id,
            name: poll.name.clone(),
            description: poll.description.clone(),
            options: poll.options.clone(),
        }
    }
}

#[derive(Default)]
pub struct Manager {
    polls: BTreeMap<Uuid, Poll>,
    subscriptions: Subscriptions,
}

impl Manager {
    /// Start the manager, returning a client to communicate with it.
    pub fn start() -> Client {
        let (tx, mut rx) = mpsc::channel(10);

        tokio::spawn(async move {
            let mut manager = Manager::default();

            while let Some(message) = rx.recv().await {
                manager.process(message).await;
            }
        });

        Client::new(tx)
    }

    /// Process an incomming message.
    async fn process(&mut self, message: Message) {
        match message {
            Message::GetSummary { poll, tx } => {
                tx.send(self.get_summary(poll)).unwrap();
            }
            Message::GetSummaries { tx } => {
                tx.send(self.get_summaries()).unwrap();
            }
            Message::Create {
                name,
                description,
                private,
                options,
                tx,
            } => {
                tx.send(self.create_poll(name, description, private, options).await)
                    .unwrap();
            }
            Message::Vote { poll, option, tx } => {
                tx.send(self.vote(poll, option).await).unwrap();
            }
            Message::RegisterPoll { poll, tx } => {
                self.register_poll_subscription(poll, tx).await;
            }
            Message::RegisterPollTotal { tx } => {
                self.register_poll_total_subscription(tx).await;
            }
            Message::RegisterOverview { tx } => {
                self.register_overview_subscription(tx).await;
            }
        }
    }

    /// Get the summary for a specific poll.
    fn get_summary(&self, id: Uuid) -> Option<PollOverview> {
        self.polls.get(&id).map(|poll| poll.into())
    }

    /// Get a list of summaries for all polls.
    fn get_summaries(&self) -> Vec<PollOverview> {
        self.polls
            .values()
            .filter(|poll| !poll.private)
            .map(|poll| poll.into())
            .collect()
    }

    /// Create a new poll.
    async fn create_poll(
        &mut self,
        name: String,
        description: String,
        private: bool,
        options: Vec<String>,
    ) -> Uuid {
        // Create and insert the new poll
        let id = Uuid::new_v4();
        self.polls
            .insert(id, Poll::new(id, name, description, private, options));

        // Notify subscribers
        self.subscriptions.update_overview(&self.polls).await;
        self.subscriptions.update_poll_total(&self.polls).await;

        id
    }

    /// Vote for an option on a poll. Returns `true` if the vote is successfully placed, or `false`
    /// if the poll or option cannot be found.
    async fn vote(&mut self, poll: Uuid, option: usize) -> bool {
        // Fetch the poll and option
        let Some(poll) = self.polls.get_mut(&poll) else {
            return false;
        };
        let Some(option) = poll.votes.get_mut(option) else {
            return false;
        };

        // Increment the vote
        *option += 1;

        // Trigger subscription updates
        self.subscriptions.update_poll(poll).await;
        self.subscriptions.update_poll_total(&self.polls).await;

        true
    }

    /// Register vote subscription for a specific poll.
    async fn register_poll_subscription(&mut self, poll: Uuid, tx: mpsc::Sender<Vec<usize>>) {
        if let Some(poll) = self.polls.get(&poll) {
            self.subscriptions.register_poll(tx, poll).await;
        }
    }

    /// Register poll total subscription for all polls.
    async fn register_poll_total_subscription(&mut self, tx: mpsc::Sender<BTreeMap<Uuid, usize>>) {
        self.subscriptions
            .register_poll_total(tx, &self.polls)
            .await;
    }

    /// Register overview subscription for all polls.
    async fn register_overview_subscription(
        &mut self,
        tx: mpsc::Sender<BTreeMap<Uuid, PollOverview>>,
    ) {
        self.subscriptions.register_overview(tx, &self.polls).await;
    }
}
