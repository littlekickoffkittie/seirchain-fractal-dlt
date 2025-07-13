use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Represents a transaction record in the ledger.
#[derive(Clone, Debug)]
pub struct LedgerTransaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: u64,
}

/// Ledger maintains an immutable record of transactions and state changes.
pub struct Ledger {
    transactions: Arc<Mutex<VecDeque<LedgerTransaction>>>,
    max_history: usize,
}

impl Ledger {
    /// Creates a new Ledger with a maximum history size.
    pub fn new(max_history: usize) -> Self {
        Ledger {
            transactions: Arc::new(Mutex::new(VecDeque::with_capacity(max_history))),
            max_history,
        }
    }

    /// Adds a new transaction record to the ledger.
    pub fn add_transaction(&self, tx: LedgerTransaction) {
        let mut transactions = self.transactions.lock().unwrap();
        if transactions.len() == self.max_history {
            transactions.pop_front();
        }
        transactions.push_back(tx);
    }

    /// Returns a snapshot of recent transactions.
    pub fn get_recent_transactions(&self) -> Vec<LedgerTransaction> {
        let transactions = self.transactions.lock().unwrap();
        transactions.iter().cloned().collect()
    }

    /// Clears all recorded transactions.
    pub fn clear_transactions(&self) {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_transactions() {
        let ledger = Ledger::new(10);
        let tx1 = LedgerTransaction {
            from: "wallet1".to_string(),
            to: "wallet2".to_string(),
            amount: 100,
            timestamp: 1234567890,
        };
        let tx2 = LedgerTransaction {
            from: "wallet2".to_string(),
            to: "wallet3".to_string(),
            amount: 50,
            timestamp: 1234567891,
        };
        ledger.add_transaction(tx1.clone());
        ledger.add_transaction(tx2.clone());

        let transactions = ledger.get_recent_transactions();
        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].from, "wallet1");
        assert_eq!(transactions[1].to, "wallet3");
    }
}
