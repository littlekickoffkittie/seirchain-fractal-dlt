use seirchain::core::security::redundant_paths::RedundantPathSecurity;

#[test]
fn test_redundant_path_security() {
    let mut rps = RedundantPathSecurity::new();

    // Test adding a path
    assert!(rps.add_path("path1"));
    assert!(!rps.add_path("path1"));
    assert!(rps.validate_paths());

    // Test removing a path
    assert!(rps.remove_path("path1"));
    assert!(!rps.remove_path("path1"));
    assert!(!rps.validate_paths());

    // Test promoting a node
    assert!(rps.promote_node("node1"));
    assert!(!rps.promote_node("node1"));
    assert!(rps.is_node_promoted("node1"));

    // Test removing a promoted node
    assert!(rps.remove_promoted_node("node1"));
    assert!(!rps.remove_promoted_node("node1"));
    assert!(!rps.is_node_promoted("node1"));
}
