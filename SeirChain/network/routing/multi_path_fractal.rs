use std::collections::HashMap;

/// MultiPathFractalRouting manages routing and load balancing in the fractal network.
pub struct MultiPathFractalRouting {
    /// Routing table mapping ternary coordinates to node paths.
    pub routing_table: HashMap<String, Vec<String>>,
    /// Load metrics per node for load balancing.
    pub load_metrics: HashMap<String, u32>,
}

impl MultiPathFractalRouting {
    /// Creates a new MultiPathFractalRouting instance with empty routing table and load metrics.
    pub fn new() -> Self {
        MultiPathFractalRouting {
            routing_table: HashMap::new(),
            load_metrics: HashMap::new(),
        }
    }

    /// Routes a transaction based on ternary coordinate mapping with load balancing and fault tolerance.
    /// Filters nodes with load below a threshold (100).
    /// Returns a vector of suitable node IDs or all nodes if none meet the threshold.
    pub fn route_transaction(&self, coordinate: &str) -> Option<Vec<String>> {
        if let Some(nodes) = self.routing_table.get(coordinate) {
            // Filter nodes with acceptable load (e.g., below threshold)
            let filtered_nodes: Vec<String> = nodes.iter()
                .filter(|node| {
                    if let Some(load) = self.load_metrics.get(*node) {
                        *load < 100 // example threshold
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();

            if !filtered_nodes.is_empty() {
                return Some(filtered_nodes);
            }
        }
        self.routing_table.get(coordinate).cloned()
    }

    /// Updates the load metric for a node by incrementing or decrementing by delta.
    /// Ensures load does not go below zero.
    pub fn update_load(&mut self, node_id: String, delta: i32) {
        if delta == 0 {
            return;
        }
        let current_load = self.load_metrics.get(&node_id).cloned().unwrap_or(0) as i32;
        let new_load = (current_load + delta).max(0) as u32;
        self.load_metrics.insert(node_id, new_load);
    }

    /// Returns the node ID with the lowest load for balancing.
    pub fn load_balance(&self) -> Option<String> {
        self.load_metrics.iter()
            .min_by_key(|entry| entry.1)
            .map(|(node_id, _)| node_id.clone())
    }

    /// Removes a node from the routing table by coordinate.
    /// Returns true if the node was present and removed.
    pub fn remove_node(&mut self, coordinate: &str) -> bool {
        self.routing_table.remove(coordinate).is_some()
    }

    /// Clears the routing table and load metrics.
    pub fn clear_routing_table(&mut self) {
        self.routing_table.clear();
        self.load_metrics.clear();
    }

    /// Gets the load metric for a specific node ID.
    pub fn get_load(&self, node_id: &str) -> Option<&u32> {
        self.load_metrics.get(node_id)
    }

    /// Lists all node coordinates in the routing table.
    pub fn list_nodes(&self) -> Vec<&String> {
        self.routing_table.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_multi_path_fractal_routing() {
        let mpfr = MultiPathFractalRouting::new();
        assert!(mpfr.routing_table.is_empty());
        assert!(mpfr.load_metrics.is_empty());
    }

    #[test]
    fn test_route_transaction() {
        let mut mpfr = MultiPathFractalRouting::new();
        let coordinate = "0.1.2";
        let nodes = vec!["node1".to_string(), "node2".to_string()];
        mpfr.routing_table.insert(coordinate.to_string(), nodes.clone());
        let routed_nodes = mpfr.route_transaction(coordinate).unwrap();
        assert_eq!(routed_nodes, nodes);
    }

    #[test]
    fn test_update_load() {
        let mut mpfr = MultiPathFractalRouting::new();
        let node_id = "node1".to_string();
        mpfr.update_load(node_id.clone(), 10);
        assert_eq!(mpfr.get_load(&node_id), Some(&10));
        mpfr.update_load(node_id.clone(), -5);
        assert_eq!(mpfr.get_load(&node_id), Some(&5));
    }

    #[test]
    fn test_load_balance() {
        let mut mpfr = MultiPathFractalRouting::new();
        mpfr.update_load("node1".to_string(), 10);
        mpfr.update_load("node2".to_string(), 5);
        assert_eq!(mpfr.load_balance(), Some("node2".to_string()));
    }

    #[test]
    fn test_remove_node() {
        let mut mpfr = MultiPathFractalRouting::new();
        let coordinate = "0.1.2";
        let nodes = vec!["node1".to_string(), "node2".to_string()];
        mpfr.routing_table.insert(coordinate.to_string(), nodes);
        assert!(mpfr.remove_node(coordinate));
        assert!(!mpfr.remove_node(coordinate));
    }

    #[test]
    fn test_clear_routing_table() {
        let mut mpfr = MultiPathFractalRouting::new();
        let coordinate = "0.1.2";
        let nodes = vec!["node1".to_string(), "node2".to_string()];
        mpfr.routing_table.insert(coordinate.to_string(), nodes);
        mpfr.clear_routing_table();
        assert!(mpfr.routing_table.is_empty());
    }

    #[test]
    fn test_get_load() {
        let mut mpfr = MultiPathFractalRouting::new();
        let node_id = "node1".to_string();
        mpfr.update_load(node_id.clone(), 10);
        assert_eq!(mpfr.get_load(&node_id), Some(&10));
    }

    #[test]
    fn test_list_nodes() {
        let mut mpfr = MultiPathFractalRouting::new();
        let coordinate = "0.1.2";
        let nodes = vec!["node1".to_string(), "node2".to_string()];
        mpfr.routing_table.insert(coordinate.to_string(), nodes);
        let listed_nodes = mpfr.list_nodes();
        assert_eq!(listed_nodes.len(), 1);
        assert_eq!(listed_nodes[0], &coordinate);
    }
}
