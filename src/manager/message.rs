use tokio::sync::{mpsc, oneshot};

use super::PollOverview;

/// All possible message variations.
pub enum Message {
    /// List all available polls.
    GetSummaries {
        tx: oneshot::Sender<Vec<PollOverview>>,
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
        tx: oneshot::Sender<Option<PollOverview>>,
    },

    /// Subscribe to vote changes on a given poll.
    RegisterPoll {
        poll: usize,
        tx: mpsc::Sender<Vec<usize>>,
    },

    /// Subscribe to an overview of all polls.
    RegisterOverview { tx: mpsc::Sender<Vec<PollOverview>> },

    /// Subscribe to a list of all poll totals.
    RegisterPollTotal { tx: mpsc::Sender<Vec<usize>> },

    /// Vote on a poll.
    Vote {
        poll: usize,
        option: usize,
        tx: oneshot::Sender<bool>,
    },
}
