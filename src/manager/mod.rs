use qubit::TypeDependencies;
use serde::Serialize;
use tokio::sync::mpsc;
use ts_rs::TS;

mod client;
mod message;
mod subscriptions;

pub use client::Client;
pub use message::Message;
use subscriptions::Subscriptions;

#[derive(Clone, Debug)]
pub struct Poll {
    id: usize,
    name: String,
    description: String,
    options: Vec<String>,
    votes: Vec<usize>,
}

impl Poll {
    pub fn new(id: usize, name: String, description: String, options: Vec<String>) -> Self {
        Self {
            id,
            name,
            description,
            votes: vec![0; options.len()],
            options,
        }
    }
}

#[derive(Clone, Debug, TS, Serialize, TypeDependencies)]
pub struct PollOverview {
    id: usize,
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
    polls: Vec<Poll>,
    subscriptions: Subscriptions,
}

impl Manager {
    /// Start the manager, returning a client to communicate with it.
    pub fn start() -> Client {
        let (tx, mut rx) = mpsc::channel(10);

        tokio::spawn(async move {
            let mut manager = Manager::default();

            // TODO: Get rid of this
            manager.polls = vec![
                Poll {
                    id: 0,
                    name: "Favourite color".to_string(),
                    description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
                    options: vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string(), "Option D".to_string()],
                    votes: vec![3, 7, 2, 5],
                },
                Poll {
                    id: 1,
                    name: "Best food".to_string(),
                    description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
                    options: vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string(), "Option D".to_string()],
                    votes: vec![3, 7, 2, 5],
                },
                Poll {
                    id: 2,
                    name: "Favourite color".to_string(),
                    description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
                    options: vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string(), "Option D".to_string()],
                    votes: vec![3, 7, 2, 5],
                },
                Poll {
                    id: 3,
                    name: "Best food".to_string(),
                    description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
                    options: vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string(), "Option D".to_string()],
                    votes: vec![3, 7, 2, 5],
                },
            ];

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
                options,
                tx,
            } => {
                tx.send(self.create_poll(name, description, options).await)
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
    fn get_summary(&self, id: usize) -> Option<PollOverview> {
        self.polls.get(id).map(|poll| poll.into())
    }

    /// Get a list of summaries for all polls.
    fn get_summaries(&self) -> Vec<PollOverview> {
        self.polls.iter().map(|poll| poll.into()).collect()
    }

    /// Create a new poll.
    async fn create_poll(
        &mut self,
        name: String,
        description: String,
        options: Vec<String>,
    ) -> usize {
        // Create and insert the new poll
        let id = self.polls.len();
        self.polls.push(Poll::new(id, name, description, options));

        // Notify subscribers
        self.subscriptions.update_overview(&self.polls).await;
        self.subscriptions.update_poll_total(&self.polls).await;

        id
    }

    /// Vote for an option on a poll. Returns `true` if the vote is successfully placed, or `false`
    /// if the poll or option cannot be found.
    async fn vote(&mut self, poll: usize, option: usize) -> bool {
        // Fetch the poll and option
        let Some(poll) = self.polls.get_mut(poll as usize) else {
            return false;
        };
        let Some(option) = poll.votes.get_mut(option as usize) else {
            return false;
        };

        // Increment the vote
        *option += 1;

        // Trigger subscription updates
        self.subscriptions.update_poll(&poll).await;
        self.subscriptions.update_poll_total(&self.polls).await;

        true
    }

    /// Register vote subscription for a specific poll.
    async fn register_poll_subscription(&mut self, poll: usize, tx: mpsc::Sender<Vec<usize>>) {
        if let Some(poll) = self.polls.get(poll) {
            self.subscriptions.register_poll(tx, poll).await;
        }
    }

    /// Register poll total subscription for all polls.
    async fn register_poll_total_subscription(&mut self, tx: mpsc::Sender<Vec<usize>>) {
        self.subscriptions
            .register_poll_total(tx, &self.polls)
            .await;
    }

    /// Register overview subscription for all polls.
    async fn register_overview_subscription(&mut self, tx: mpsc::Sender<Vec<PollOverview>>) {
        self.subscriptions.register_overview(tx, &self.polls).await;
    }
}
