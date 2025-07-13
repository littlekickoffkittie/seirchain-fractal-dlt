use std::collections::HashSet;
use sha2::{Digest, Sha256};

/// RedundantPathSecurity manages multi-path validation and node promotion for security using cryptographic hashes.
pub struct RedundantPathSecurity {
    /// Set of active redundant path hashes.
    pub active_paths: HashSet<[u8; 32]>,
    /// Set of promoted node hashes.
    pub promoted_nodes: HashSet<[u8; 32]>,
}

impl RedundantPathSecurity {
    /// Creates a new RedundantPathSecurity instance with empty active paths and promoted nodes.
    pub fn new() -> Self {
        RedundantPathSecurity {
            active_paths: HashSet::new(),
            promoted_nodes: HashSet::new(),
        }
    }

    /// Hashes a given ID string using SHA-256.
    fn hash_id(id: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        let result = hasher.finalize();
        let mut hash_arr = [0u8; 32];
        hash_arr.copy_from_slice(&result);
        hash_arr
    }

    /// Validates that there are active redundant paths.
    /// Returns true if there is at least one active path.
    pub fn validate_paths(&self) -> bool {
        !self.active_paths.is_empty()
    }

    /// Adds a new active path by its ID.
    /// Returns true if the path was newly inserted.
    pub fn add_path(&mut self, path_id: String) -> bool {
        let hash = Self::hash_id(&path_id);
        self.active_paths.insert(hash)
    }

    /// Removes an active path by its ID.
    /// Returns true if the path was present and removed.
    pub fn remove_path(&mut self, path_id: &str) -> bool {
        let hash = Self::hash_id(path_id);
        self.active_paths.remove(&hash)
    }

    /// Promotes a node by its ID.
    /// Returns true if the node was newly promoted.
    pub fn promote_node(&mut self, node_id: &str) -> bool {
        let hash = Self::hash_id(node_id);
        self.promoted_nodes.insert(hash)
    }

    /// Removes a promoted node by its ID.
    /// Returns true if the node was present and removed.
    pub fn remove_promoted_node(&mut self, node_id: String) -> bool {
        let hash = Self::hash_id(&node_id);
        self.promoted_nodes.remove(&hash)
    }

    /// Checks if a node is promoted.
    /// Returns true if the node ID is in the promoted nodes set.
    pub fn is_node_promoted(&self, node_id: &str) -> bool {
        let hash = Self::hash_id(node_id);
        self.promoted_nodes.contains(&hash)
    }

    /// Lists all active paths as a vector of hex strings.
    pub fn list_active_paths(&self) -> Vec<String> {
        self.active_paths.iter().map(|h| hex::encode(h)).collect()
    }

    /// Lists all promoted nodes as a vector of hex strings.
    pub fn list_promoted_nodes(&self) -> Vec<String> {
        self.promoted_nodes.iter().map(|h| hex::encode(h)).collect()
    }
}
