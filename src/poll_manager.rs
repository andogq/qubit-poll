use futures::{
    stream::{self, FuturesUnordered},
    Stream, StreamExt,
};
use qubit::TypeDependencies;
use serde::Serialize;
use tokio::sync::{mpsc, oneshot};
use ts_rs::TS;

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
pub struct PollManager {
    tx: mpsc::Sender<Message>,
}

impl PollManager {
    pub fn new() -> Self {
        // Start the manager
        let tx = Manager::start();

        Self { tx }
    }

    /// Retrieve a list of all available polls.
    pub async fn list_polls(&self) -> Vec<PollSummary> {
        send_message!(self, GetSummaries {})
    }

    /// Create a new poll with the provided name.
    pub async fn create_poll(
        &self,
        name: String,
        description: String,
        options: Vec<String>,
    ) -> usize {
        send_message!(
            self,
            Create {
                name: name,
                description: description,
                options: options
            }
        )
    }

    /// Get a poll with the provided ID.
    pub async fn get_poll(&self, poll: usize) -> Option<PollSummary> {
        send_message!(self, GetSummary { poll: poll })
    }

    /// Vote for the given option on a poll.
    pub async fn vote(&self, poll: usize, option: usize) -> bool {
        send_message!(
            self,
            Vote {
                poll: poll,
                option: option
            }
        )
    }

    /// Subscribe to vote changes on the given stream
    pub async fn poll_votes(&self, poll: usize) -> impl Stream<Item = Vec<usize>> {
        // Setup the channel
        let (tx, rx) = mpsc::channel(10);

        // Send it to the manager
        self.tx.send(Message::PollVotes { poll, tx }).await.unwrap();

        // Turn the resulting channel into a stream
        stream::unfold(rx, |mut rx| async move { Some((rx.recv().await?, rx)) })
    }

    /// Return a stream of vote results, which will emit new values whenever a poll changes.
    pub async fn overview(&self) -> impl Stream<Item = Vec<PollSummary>> {
        let (tx, rx) = mpsc::channel(10);

        self.tx.send(Message::Overview { tx }).await.unwrap();

        // Stream back the overview results
        stream::unfold(rx, |mut rx| async move { Some((rx.recv().await?, rx)) })
    }

    /// Return a stream of poll totals.
    pub async fn poll_totals(&self) -> impl Stream<Item = Vec<usize>> {
        let (tx, rx) = mpsc::channel(10);

        self.tx.send(Message::PollTotals { tx }).await.unwrap();

        // Stream back the overview results
        stream::unfold(rx, |mut rx| async move { Some((rx.recv().await?, rx)) })
    }
}

/// All possible message variations.
enum Message {
    /// List all available polls.
    GetSummaries {
        tx: oneshot::Sender<Vec<PollSummary>>,
    },

    /// Create a new poll with the provided name. If the creation was successful, `true` will be
    /// returned, or `false` if there was an error.
    Create {
        name: String,
        description: String,
        options: Vec<String>,
        tx: oneshot::Sender<usize>,
    },

    /// Get a poll with the given ID.
    GetSummary {
        poll: usize,
        tx: oneshot::Sender<Option<PollSummary>>,
    },

    /// Subscribe to vote changes on a given poll.
    PollVotes {
        poll: usize,
        tx: mpsc::Sender<Vec<usize>>,
    },

    /// Subscribe to an overview of all polls.
    Overview { tx: mpsc::Sender<Vec<PollSummary>> },

    /// Subscribe to a list of all poll totals.
    PollTotals { tx: mpsc::Sender<Vec<usize>> },

    /// Vote on a poll.
    Vote {
        poll: usize,
        option: usize,
        tx: oneshot::Sender<bool>,
    },
}

#[derive(Clone, Debug, TS, Serialize, TypeDependencies)]
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
pub struct PollSummary {
    id: usize,
    name: String,
    description: String,
    options: Vec<String>,
}

impl From<&Poll> for PollSummary {
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
struct Subscriptions {
    poll: Vec<(usize, mpsc::Sender<Vec<usize>>)>,
    poll_total: Vec<mpsc::Sender<Vec<usize>>>,
    overview: Vec<mpsc::Sender<Vec<PollSummary>>>,
}

impl Subscriptions {
    /// Register a new `poll` subscription.
    async fn register_poll(&mut self, tx: mpsc::Sender<Vec<usize>>, poll: &Poll) {
        // Send the initial state
        tx.send(poll.votes.clone()).await.unwrap();

        self.poll.push((poll.id, tx));
    }

    /// Register a new `poll_total` subscription.
    async fn register_poll_total(&mut self, tx: mpsc::Sender<Vec<usize>>, polls: &[Poll]) {
        // Send the initial state
        tx.send(Self::poll_totals(polls)).await.unwrap();

        self.poll_total.push(tx);
    }

    /// Register a new `overview` subscription.
    async fn register_overview(&mut self, tx: mpsc::Sender<Vec<PollSummary>>, polls: &[Poll]) {
        // Send the initial state
        tx.send(polls.iter().map(|poll| poll.into()).collect())
            .await
            .unwrap();

        self.overview.push(tx);
    }

    /// Update all `poll` subscriptions.
    async fn update_poll(&self, poll: &Poll) {
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
    async fn update_poll_total(&self, polls: &[Poll]) {
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
    async fn update_overview(&self, polls: &[Poll]) {
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

#[derive(Default)]
struct Manager {
    polls: Vec<Poll>,
    subscriptions: Subscriptions,
}

impl Manager {
    pub fn start() -> mpsc::Sender<Message> {
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

        tx
    }

    /// Process an incomming message
    async fn process(&mut self, message: Message) {
        match message {
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
            Message::GetSummary { poll, tx } => {
                tx.send(self.get_summary(poll)).unwrap();
            }
            Message::Vote { poll, option, tx } => {
                tx.send(self.vote(poll, option).await).unwrap();
            }
            Message::PollVotes { poll, tx } => {
                self.register_poll_subscription(poll, tx).await;
            }
            Message::Overview { tx } => {
                self.register_overview_subscription(tx).await;
            }
            Message::PollTotals { tx } => {
                self.register_poll_total_subscription(tx).await;
            }
        }
    }

    /// Get the summary for a specific poll.
    fn get_summary(&self, id: usize) -> Option<PollSummary> {
        self.polls.get(id).map(|poll| poll.into())
    }

    /// Get a list of summaries for all polls.
    fn get_summaries(&self) -> Vec<PollSummary> {
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
    async fn register_overview_subscription(&mut self, tx: mpsc::Sender<Vec<PollSummary>>) {
        self.subscriptions.register_overview(tx, &self.polls).await;
    }
}
