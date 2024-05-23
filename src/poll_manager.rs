use std::collections::HashMap;

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
    pub async fn list_polls(&self) -> Vec<Poll> {
        send_message!(self, ListPolls {})
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
            CreatePoll {
                name: name,
                description: description,
                options: options
            }
        )
    }

    /// Get a poll with the provided ID.
    pub async fn get_poll(&self, id: u32) -> Option<Poll> {
        send_message!(self, GetPoll { id: id })
    }

    /// Vote for the given option on a poll.
    pub async fn vote(&self, poll: u32, option: String) -> bool {
        send_message!(
            self,
            Vote {
                poll: poll,
                option: option
            }
        )
    }

    /// Subscribe to vote changes on the given stream
    pub async fn subscribe(&self, poll: u32) -> impl Stream<Item = HashMap<String, usize>> {
        // Setup the channel
        let (tx, rx) = mpsc::channel(10);

        // Send it to the manager
        self.tx.send(Message::Subscribe { poll, tx }).await.unwrap();

        // Turn the resulting channel into a stream
        stream::unfold(rx, |mut rx| async move { Some((rx.recv().await?, rx)) })
    }
}

/// All possible message variations.
enum Message {
    /// List all available polls.
    ListPolls { tx: oneshot::Sender<Vec<Poll>> },

    /// Create a new poll with the provided name. If the creation was successful, `true` will be
    /// returned, or `false` if there was an error.
    CreatePoll {
        name: String,
        description: String,
        options: Vec<String>,
        tx: oneshot::Sender<u32>,
    },

    /// Get a poll with the given ID.
    GetPoll {
        id: u32,
        tx: oneshot::Sender<Option<Poll>>,
    },

    /// Subscribe to vote changes on a given poll
    Subscribe {
        poll: u32,
        tx: mpsc::Sender<HashMap<String, usize>>,
    },

    /// Vote on a poll
    Vote {
        poll: u32,
        option: String,
        tx: oneshot::Sender<bool>,
    },
}

#[derive(Clone, Debug, TS, Serialize, TypeDependencies)]
pub struct Poll {
    id: u32,
    name: String,
    description: String,
    options: HashMap<String, usize>,
}

async fn manager(mut rx: mpsc::Receiver<Message>) {
    let mut polls = vec![
        Poll {
            id: 0,
            name: "Favourite color".to_string(),
            description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
            options: ["Option A", "Option B", "Option C", "Option D"].into_iter().enumerate().map(|(i, s)| (s.to_string(), i)).collect(),
        },
        Poll {
            id: 1,
            name: "Best food".to_string(),
            description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
            options: ["Option A", "Option B", "Option C", "Option D"].into_iter().enumerate().map(|(i, s)| (s.to_string(), i)).collect(),
        },
        Poll {
            id: 2,
            name: "Favourite color".to_string(),
            description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
            options: ["Option A", "Option B", "Option C", "Option D"].into_iter().enumerate().map(|(i, s)| (s.to_string(), i)).collect(),
        },
        Poll {
            id: 3,
            name: "Best food".to_string(),
            description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
            options: ["Option A", "Option B", "Option C", "Option D"].into_iter().enumerate().map(|(i, s)| (s.to_string(), i)).collect(),
        },
    ];

    let mut subscriptions = Vec::new();

    while let Some(message) = rx.recv().await {
        match message {
            Message::ListPolls { tx } => {
                tx.send(polls.clone()).unwrap();
            }
            Message::CreatePoll {
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
                    options: options.into_iter().map(|s| (s, 0)).collect(),
                });

                tx.send(id).unwrap();
            }
            Message::GetPoll { id, tx } => {
                tx.send(polls.get(id as usize).cloned()).unwrap();
            }
            Message::Subscribe { poll, tx } => {
                subscriptions.push((poll, tx));
            }
            Message::Vote { poll, option, tx } => {
                let Some(poll) = polls.get_mut(poll as usize) else {
                    return tx.send(false).unwrap();
                };

                let Some(option) = poll.options.get_mut(&option) else {
                    return tx.send(false).unwrap();
                };

                // Increment the vote
                *option += 1;

                // Notify subscribers
                subscriptions
                    .iter()
                    .filter(|(id, _)| *id == poll.id)
                    .map(|(_, tx)| tx.send(poll.options.clone()))
                    .collect::<FuturesUnordered<_>>()
                    .for_each_concurrent(None, |r| async move { r.unwrap() })
                    .await;

                tx.send(true).unwrap();
            }
        }
    }
}
