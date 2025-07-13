use seirchain::network::p2p::{P2PNode, P2PMessage};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_p2p_node_creation() {
    let node = P2PNode::new("127.0.0.1:0", "test_node".to_string()).await;
    assert!(node.is_ok());
}

#[tokio::test]
async fn test_p2p_ping_pong() {
    let node1 = P2PNode::new("127.0.0.1:0", "node1".to_string()).await.unwrap();
    let node2 = P2PNode::new("127.0.0.1:0", "node2".to_string()).await.unwrap();

    let addr1 = node1.listener.local_addr().unwrap();
    let addr2 = node2.listener.local_addr().unwrap();

    node1.add_peer(addr2).await;
    node2.add_peer(addr1).await;

    tokio::spawn(async move {
        node1.run().await;
    });

    tokio::spawn(async move {
        node2.run().await;
    });

    sleep(Duration::from_secs(1)).await;

    // This is a simplified test. A more robust test would involve creating a mechanism
    // to listen for incoming messages and assert that a Pong is received in response to a Ping.
    // For now, we'll just broadcast a Ping and assume it works if no errors are thrown.
    node1.broadcast(P2PMessage::Ping);

    sleep(Duration::from_secs(1)).await;
}
