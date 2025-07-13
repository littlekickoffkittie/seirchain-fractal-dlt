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
    /// Votes mapped by proposal ID to votes for and against.
    votes: HashMap<u64, (HashSet<String>, HashSet<String>)>,
    /// Next proposal ID to assign.
    next_proposal_id: u64,
    /// Transaction fee.
    fee: u64,
}

impl WaclaniumToken {
    /// Creates a new WaclaniumToken with initial and max supply.
    pub fn new(initial_supply: u64, max_supply: u64, fee: u64) -> Self {
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
            fee,
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
        self.votes.insert(proposal_id, (HashSet::new(), HashSet::new()));
        self.next_proposal_id += 1;
        proposal_id
    }

    /// Casts a vote for a proposal by a user.
    /// Returns an error if proposal does not exist, user has no stake, or user already voted.
    pub fn cast_vote(&mut self, user: &str, proposal_id: u64, vote_for: bool) -> Result<(), String> {
        if !self.governance_proposals.contains_key(&proposal_id) {
            return Err("Proposal does not exist".to_string());
        }
        let stake = self.stakes.get(user).cloned().unwrap_or(0);
        if stake == 0 {
            return Err("User has no stake to vote".to_string());
        }
        let (voters_for, voters_against) = self.votes.get_mut(&proposal_id).unwrap();
        if voters_for.contains(user) || voters_against.contains(user) {
            return Err("User has already voted".to_string());
        }
        if vote_for {
            voters_for.insert(user.to_string());
        } else {
            voters_against.insert(user.to_string());
        }
        Ok(())
    }

    /// Checks if a user has voted on a proposal.
    pub fn has_voted(&self, user: &str, proposal_id: u64) -> bool {
        if let Some((voters_for, voters_against)) = self.votes.get(&proposal_id) {
            voters_for.contains(user) || voters_against.contains(user)
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

    /// Calculates the voting power of a user based on their stake.
    /// The voting power is the square root of the stake.
    pub fn get_voting_power(&self, user: &str) -> u64 {
        let stake = self.get_stake(user);
        (stake as f64).sqrt() as u64
    }

    /// Tallies votes for a proposal and returns the total voting power for and against.
    /// Returns None if the proposal does not exist.
    pub fn tally_votes(&self, proposal_id: u64) -> Option<(u64, u64)> {
        self.votes.get(&proposal_id).map(|(voters_for, voters_against)| {
            let votes_for = voters_for
                .iter()
                .map(|voter| self.get_voting_power(voter))
                .sum();
            let votes_against = voters_against
                .iter()
                .map(|voter| self.get_voting_power(voter))
                .sum();
            (votes_for, votes_against)
        })
    }

    /// Checks if a proposal has passed.
    /// A proposal passes if the total voting power of "for" votes is greater than the total voting power of "against" votes.
    /// Returns None if the proposal does not exist.
    pub fn proposal_passed(&self, proposal_id: u64) -> Option<bool> {
        self.tally_votes(proposal_id)
            .map(|(votes_for, votes_against)| votes_for > votes_against)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_waclanium_token() {
        let token = WaclaniumToken::new(1000, 10000, 1);
        assert_eq!(token.get_balance("genesis"), 1000);
    }

    #[test]
    fn test_transfer() {
        let mut token = WaclaniumToken::new(1000, 10000, 1);
        token.mint("user1", 100).unwrap();
        assert!(token.transfer("user1", "user2", 50).is_ok());
        assert_eq!(token.get_balance("user1"), 50);
        assert_eq!(token.get_balance("user2"), 50);
    }

    #[test]
    fn test_stake_and_unstake() {
        let mut token = WaclaniumToken::new(1000, 10000, 1);
        token.mint("user1", 100).unwrap();
        assert!(token.stake("user1", 50).is_ok());
        assert_eq!(token.get_balance("user1"), 50);
        assert_eq!(token.get_stake("user1"), 50);
        assert!(token.unstake("user1", 50).is_ok());
        assert_eq!(token.get_balance("user1"), 100);
        assert_eq!(token.get_stake("user1"), 0);
    }

    #[test]
    fn test_mint() {
        let mut token = WaclaniumToken::new(1000, 10000, 1);
        assert!(token.mint("user1", 100).is_ok());
        assert_eq!(token.get_balance("user1"), 100);
        assert_eq!(token.total_supply, 1100);
    }

    #[test]
    fn test_governance() {
        let mut token = WaclaniumToken::new(1000, 10000, 1);
        token.mint("user1", 100).unwrap();
        token.stake("user1", 100).unwrap();
        token.mint("user2", 100).unwrap();
        token.stake("user2", 25).unwrap();
        let proposal_id = token.create_proposal("Test proposal".to_string());
        assert!(token.cast_vote("user1", proposal_id, true).is_ok());
        assert!(token.cast_vote("user2", proposal_id, false).is_ok());
        assert!(token.has_voted("user1", proposal_id));
        assert!(token.has_voted("user2", proposal_id));
        assert_eq!(token.tally_votes(proposal_id), Some((10, 5)));
        assert_eq!(token.proposal_passed(proposal_id), Some(true));
    }
}
