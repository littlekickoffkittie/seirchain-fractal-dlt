use std::sync::{Arc, Mutex};
use crate::interface::economics::waclanium_token::WaclaniumToken;
use crate::interface::ledger::ledger::Ledger;
use crate::interface::wallet::wallet::Wallet;

pub fn create_sample_wallet() -> Wallet {
    // Create a WaclaniumToken instance with initial supply and max supply
    let token = Arc::new(Mutex::new(WaclaniumToken::new(1000, 10000)));

    // Create a Ledger instance with max history
    let ledger = Arc::new(Ledger::new(100));

    // Create the Wallet instance
    let wallet = Wallet::new(token.clone(), ledger.clone());

    // Mint some tokens to a user
    {
        let mut token_lock = token.lock().unwrap();
        token_lock.mint("user1", 500).unwrap();
    }

    wallet
}

impl Wallet {
    pub fn create_address_with_w(&self, user_id: String) -> String {
        let rand_string: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(29) // 29 chars + 'W' prefix = 30 chars total
            .map(char::from)
            .collect();
        let address = format!("W{}", rand_string);
        let mut addresses = self.addresses.lock().unwrap();
        addresses.insert(address.clone(), user_id);
        address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet_and_address_with_w() {
        let wallet = create_sample_wallet();

        // Create a new address for user1 with 'W' prefix
        let address = wallet.create_address_with_w("user1".to_string());

        // Check that address starts with 'W'
        assert!(address.starts_with('W'));

        // Check initial balance for the address
        let balance = wallet.get_balance(&address);
        assert_eq!(balance, 500);

        // Create another address and transfer tokens
        let address2 = wallet.create_address_with_w("user2".to_string());
        wallet.transfer(&address, &address2, 200).unwrap();

        // Check balances after transfer
        assert_eq!(wallet.get_balance(&address), 300);
        assert_eq!(wallet.get_balance(&address2), 200);
    }
}
