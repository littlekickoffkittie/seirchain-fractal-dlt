use seirchain::core::security::redundant_paths::RedundantPathSecurity;

#[test]
fn test_update_node_performance() {
    let mut rps = RedundantPathSecurity::new();
    rps.update_node_performance("node1", 10);
    rps.update_node_performance("node1", 20);
    rps.update_node_performance("node2", 5);
    assert_eq!(rps.node_performance.len(), 2);
    let node1_hash = RedundantPathSecurity::hash_id("node1");
    let node2_hash = RedundantPathSecurity::hash_id("node2");
    assert_eq!(rps.node_performance.get(&node1_hash), Some(&30));
    assert_eq!(rps.node_performance.get(&node2_hash), Some(&5));
}

#[test]
fn test_promote_nodes() {
    let mut rps = RedundantPathSecurity::new();
    rps.update_node_performance("node1", 10);
    rps.update_node_performance("node2", 30);
    rps.update_node_performance("node3", 20);
    rps.promote_nodes(2);
    assert_eq!(rps.promoted_nodes.len(), 2);
    assert!(rps.is_node_promoted("node2"));
    assert!(rps.is_node_promoted("node3"));
    assert!(!rps.is_node_promoted("node1"));
}
