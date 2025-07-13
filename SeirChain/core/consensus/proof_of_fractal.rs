
use sha2::{Digest, Sha256};
use std::fmt;
use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

/// ProofOfFractal represents the Proof-of-Fractal puzzle state and logic.
pub struct ProofOfFractal {
    pub nonce: AtomicU64,
    pub difficulty: Mutex<u32>,
    pub target: Mutex<[u8; 32]>,
    pub hash: Mutex<[u8; 32]>,
}

impl ProofOfFractal {
    /// Creates a new ProofOfFractal with the given difficulty.
    /// Difficulty can be adjusted dynamically based on Triad count or other metrics.
    pub fn new(difficulty: u32) -> Self {
        let target = ProofOfFractal::calculate_target(difficulty);
        ProofOfFractal {
            nonce: AtomicU64::new(0),
            difficulty: Mutex::new(difficulty),
            target: Mutex::new(target),
            hash: Mutex::new([0u8; 32]),
        }
    }

    /// Adjusts difficulty based on Triad count.
    /// For example, difficulty increases logarithmically with triad_count.
    pub fn adjust_difficulty(&self, triad_count: u64) {
        let base_difficulty = 4;
        let adjusted = base_difficulty + (64 - triad_count.leading_zeros()) as u32;
        let new_difficulty = adjusted.min(32);
        let mut difficulty_guard = self.difficulty.lock().unwrap();
        *difficulty_guard = new_difficulty;
        let mut target_guard = self.target.lock().unwrap();
        *target_guard = ProofOfFractal::calculate_target(new_difficulty);
    }

    /// Calculates the target hash based on difficulty.
    /// Higher difficulty means more leading zeros in the target.
    fn calculate_target(difficulty: u32) -> [u8; 32] {
        let mut target = [0xffu8; 32];
        let byte_count = (difficulty / 8) as usize;
        let bit_count = (difficulty % 8) as usize;

        for i in 0..byte_count {
            target[i] = 0x00;
        }
        if byte_count < 32 {
            target[byte_count] = 0xff >> bit_count;
        }
        target
    }

    /// Attempts to solve the PoF puzzle by finding a nonce that produces a hash below the target.
    /// Returns true if a valid nonce is found.
    pub fn solve_puzzle(&self, data: &[u8]) -> bool {
        let mut rng = OsRng;
        loop {
            let nonce_candidate = rng.next_u64();
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.update(&nonce_candidate.to_le_bytes());
            let result = hasher.finalize();
            let mut hash_arr = [0u8; 32];
            hash_arr.copy_from_slice(&result);

            let target_guard = self.target.lock().unwrap();
            if ProofOfFractal::hash_meets_target(&hash_arr, &target_guard) {
                self.nonce.store(nonce_candidate, Ordering::SeqCst);
                let mut hash_guard = self.hash.lock().unwrap();
                *hash_guard = hash_arr;
                return true;
            }
            // Optional: Add early exit or yield to other threads in real implementation
        }
    }

    /// Checks if the given hash meets the target difficulty.
    fn hash_meets_target(hash: &[u8; 32], target: &[u8; 32]) -> bool {
        for (h, t) in hash.iter().zip(target.iter()) {
            if h < t {
                return true;
            } else if h > t {
                return false;
            }
        }
        true
    }

    /// Verifies that the current nonce produces a valid hash below the target.
    pub fn verify_solution(&self, data: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let nonce_val = self.nonce.load(Ordering::SeqCst);
        hasher.update(&nonce_val.to_le_bytes());
        let result = hasher.finalize();
        let mut hash_arr = [0u8; 32];
        hash_arr.copy_from_slice(&result);

        let target_guard = self.target.lock().unwrap();
        ProofOfFractal::hash_meets_target(&hash_arr, &target_guard)
    }

    /// Resets the PoF state.
    pub fn reset(&self) {
        self.nonce.store(0, Ordering::SeqCst);
        let mut hash_guard = self.hash.lock().unwrap();
        *hash_guard = [0u8; 32];
    }

    /// Returns the current hash as a hexadecimal string.
    pub fn hash_hex(&self) -> String {
        let hash_guard = self.hash.lock().unwrap();
        hex::encode(*hash_guard)
    }
}

impl fmt::Display for ProofOfFractal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nonce_val = self.nonce.load(Ordering::SeqCst);
        let difficulty_guard = self.difficulty.lock().unwrap();
        write!(
            f,
            "PoF(nonce: {}, difficulty: {}, hash: {})",
            nonce_val,
            *difficulty_guard,
            self.hash_hex()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_proof_of_fractal() {
        let pof = ProofOfFractal::new(4);
        assert_eq!(*pof.difficulty.lock().unwrap(), 4);
        let target = ProofOfFractal::calculate_target(4);
        assert_eq!(*pof.target.lock().unwrap(), target);
    }

    #[test]
    fn test_adjust_difficulty() {
        let pof = ProofOfFractal::new(4);
        pof.adjust_difficulty(1000);
        let difficulty = *pof.difficulty.lock().unwrap();
        assert!(difficulty > 4);
    }

    #[test]
    fn test_solve_and_verify() {
        let pof = ProofOfFractal::new(8); // Use a low difficulty for testing
        let data = b"test data";
        assert!(pof.solve_puzzle(data));
        assert!(pof.verify_solution(data));
    }

    #[test]
    fn test_reset() {
        let pof = ProofOfFractal::new(4);
        pof.solve_puzzle(b"test");
        pof.reset();
        assert_eq!(pof.nonce.load(Ordering::SeqCst), 0);
        assert_eq!(*pof.hash.lock().unwrap(), [0u8; 32]);
    }
}
