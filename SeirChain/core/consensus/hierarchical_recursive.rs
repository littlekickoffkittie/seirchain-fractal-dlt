use std::collections::HashMap;
use crate::core::consensus::proof_of_fractal::ProofOfFractal;
use crate::core::security::redundant_paths::RedundantPathSecurity;

/// HierarchicalRecursiveConsensus implements recursive PBFT-like consensus for SeirChain.
pub struct HierarchicalRecursiveConsensus {
    pub nodes: Vec<String>, // List of node IDs in the sub-fractal
    pub state: HashMap<String, String>, // State per node (e.g., votes, messages)
    pub fault_tolerance: usize, // Number of tolerated faulty nodes
    pub proof: ProofOfFractal, // Proof-of-Fractal puzzle instance
    pub security: RedundantPathSecurity, // Security module instance
    pub children: Vec<HierarchicalRecursiveConsensus>, // Child sub-fractals
}

impl HierarchicalRecursiveConsensus {
    /// Creates a new HRC instance with given nodes and fault tolerance.
    pub fn new(nodes: Vec<String>, fault_tolerance: usize, difficulty: u32, depth: u32) -> Self {
        let proof = ProofOfFractal::new(difficulty);
        proof.adjust_difficulty(1000);

        let children = if depth > 0 {
            nodes.chunks(nodes.len() / 3).map(|chunk| {
                HierarchicalRecursiveConsensus::new(chunk.to_vec(), fault_tolerance, difficulty, depth - 1)
            }).collect()
        } else {
            Vec::new()
        };

        HierarchicalRecursiveConsensus {
            nodes,
            state: HashMap::new(),
            fault_tolerance,
            proof,
            security: RedundantPathSecurity::new(),
            children,
        }
    }


    /// Runs the recursive consensus algorithm with simulated network communication and fault handling.
    pub fn run_consensus<R: rand::Rng>(&mut self, rng: &mut R) -> bool {
        // If this is not a leaf node, run consensus on children first.
        if !self.children.is_empty() {
            let mut child_results = Vec::new();
            for child in &mut self.children {
                child_results.push(child.run_consensus(rng));
            }
            // Aggregate results from children. For simplicity, we require all children to agree.
            if child_results.iter().all(|&r| r) {
                return true;
            } else {
                return false;
            }
        }

        // Leaf node consensus (PBFT-like simulation)
        let proposal = "block_data";

        if !self.proof.solve_puzzle(proposal.as_bytes()) {
            return false;
        }

        for node in &self.nodes {
            let vote = if rng.gen::<f32>() < 0.9 { "vote" } else { "fault" };
            self.state.insert(node.clone(), vote.to_string());
            if vote == "vote" {
                self.security.add_path(node);
            } else {
                self.security.remove_path(node);
            }
        }

        let votes = self.state.values().filter(|&v| v == "vote").count();
        let faults = self.state.values().filter(|&v| v == "fault").count();

        if faults > self.fault_tolerance || !self.security.validate_paths() {
            return false;
        }

        let quorum = 2 * self.fault_tolerance + 1;
        votes >= quorum && self.validate_subfractal()
    }

    /// Validates the sub-fractal consensus state.
    pub fn validate_subfractal(&self) -> bool {
        self.nodes.len() >= 3 * self.fault_tolerance + 1 && self.proof.verify_solution("block_data".as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_hierarchical_recursive_consensus() {
        let nodes = vec![
            "node1".to_string(), "node2".to_string(), "node3".to_string(), "node4".to_string(),
            "node5".to_string(), "node6".to_string(), "node7".to_string(), "node8".to_string(),
            "node9".to_string(), "node10".to_string(), "node11".to_string(), "node12".to_string(),
        ];
        let mut hrc = HierarchicalRecursiveConsensus::new(nodes, 1, 4, 1);
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        assert!(hrc.run_consensus(&mut rng));
    }

    #[test]
    fn test_leaf_consensus() {
        let nodes = vec!["node1".to_string(), "node2".to_string(), "node3".to_string(), "node4".to_string()];
        let mut hrc = HierarchicalRecursiveConsensus::new(nodes, 1, 4, 0);
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        assert!(hrc.run_consensus(&mut rng));
        assert!(hrc.validate_subfractal());
    }
}
