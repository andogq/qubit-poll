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
        let (tx, rx) = mpsc::channel(10);

        // Spin off the other thread
        tokio::spawn(manager(rx));

        Self { tx }
    }

    /// Retrieve a list of all available polls.
    pub async fn list_polls(&self) -> Vec<PollSummary> {
        send_message!(self, List {})
    }

    /// Create a new poll with the provided name.
    pub async fn create_poll(
        &self,
        name: String,
        description: String,
        options: Vec<String>,
    ) -> u32 {
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
    pub async fn get_poll(&self, poll: u32) -> Option<PollSummary> {
        send_message!(self, GetSummary { poll: poll })
    }

    /// Vote for the given option on a poll.
    pub async fn vote(&self, poll: u32, option: u32) -> bool {
        send_message!(
            self,
            Vote {
                poll: poll,
                option: option
            }
        )
    }

    /// Subscribe to vote changes on the given stream
    pub async fn poll_votes(&self, poll: u32) -> impl Stream<Item = Vec<usize>> {
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
    List {
        tx: oneshot::Sender<Vec<PollSummary>>,
    },

    /// Create a new poll with the provided name. If the creation was successful, `true` will be
    /// returned, or `false` if there was an error.
    Create {
        name: String,
        description: String,
        options: Vec<String>,
        tx: oneshot::Sender<u32>,
    },

    /// Get a poll with the given ID.
    GetSummary {
        poll: u32,
        tx: oneshot::Sender<Option<PollSummary>>,
    },

    /// Subscribe to vote changes on a given poll.
    PollVotes {
        poll: u32,
        tx: mpsc::Sender<Vec<usize>>,
    },

    /// Subscribe to an overview of all polls.
    Overview { tx: mpsc::Sender<Vec<PollSummary>> },

    /// Subscribe to a list of all poll totals.
    PollTotals { tx: mpsc::Sender<Vec<usize>> },

    /// Vote on a poll.
    Vote {
        poll: u32,
        option: u32,
        tx: oneshot::Sender<bool>,
    },
}

#[derive(Clone, Debug, TS, Serialize, TypeDependencies)]
pub struct Poll {
    id: u32,
    name: String,
    description: String,
    options: Vec<String>,
    votes: Vec<usize>,
}

#[derive(Clone, Debug, TS, Serialize, TypeDependencies)]
pub struct PollSummary {
    id: u32,
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

async fn manager(mut rx: mpsc::Receiver<Message>) {
    let mut polls = vec![
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

    let mut poll_subscriptions = Vec::new();
    let mut poll_total_subscriptions = Vec::new();
    let mut overview_subscriptions = Vec::new();

    while let Some(message) = rx.recv().await {
        match message {
            Message::List { tx } => {
                tx.send(polls.iter().map(|poll| poll.into()).collect())
                    .unwrap();
            }
            Message::Create {
                name,
                description,
                options,
                tx,
            } => {
                let id = polls.len() as u32;
                polls.push(Poll {
                    id,
                    name,
                    description,
                    votes: vec![0; options.len()],
                    options,
                });

                tx.send(id).unwrap();

                // Generate poll overview and totals
                let (overview, totals): (Vec<_>, Vec<_>) = polls
                    .iter()
                    .map(|poll| (poll.into(), poll.votes.iter().sum::<usize>()))
                    .unzip();

                // Update overview
                overview_subscriptions
                    .iter()
                    .map(|tx: &mpsc::Sender<_>| {
                        let overview = overview.clone();

                        async move { tx.send(overview).await.unwrap() }
                    })
                    .collect::<FuturesUnordered<_>>()
                    .collect::<()>()
                    .await;

                // Update poll totals
                poll_total_subscriptions
                    .iter()
                    .map(|tx: &mpsc::Sender<_>| {
                        let totals = totals.clone();
                        async move { tx.send(totals).await.unwrap() }
                    })
                    .collect::<FuturesUnordered<_>>()
                    .collect::<()>()
                    .await;
            }
            Message::GetSummary { poll, tx } => {
                tx.send(polls.get(poll as usize).map(|poll| poll.into()))
                    .unwrap();
            }
            Message::PollVotes { poll: poll_id, tx } => {
                // Send initial state
                let Some(poll) = polls.get(poll_id as usize) else {
                    return;
                };
                tx.send(poll.votes.clone()).await.unwrap();

                // Save the subscription
                poll_subscriptions.push((poll_id, tx));
            }
            Message::Vote { poll, option, tx } => {
                let Some(poll) = polls.get_mut(poll as usize) else {
                    return tx.send(false).unwrap();
                };

                let Some(option) = poll.votes.get_mut(option as usize) else {
                    return tx.send(false).unwrap();
                };

                // Increment the vote
                *option += 1;

                // Notify subscribers
                poll_subscriptions
                    .iter()
                    .filter(|(id, _)| *id == poll.id)
                    .map(|(_, tx)| {
                        let votes = poll.votes.clone();

                        async move { tx.send(votes).await.unwrap() }
                    })
                    .collect::<FuturesUnordered<_>>()
                    .collect::<()>()
                    .await;

                let totals = polls
                    .iter()
                    .map(|poll| poll.votes.iter().sum())
                    .collect::<Vec<_>>();
                poll_total_subscriptions
                    .iter()
                    .map(|tx: &mpsc::Sender<_>| {
                        let totals = totals.clone();
                        async move { tx.send(totals).await.unwrap() }
                    })
                    .collect::<FuturesUnordered<_>>()
                    .collect::<()>()
                    .await;

                tx.send(true).unwrap();
            }
            Message::Overview { tx } => {
                // Send inital state
                tx.send(polls.iter().map(|poll| poll.into()).collect())
                    .await
                    .unwrap();

                // Store subscription
                overview_subscriptions.push(tx);
            }

            Message::PollTotals { tx } => {
                // Send initial state
                tx.send(polls.iter().map(|poll| poll.votes.iter().sum()).collect())
                    .await
                    .unwrap();

                // Save subscription
                poll_total_subscriptions.push(tx);
            }
        }
    }
}
