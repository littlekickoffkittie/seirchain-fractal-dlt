use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::{distributions::Alphanumeric, Rng};
use crate::interface::economics::waclanium_token::WaclaniumToken;
use crate::interface::ledger::ledger::{Ledger, LedgerTransaction};
use crate::interface::explorer::triad_explorer::{TriadExplorer, TriadActivity};
use std::time::{SystemTime, UNIX_EPOCH};

/// Wallet struct manages wallet addresses and integrates with token, ledger, and explorer.
pub struct Wallet {
    /// Mapping from wallet address to user identifier (optional).
    pub addresses: Arc<Mutex<HashMap<String, String>>>,
    /// Reference to the WaclaniumToken instance for balance and transfers.
    pub token: Arc<Mutex<WaclaniumToken>>,
    /// Reference to the Ledger instance for recording transactions.
    pub ledger: Arc<Ledger>,
    /// Reference to the TriadExplorer for logging wallet activities.
    pub explorer: Arc<TriadExplorer>,
}

impl Wallet {
    /// Creates a new Wallet instance.
    pub fn new(token: Arc<Mutex<WaclaniumToken>>, ledger: Arc<Ledger>, explorer: Arc<TriadExplorer>) -> Self {
        Wallet {
            addresses: Arc::new(Mutex::new(HashMap::new())),
            token,
            ledger,
            explorer,
        }
    }

    /// Generates a new wallet address with "w" prefix.
    /// Logs the wallet creation event on the ledger and explorer.
    pub fn create_address(&self, user_id: String) -> String {
        let rand_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let address = format!("w{}", rand_string);
        let mut addresses = self.addresses.lock().unwrap();
        addresses.insert(address.clone(), user_id.clone());

        // Log wallet creation as a ledger transaction
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let tx = LedgerTransaction {
            from: "system".to_string(),
            to: address.clone(),
            amount: 0,
            timestamp,
        };
        self.ledger.add_transaction(tx.clone());

        // Log wallet creation event to explorer
        self.explorer.add_activity(TriadActivity::Other(format!("Wallet created: {}", address)));

        address
    }

    /// Gets the balance of a wallet address.
    pub fn get_balance(&self, address: &str) -> u64 {
        let addresses = self.addresses.lock().unwrap();
        if let Some(user_id) = addresses.get(address) {
            let token = self.token.lock().unwrap();
            token.get_balance(user_id)
        } else {
            0
        }
    }

    /// Transfers tokens from one wallet address to another.
    /// Returns error if addresses are invalid or transfer fails.
    /// Logs the transfer on the ledger and explorer.
    pub fn transfer(&self, from_address: &str, to_address: &str, amount: u64) -> Result<(), String> {
        let addresses = self.addresses.lock().unwrap();
        let from_user = addresses.get(from_address).ok_or("Invalid from address")?;
        let to_user = addresses.get(to_address).ok_or("Invalid to address")?;

        let mut token = self.token.lock().unwrap();
        token.transfer(from_user, to_user, amount)?;

        // Record transaction in ledger
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let tx = LedgerTransaction {
            from: from_address.to_string(),
            to: to_address.to_string(),
            amount,
            timestamp,
        };
        self.ledger.add_transaction(tx.clone());

        // Convert LedgerTransaction to Transaction for explorer logging
        let explorer_tx = crate::core::triad_matrix::triad_structure::Transaction {
            sender: tx.from.clone(),
            receiver: tx.to.clone(),
            amount: tx.amount,
            timestamp: tx.timestamp,
        };

        // Log transfer event to explorer
        self.explorer.add_activity(TriadActivity::TransactionAdded(explorer_tx));

        Ok(())
    }

    /// Returns recent transactions involving the given wallet address.
    pub fn get_transaction_history(&self, address: &str) -> Vec<LedgerTransaction> {
        let transactions = self.ledger.get_recent_transactions();
        transactions.into_iter()
            .filter(|tx| tx.from == address || tx.to == address)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::economics::waclanium_token::WaclaniumToken;
    use crate::interface::ledger::ledger::Ledger;
    use crate::interface::explorer::triad_explorer::TriadExplorer;

    #[test]
    fn test_wallet_create_and_transfer() {
        let token = Arc::new(Mutex::new(WaclaniumToken::new(1000, 10000)));
        let ledger = Arc::new(Ledger::new(100));
        let explorer = Arc::new(TriadExplorer::new(100));
        let wallet = Wallet::new(token.clone(), ledger.clone(), explorer.clone());

        let addr1 = wallet.create_address("user1".to_string());
        let addr2 = wallet.create_address("user2".to_string());

        // Mint tokens to user1
        {
            let mut token_lock = token.lock().unwrap();
            token_lock.mint("user1", 500).unwrap();
        }

        // Check initial balances
        assert_eq!(wallet.get_balance(&addr1), 500);
        assert_eq!(wallet.get_balance(&addr2), 0);

        // Transfer tokens from addr1 to addr2
        wallet.transfer(&addr1, &addr2, 200).unwrap();

        // Check balances after transfer
        assert_eq!(wallet.get_balance(&addr1), 300);
        assert_eq!(wallet.get_balance(&addr2), 200);

        // Check transaction history for addr1 and addr2
        let txs1 = wallet.get_transaction_history(&addr1);
        let txs2 = wallet.get_transaction_history(&addr2);
        assert_eq!(txs1.len(), 1);
        assert_eq!(txs2.len(), 1);
        assert_eq!(txs1[0].amount, 200);
        assert_eq!(txs2[0].amount, 200);
    }
}
