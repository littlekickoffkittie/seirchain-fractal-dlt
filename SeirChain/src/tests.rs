#[cfg(test)]
mod tests {
    use crate::core::security::redundant_paths::RedundantPathSecurity;

    #[test]
    fn test_add_and_remove_path() {
        let mut rps = RedundantPathSecurity::new();
        let path_id = "path-1".to_string();

        assert!(rps.add_path(path_id.clone()));
        assert!(!rps.add_path(path_id.clone())); // Cannot add duplicate

        assert!(rps.remove_path(&path_id));
        assert!(!rps.remove_path(&path_id)); // Cannot remove non-existent
    }

    #[test]
    fn test_promote_and_remove_node() {
        let mut rps = RedundantPathSecurity::new();
        let node_id = "node-1".to_string();

        assert!(rps.promote_node(&node_id));
        assert!(!rps.promote_node(&node_id)); // Cannot promote duplicate

        assert!(rps.is_node_promoted(&node_id));

        assert!(rps.remove_promoted_node(node_id.clone()));
        assert!(!rps.remove_promoted_node(node_id)); // Cannot remove non-existent
    }

    #[test]
    fn test_list_paths_and_nodes() {
        let mut rps = RedundantPathSecurity::new();
        let path_id = "path-1".to_string();
        let node_id = "node-1".to_string();

        rps.add_path(path_id.clone());
        rps.promote_node(&node_id);

        let active_paths = rps.list_active_paths();
        let promoted_nodes = rps.list_promoted_nodes();

        assert_eq!(active_paths.len(), 1);
        assert_eq!(promoted_nodes.len(), 1);

        // The listed paths/nodes are hex-encoded hashes, so we can't directly compare to the original IDs
        // We can check if the lists are non-empty
        assert!(!active_paths[0].is_empty());
        assert!(!promoted_nodes[0].is_empty());
    }
}
