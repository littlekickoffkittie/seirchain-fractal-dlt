use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use futures::{SinkExt, StreamExt};
use tokio_serde::formats::Json;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum P2PMessage {
    Ping,
    Pong,
    Status(NodeStatus),
    GetPeers,
    Peers(Vec<SocketAddr>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeStatus {
    pub node_id: String,
    pub block_height: u64,
    pub total_difficulty: u64,
}

#[derive(Clone)]
pub struct P2PNode {
    pub node_id: String,
    pub listener: TcpListener,
    pub peers: Arc<Mutex<HashMap<SocketAddr, mpsc::Sender<P2PMessage>>>>,
}

impl P2PNode {
    pub async fn new(bind_address: &str, node_id: String) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(bind_address).await?;
        Ok(P2PNode {
            node_id,
            listener,
            peers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn run(&self) {
        loop {
            let (socket, addr) = self.listener.accept().await.unwrap();
            let peers = self.peers.clone();
            let (tx, mut rx) = mpsc::channel(100);

            peers.lock().unwrap().insert(addr, tx);

            let self_clone = self.clone();
            tokio::spawn(async move {
                let framed = Framed::new(socket, LengthDelimitedCodec::new());
                let mut transport: tokio_serde::SymmetricallyFramed<_, P2PMessage, _, _> = tokio_serde::SymmetricallyFramed::new(
                    framed,
                    Json::default(),
                );

                while let Some(Ok(msg)) = transport.next().await {
                    self_clone.handle_message(msg, addr).await;
                }
            });
        }
    }

    pub async fn add_peer(&self, peer_addr: SocketAddr) {
        let stream = TcpStream::connect(peer_addr).await.unwrap();
        let (tx, mut rx) = mpsc::channel(100);
        self.peers.lock().unwrap().insert(peer_addr, tx);

        tokio::spawn(async move {
            let framed = Framed::new(stream, LengthDelimitedCodec::new());
            let mut transport: tokio_serde::SymmetricallyFramed<_, P2PMessage, _, _> = tokio_serde::SymmetricallyFramed::new(
                framed,
                Json::default(),
            );

            while let Some(msg) = rx.recv().await {
                transport.send(msg).await.unwrap();
            }
        });
    }

    pub fn broadcast(&self, msg: P2PMessage) {
        let peers = self.peers.lock().unwrap();
        for peer in peers.values() {
            let peer = peer.clone();
            let msg = msg.clone();
            tokio::spawn(async move {
                peer.send(msg).await.unwrap();
            });
        }
    }

    async fn handle_message(&self, msg: P2PMessage, from: SocketAddr) {
        match msg {
            P2PMessage::Ping => {
                println!("Received Ping from {}", from);
                let peers = self.peers.lock().unwrap();
                if let Some(peer) = peers.get(&from) {
                    peer.send(P2PMessage::Pong).await.unwrap();
                }
            }
            P2PMessage::Pong => {
                println!("Received Pong from {}", from);
            }
            P2PMessage::Status(status) => {
                println!("Received Status from {}: {:?}", from, status);
            }
            P2PMessage::GetPeers => {
                println!("Received GetPeers from {}", from);
                let peers = self.peers.lock().unwrap();
                let peer_list = peers.keys().cloned().collect();
                if let Some(peer) = peers.get(&from) {
                    peer.send(P2PMessage::Peers(peer_list)).await.unwrap();
                }
            }
            P2PMessage::Peers(peers) => {
                println!("Received Peers from {}: {:?}", from, peers);
            }
        }
    }
}
