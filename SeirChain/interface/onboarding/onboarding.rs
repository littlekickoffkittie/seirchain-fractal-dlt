use std::sync::{Arc, Mutex};
use crate::interface::ledger::ledger::Ledger;

/// Onboarding guides new users through initial setup.
pub struct Onboarding {
    _ledger: Arc<Mutex<Ledger>>,
}

impl Onboarding {
    /// Creates a new Onboarding instance with given ledger.
    pub fn new(ledger: Arc<Mutex<Ledger>>) -> Self {
        Onboarding { _ledger: ledger }
    }

    /// Runs the onboarding process interactively.
    pub fn run(&mut self) {
        println!("Welcome to SeirChain Onboarding!");
        println!("Onboarding complete. You can now start using SeirChain.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::ledger::ledger::Ledger;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_onboarding_creation() {
        let ledger = Arc::new(Mutex::new(Ledger::new(1000)));
        let mut onboarding = Onboarding::new(Arc::clone(&ledger));
        assert!(true);
    }
}
