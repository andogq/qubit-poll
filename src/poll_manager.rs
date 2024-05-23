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
}

#[derive(Clone, Debug, TS, Serialize, TypeDependencies)]
pub struct Poll {
    name: String,
    description: String,
    options: Vec<String>,
}

async fn manager(mut rx: mpsc::Receiver<Message>) {
    let mut polls = vec![
        Poll {
            name: "Favourite color".to_string(),
            description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
            options: Vec::new(),
        },
        Poll {
            name: "Best food".to_string(),
            description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
            options: Vec::new(),
        },
        Poll {
            name: "Favourite color".to_string(),
            description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
            options: Vec::new(),
        },
        Poll {
            name: "Best food".to_string(),
            description: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string(),
            options: Vec::new(),
        },
    ];

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
                    name,
                    description,
                    options,
                });

                tx.send(id).unwrap();
            }
        }
    }
}
