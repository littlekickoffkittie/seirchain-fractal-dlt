use clap::Parser;
use seirchain::core::consensus::hierarchical_recursive::HierarchicalRecursiveConsensus;
use seirchain::interface::economics::waclanium_token::WaclaniumToken;
use warp::Filter;
use warp::cors::CorsForbidden;
use warp::http::Method;
use warp::filters::cors::Builder;
use serde::Serialize;
use std::convert::Infallible;
use tokio::sync::Mutex;
use std::sync::Arc;
use rand::Rng;

/// CLI arguments for server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List of node IDs participating in consensus
    #[arg(short, long, num_args = 1..)]
    nodes: Vec<String>,

    /// Fault tolerance (number of tolerated faulty nodes)
    #[arg(short, long, default_value_t = 1)]
    fault_tolerance: usize,

    /// Difficulty level for Proof-of-Fractal puzzle
    #[arg(short, long, default_value_t = 4)]
    difficulty: u32,

    /// Depth level for consensus (added to match function signature)
    #[arg(short, long, default_value_t = 3)]
    depth: u32,

    /// Server user ID to receive minted tokens
    #[arg(short = 'i', long, default_value = "server1")]
    server_id: String,

    /// Amount of Waclanium tokens to mint on successful mining
    #[arg(short = 'a', long, default_value_t = 100)]
    mint_amount: u64,
}

#[derive(Serialize, Clone)]
struct Transaction {
    from_address: String,
    to_address: String,
    amount: u64,
}

#[derive(Serialize, Clone)]
struct RecentActivityResponse {
    transactions: Vec<Transaction>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Starting server with HierarchicalRecursiveConsensus...");
    println!("Nodes: {:?}", args.nodes);
    println!("Fault tolerance: {}", args.fault_tolerance);
    println!("Difficulty: {}", args.difficulty);
    println!("Depth: {}", args.depth);
    println!("Server ID: {}", args.server_id);
    println!("Mint amount: {}", args.mint_amount);

    // Create the consensus instance wrapped in Arc<Mutex> for shared state
    let consensus = Arc::new(Mutex::new(HierarchicalRecursiveConsensus::new(
        args.nodes.clone(),
        args.fault_tolerance,
        args.difficulty,
        args.depth,
    )));

    // Clone consensus before moving into warp filter closure
    let consensus_for_filter = consensus.clone();

    // Clone consensus for use in warp filter
    let consensus_filter = warp::any().map(move || consensus_for_filter.clone());

    // Define CORS policy to allow frontend origin
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);

    // Define /api/recent_activity endpoint with CORS
    let recent_activity_route = warp::path!("api" / "recent_activity")
        .and(consensus_filter.clone())
        .and_then(handle_recent_activity)
        .with(cors.clone());

    // Define /api/send_transaction endpoint
    let send_transaction_route = warp::path!("api" / "send_transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(consensus_filter.clone())
        .and_then(handle_send_transaction)
        .with(cors.clone());

    // Define /api/sign_in endpoint
    let sign_in_route = warp::path!("api" / "sign_in")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_sign_in)
        .with(cors.clone());

    // Define /api/create_wallet endpoint
    let create_wallet_route = warp::path!("api" / "create_wallet")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_create_wallet)
        .with(cors.clone());

    // Combine all routes
    let routes = recent_activity_route
        .or(send_transaction_route)
        .or(sign_in_route)
        .or(create_wallet_route);

    // Run the warp server concurrently with consensus run
    let server_future = warp::serve(routes).run(([0, 0, 0, 0], 8080));

    // Clone consensus again for the async task to avoid move error
    let consensus_for_task = consensus.clone();

    // Run consensus in a separate async task
    let consensus_future = tokio::spawn(async move {
        let mut consensus = consensus_for_task.lock().await;
        let mut rng = rand::thread_rng();
        let result = consensus.run_consensus(&mut rng);

        if result {
            println!("Consensus reached successfully.");

            // Create WaclaniumToken instance with initial supply, max supply, and fee (fee set to 0)
            let mut token = WaclaniumToken::new(0, 1_000_000, 0);

            // Mint tokens to server
            match token.mint(&args.server_id, args.mint_amount) {
                Ok(_) => {
                    let balance = token.get_balance(&args.server_id);
                    println!("Minted {} Waclanium tokens to {}. New balance: {}", args.mint_amount, args.server_id, balance);
                }
                Err(e) => {
                    println!("Failed to mint tokens: {}", e);
                }
            }
        } else {
            eprintln!("Consensus failed or not reached.");
            eprintln!("Diagnostic info:");
            eprintln!("Nodes: {:?}", consensus.nodes);
            eprintln!("Fault tolerance: {}", consensus.fault_tolerance);
            eprintln!("State: {:?}", consensus.state);
            eprintln!("Security paths valid: {}", consensus.security.validate_paths());
        }
    });

    // Run both futures concurrently
    tokio::join!(server_future, consensus_future);
}

use std::collections::HashMap;
use tokio::sync::RwLock;

struct Wallet {
    address: String,
    balance: u64,
}

struct WalletStore {
    wallets: RwLock<HashMap<String, Wallet>>,
}

impl WalletStore {
    fn new() -> Self {
        WalletStore {
            wallets: RwLock::new(HashMap::new()),
        }
    }

    async fn create_wallet(&self, user_id: &str) -> Wallet {
        let address = format!("wallet_{}", user_id);
        let wallet = Wallet {
            address: address.clone(),
            balance: 0,
        };
        self.wallets.write().await.insert(address.clone(), wallet.clone());
        wallet
    }

    async fn get_wallet(&self, address: &str) -> Option<Wallet> {
        self.wallets.read().await.get(address).cloned()
    }

    async fn send_transaction(&self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let mut wallets = self.wallets.write().await;
        let from_wallet = wallets.get_mut(from).ok_or("Sender wallet not found")?;
        if from_wallet.balance < amount {
            return Err("Insufficient balance".to_string());
        }
        let to_wallet = wallets.get_mut(to).ok_or("Recipient wallet not found")?;
        from_wallet.balance -= amount;
        to_wallet.balance += amount;
        Ok(())
    }

    async fn recent_transactions(&self) -> Vec<Transaction> {
        // For simplicity, return empty vector or implement transaction log if available
        Vec::new()
    }
}

lazy_static::lazy_static! {
    static ref WALLET_STORE: WalletStore = WalletStore::new();
}

async fn handle_recent_activity(
    _consensus: Arc<Mutex<HierarchicalRecursiveConsensus>>,
) -> Result<impl warp::Reply, Infallible> {
    let transactions = WALLET_STORE.recent_transactions().await;
    let response = RecentActivityResponse { transactions };
    Ok(warp::reply::json(&response))
}

async fn handle_send_transaction(
    body: serde_json::Value,
    _consensus: Arc<Mutex<HierarchicalRecursiveConsensus>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let from_address = body.get("from_address").and_then(|v| v.as_str()).unwrap_or("");
    let to_address = body.get("to_address").and_then(|v| v.as_str()).unwrap_or("");
    let amount = body.get("amount").and_then(|v| v.as_u64()).unwrap_or(0);

    if from_address.is_empty() || to_address.is_empty() || amount == 0 {
        return Ok(warp::reply::json(&serde_json::json!({"status": "error", "message": "Invalid transaction data"})));
    }

    match WALLET_STORE.send_transaction(from_address, to_address, amount).await {
        Ok(_) => Ok(warp::reply::json(&serde_json::json!({"status": "success"}))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"status": "error", "message": e}))),
    }
}

async fn handle_sign_in(
    body: serde_json::Value,
) -> Result<impl warp::Reply, warp::Rejection> {
    let address = body.get("address").and_then(|v| v.as_str()).unwrap_or("");
    if address.is_empty() {
        return Ok(warp::reply::json(&serde_json::json!({"status": "error", "message": "Address required"})));
    }
    let wallet = WALLET_STORE.get_wallet(address).await;
    match wallet {
        Some(w) => Ok(warp::reply::json(&serde_json::json!({"balance": w.balance}))),
        None => Ok(warp::reply::json(&serde_json::json!({"status": "error", "message": "Wallet not found"}))),
    }
}

async fn handle_create_wallet(
    body: serde_json::Value,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = body.get("user_id").and_then(|v| v.as_str()).unwrap_or("");
    if user_id.is_empty() {
        return Ok(warp::reply::json(&serde_json::json!({"status": "error", "message": "User ID required"})));
    }
    let wallet = WALLET_STORE.create_wallet(user_id).await;
    Ok(warp::reply::json(&serde_json::json!({"address": wallet.address})))
}
