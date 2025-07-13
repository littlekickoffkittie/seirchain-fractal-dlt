use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::core::triad_matrix::triad_structure::{Transaction};

/// Represents an activity related to a Triad.
#[derive(Clone, Debug, PartialEq)]
pub enum TriadActivity {
    /// A transaction was added to a Triad.
    TransactionAdded(Transaction),
    /// Consensus was reached, with a description or ID.
    ConsensusReached(String),
    /// Proof of Fractal puzzle was solved with a nonce.
    ProofOfFractalSolved(u64),
    /// Other activity with a description.
    Other(String),
}

/// TriadExplorer keeps track of all activities related to Triads.
pub struct TriadExplorer {
    activities: Arc<Mutex<VecDeque<TriadActivity>>>,
    max_history: usize,
}

impl TriadExplorer {
    /// Creates a new TriadExplorer with a maximum history size.
    pub fn new(max_history: usize) -> Self {
        TriadExplorer {
            activities: Arc::new(Mutex::new(VecDeque::with_capacity(max_history))),
            max_history,
        }
    }

    /// Adds a new activity to the explorer.
    pub fn add_activity(&self, activity: TriadActivity) {
        let mut activities = self.activities.lock().unwrap();
        if activities.len() == self.max_history {
            activities.pop_front();
        }
        activities.push_back(activity);
    }

    /// Returns a snapshot of recent activities.
    pub fn get_recent_activities(&self) -> Vec<TriadActivity> {
        let activities = self.activities.lock().unwrap();
        activities.iter().cloned().collect()
    }

    /// Clears all recorded activities.
    pub fn clear_activities(&self) {
        let mut activities = self.activities.lock().unwrap();
        activities.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::triad_matrix::triad_structure::Transaction;

    #[test]
    fn test_add_and_get_activities() {
        let explorer = TriadExplorer::new(10);
        let tx = Transaction {
            sender: "wallet1".to_string(),
            receiver: "wallet2".to_string(),
            amount: 50,
            timestamp: 1234567890,
        };
        explorer.add_activity(TriadActivity::TransactionAdded(tx.clone()));
        explorer.add_activity(TriadActivity::ConsensusReached("Consensus1".to_string()));

        let activities = explorer.get_recent_activities();
        assert_eq!(activities.len(), 2);
        match &activities[0] {
            TriadActivity::TransactionAdded(t) => assert_eq!(t.sender, "wallet1"),
            _ => panic!("Expected TransactionAdded activity"),
        }
        match &activities[1] {
            TriadActivity::ConsensusReached(desc) => assert_eq!(desc, "Consensus1"),
            _ => panic!("Expected ConsensusReached activity"),
        }
    }

    #[test]
    fn test_max_history_limit() {
        let explorer = TriadExplorer::new(3);
        for i in 0..5 {
            explorer.add_activity(TriadActivity::Other(format!("Activity {}", i)));
        }
        let activities = explorer.get_recent_activities();
        assert_eq!(activities.len(), 3);
        assert_eq!(activities[0], TriadActivity::Other("Activity 2".to_string()));
    }
}
