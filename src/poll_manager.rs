use std::collections::HashMap;

use tokio::sync::{mpsc, oneshot};

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
    pub async fn list_polls(&self) -> Vec<String> {
        send_message!(self, ListPolls {})
    }

    /// Create a new poll with the provided name.
    pub async fn create_poll(&self, name: String) -> bool {
        send_message!(self, CreatePoll { name: name })
    }
}

/// All possible message variations.
enum Message {
    /// List all available polls.
    ListPolls { tx: oneshot::Sender<Vec<String>> },

    /// Create a new poll with the provided name. If the creation was successful, `true` will be
    /// returned, or `false` if there was an error.
    CreatePoll {
        name: String,
        tx: oneshot::Sender<bool>,
    },
}

async fn manager(mut rx: mpsc::Receiver<Message>) {
    let mut polls = HashMap::new();

    while let Some(message) = rx.recv().await {
        match message {
            Message::ListPolls { tx } => {
                let available_polls = polls.keys().cloned().collect();
                tx.send(available_polls).unwrap();
            }
            Message::CreatePoll { name, tx } => {
                if polls.contains_key(&name) {
                    return tx.send(false).unwrap();
                }

                polls.insert(name, ());

                tx.send(true).unwrap();
            }
        }
    }
}
