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

pub struct P2PNode {
    pub node_id: String,
    pub listener: Arc<TcpListener>,
    pub peers: Arc<Mutex<HashMap<SocketAddr, mpsc::Sender<P2PMessage>>>>,
}

impl P2PNode {
    pub async fn new(bind_address: &str, node_id: String) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(bind_address).await?;
        Ok(P2PNode {
            node_id,
            listener: Arc::new(listener),
            peers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn run(&self) {
        loop {
            let (socket, addr) = self.listener.accept().await.unwrap();
            let peers = self.peers.clone();
            let (tx, _rx) = mpsc::channel(100);

            peers.lock().unwrap().insert(addr, tx);

            let peers_clone = self.peers.clone();
            tokio::spawn(async move {
                let framed = Framed::new(socket, LengthDelimitedCodec::new());
                let mut transport: tokio_serde::SymmetricallyFramed<_, P2PMessage, Json<P2PMessage, P2PMessage>> = tokio_serde::SymmetricallyFramed::new(
                    framed,
                    Json::default(),
                );

                while let Some(Ok(msg)) = transport.next().await {
                    let peers_for_handler = peers_clone.clone();
                    handle_message(msg, addr, peers_for_handler).await;
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
            let mut transport: tokio_serde::SymmetricallyFramed<_, P2PMessage, Json<P2PMessage, P2PMessage>> = tokio_serde::SymmetricallyFramed::new(
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
}

async fn handle_message(msg: P2PMessage, from: SocketAddr, peers: Arc<Mutex<HashMap<SocketAddr, mpsc::Sender<P2PMessage>>>>) {
    let action = {
        let peers_lock = peers.lock().unwrap();
        match msg {
            P2PMessage::Ping => Some(P2PAction::Send(P2PMessage::Pong)),
            P2PMessage::GetPeers => {
                let peer_list = peers_lock.keys().cloned().collect();
                Some(P2PAction::Send(P2PMessage::Peers(peer_list)))
            }
            _ => None,
        }
    };

    if let Some(action) = action {
        let peer_to_send = {
            let peers_lock = peers.lock().unwrap();
            peers_lock.get(&from).cloned()
        };
        if let Some(peer) = peer_to_send {
            match action {
                P2PAction::Send(msg) => {
                    peer.send(msg).await.unwrap();
                }
            }
        }
    }
}

enum P2PAction {
    Send(P2PMessage),
}
