use std::collections::{HashMap, HashSet};

/// WaclaniumToken manages balances, stakes, and governance for the WAC token.
pub struct WaclaniumToken {
    /// Mapping of user IDs to token balances.
    balances: HashMap<String, u64>,
    /// Mapping of user IDs to staked token amounts.
    stakes: HashMap<String, u64>,
    /// Total supply of tokens in circulation.
    total_supply: u64,
    /// Maximum allowed token supply.
    max_supply: u64,
    /// Governance proposals mapped by proposal ID to description.
    governance_proposals: HashMap<u64, String>,
    /// Votes mapped by proposal ID to set of voters.
    votes: HashMap<u64, HashSet<String>>,
    /// Next proposal ID to assign.
    next_proposal_id: u64,
}

impl WaclaniumToken {
    /// Creates a new WaclaniumToken with initial and max supply.
    pub fn new(initial_supply: u64, max_supply: u64) -> Self {
        let mut balances = HashMap::new();
        balances.insert("genesis".to_string(), initial_supply);
        WaclaniumToken {
            balances,
            stakes: HashMap::new(),
            total_supply: initial_supply,
            max_supply,
            governance_proposals: HashMap::new(),
            votes: HashMap::new(),
            next_proposal_id: 1,
        }
    }

    /// Returns an immutable reference to balances.
    pub fn balances(&self) -> &HashMap<String, u64> {
        &self.balances
    }

    /// Returns an immutable reference to stakes.
    pub fn stakes(&self) -> &HashMap<String, u64> {
        &self.stakes
    }

    /// Creates a new governance proposal.
    /// Returns the proposal ID.
    pub fn create_proposal(&mut self, description: String) -> u64 {
        let proposal_id = self.next_proposal_id;
        self.governance_proposals.insert(proposal_id, description);
        self.votes.insert(proposal_id, HashSet::new());
        self.next_proposal_id += 1;
        proposal_id
    }

    /// Casts a vote for a proposal by a user.
    /// Returns an error if proposal does not exist, user has no stake, or user already voted.
    pub fn cast_vote(&mut self, user: &str, proposal_id: u64) -> Result<(), String> {
        if !self.governance_proposals.contains_key(&proposal_id) {
            return Err("Proposal does not exist".to_string());
        }
        let stake = self.stakes.get(user).cloned().unwrap_or(0);
        if stake == 0 {
            return Err("User has no stake to vote".to_string());
        }
        let voters = self.votes.get_mut(&proposal_id).unwrap();
        if voters.contains(user) {
            return Err("User has already voted".to_string());
        }
        voters.insert(user.to_string());
        Ok(())
    }

    /// Checks if a user has voted on a proposal.
    pub fn has_voted(&self, user: &str, proposal_id: u64) -> bool {
        if let Some(voters) = self.votes.get(&proposal_id) {
            voters.contains(user)
        } else {
            false
        }
    }

    /// Transfers tokens from one user to another.
    /// Returns an error if sender has insufficient balance.
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let from_balance = self.balances.get(from).cloned().unwrap_or(0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        self.balances.insert(from.to_string(), from_balance - amount);
        let to_balance = self.balances.get(to).cloned().unwrap_or(0);
        self.balances.insert(to.to_string(), to_balance + amount);
        Ok(())
    }

    /// Stakes tokens for a user.
    /// Returns an error if user has insufficient balance.
    pub fn stake(&mut self, user: &str, amount: u64) -> Result<(), String> {
        let balance = self.balances.get(user).cloned().unwrap_or(0);
        if balance < amount {
            return Err("Insufficient balance to stake".to_string());
        }
        self.balances.insert(user.to_string(), balance - amount);
        let stake = self.stakes.get(user).cloned().unwrap_or(0);
        self.stakes.insert(user.to_string(), stake + amount);
        Ok(())
    }

    /// Unstakes tokens for a user.
    /// Returns an error if user has insufficient stake.
    pub fn unstake(&mut self, user: &str, amount: u64) -> Result<(), String> {
        let stake = self.stakes.get(user).cloned().unwrap_or(0);
        if stake < amount {
            return Err("Insufficient stake to unstake".to_string());
        }
        self.stakes.insert(user.to_string(), stake - amount);
        let balance = self.balances.get(user).cloned().unwrap_or(0);
        self.balances.insert(user.to_string(), balance + amount);
        Ok(())
    }

    /// Mints new tokens up to the max supply.
    /// Returns an error if minting exceeds max supply.
    pub fn mint(&mut self, to: &str, amount: u64) -> Result<(), String> {
        if self.total_supply + amount > self.max_supply {
            return Err("Minting amount exceeds max supply".to_string());
        }
        let balance = self.balances.get(to).cloned().unwrap_or(0);
        self.balances.insert(to.to_string(), balance + amount);
        self.total_supply += amount;
        Ok(())
    }

    /// Gets the balance of a user.
    pub fn get_balance(&self, user: &str) -> u64 {
        self.balances.get(user).cloned().unwrap_or(0)
    }

    /// Gets the stake of a user.
    pub fn get_stake(&self, user: &str) -> u64 {
        self.stakes.get(user).cloned().unwrap_or(0)
    }

    /// Placeholder for quadratic voting logic.
    /// Returns true if votes are less than or equal to user's stake.
    pub fn governance_vote(&self, user: &str, votes: u64) -> bool {
        let stake = self.stakes.get(user).cloned().unwrap_or(0);
        votes <= stake
    }

    /// Tallies votes for a proposal and returns the number of votes cast.
    /// Returns None if the proposal does not exist.
    pub fn tally_votes(&self, proposal_id: u64) -> Option<usize> {
        self.votes.get(&proposal_id).map(|voters| voters.len())
    }

    /// Checks if a proposal has passed based on a simple majority.
    /// Returns None if the proposal does not exist.
    pub fn proposal_passed(&self, proposal_id: u64) -> Option<bool> {
        let total_votes = self.tally_votes(proposal_id)?;
        // For simplicity, assume majority is more than half of total staked tokens
        let total_stake: u64 = self.stakes.values().sum();
        Some((total_votes as u64) > total_stake / 2)
    }
}
