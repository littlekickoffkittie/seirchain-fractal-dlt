// triad_structure.rs
// Defines the Triad structure, core dependency for SeirChain

use sha2::{Digest, Sha256};

pub struct Triad {
    pub transactions: Vec<Transaction>,
    pub child_references: [Option<Box<Triad>>; 3],
    pub merkle_root: [u8; 32],
    pub proof_of_fractal_data: ProofOfFractalData,
    pub parent_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
}

pub struct ProofOfFractalData {
    pub nonce: u64,
    pub difficulty: u32,
    pub hash: [u8; 32],
}

impl Triad {
    pub fn new() -> Self {
        Triad {
            transactions: Vec::new(),
            child_references: [None, None, None],
            merkle_root: [0u8; 32],
            proof_of_fractal_data: ProofOfFractalData::new(),
            parent_hash: [0u8; 32],
        }
    }

    /// Creates the Genesis Triad with no parent hash, empty children, and optionally initial transactions.
    pub fn genesis(initial_transactions: Option<Vec<Transaction>>) -> Self {
        let mut triad = Triad {
            transactions: initial_transactions.unwrap_or_else(Vec::new),
            child_references: [None, None, None],
            merkle_root: [0u8; 32],
            proof_of_fractal_data: ProofOfFractalData::new(),
            parent_hash: [0u8; 32], // No parent for genesis
        };
        triad.calculate_merkle_root();
        triad
    }

    pub fn calculate_merkle_root(&mut self) {
        let mut hashes: Vec<[u8; 32]> = self.transactions.iter()
            .map(|tx| tx.hash())
            .collect();

        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            for i in (0..hashes.len()).step_by(2) {
                let left = hashes[i];
                let right = if i + 1 < hashes.len() { hashes[i + 1] } else { left };
                let mut hasher = Sha256::new();
                hasher.update(&left);
                hasher.update(&right);
                let result = hasher.finalize();
                let mut hash_arr = [0u8; 32];
                hash_arr.copy_from_slice(&result);
                new_hashes.push(hash_arr);
            }
            hashes = new_hashes;
        }

        self.merkle_root = if !hashes.is_empty() { hashes[0] } else { [0u8; 32] };
    }

    pub fn insert_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        self.calculate_merkle_root();
    }

    pub fn add_child(&mut self, index: usize, child: Triad) -> Result<(), String> {
        if index >= 3 {
            return Err("Child index must be 0, 1, or 2".to_string());
        }
        if self.child_references[index].is_some() {
            return Err("Child already exists at this index".to_string());
        }
        self.child_references[index] = Some(Box::new(child));
        Ok(())
    }

    pub fn remove_child(&mut self, index: usize) -> Result<(), String> {
        if index >= 3 {
            return Err("Child index must be 0, 1, or 2".to_string());
        }
        if self.child_references[index].is_none() {
            return Err("No child exists at this index".to_string());
        }
        self.child_references[index] = None;
        Ok(())
    }

    pub fn get_child(&self, index: usize) -> Option<&Triad> {
        if index >= 3 {
            return None;
        }
        self.child_references[index].as_deref()
    }

    pub fn get_all_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
        self.merkle_root = [0u8; 32];
    }
}

impl Transaction {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.sender.as_bytes());
        hasher.update(self.receiver.as_bytes());
        hasher.update(&self.amount.to_le_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        let result = hasher.finalize();
        let mut hash_arr = [0u8; 32];
        hash_arr.copy_from_slice(&result);
        hash_arr
    }
}

impl ProofOfFractalData {
    pub fn new() -> Self {
        ProofOfFractalData {
            nonce: 0,
            difficulty: 1,
            hash: [0u8; 32],
        }
    }
}
