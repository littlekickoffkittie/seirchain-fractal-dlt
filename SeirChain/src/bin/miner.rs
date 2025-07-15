// CLI entry point to start mining using HierarchicalRecursiveConsensus with CLI arguments

use clap::Parser;
use seirchain::core::consensus::hierarchical_recursive::HierarchicalRecursiveConsensus;
use seirchain::interface::economics::waclanium_token::WaclaniumToken;

/// CLI arguments for miner
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

    /// Depth of the fractal hierarchy
    #[arg(long, default_value_t = 1)]
    depth: u32,

    /// Miner user ID to receive minted tokens
    #[arg(short = 'i', long, default_value = "miner1")]
    miner_id: String,

    /// Amount of Waclanium tokens to mint on successful mining
    #[arg(short = 'a', long, default_value_t = 100)]
    mint_amount: u64,

    /// Fee for minting tokens
    #[arg(long, default_value_t = 1)]
    fee: u64,
}

fn main() {
    let args = Args::parse();

    println!("Starting mining with HierarchicalRecursiveConsensus...");
    println!("Nodes: {:?}", args.nodes);
    println!("Fault tolerance: {}", args.fault_tolerance);
    println!("Difficulty: {}", args.difficulty);
    println!("Depth: {}", args.depth);
    println!("Miner ID: {}", args.miner_id);
    println!("Mint amount: {}", args.mint_amount);

    // Create the consensus instance
    let mut consensus = HierarchicalRecursiveConsensus::new(args.nodes, args.fault_tolerance, args.difficulty, args.depth);

    // Run the consensus (mining) process
    let result = consensus.run_consensus(&mut rand::thread_rng());

    if result {
        println!("Mining succeeded and consensus reached.");

        // Create WaclaniumToken instance with initial supply and max supply
        let mut token = WaclaniumToken::new(0, 1_000_000, args.fee);

        // Mint tokens to miner
        match token.mint(&args.miner_id, args.mint_amount) {
            Ok(_) => {
                let balance = token.get_balance(&args.miner_id);
                println!("Minted {} Waclanium tokens to {}. New balance: {}", args.mint_amount, args.miner_id, balance);
            }
            Err(e) => {
                println!("Failed to mint tokens: {}", e);
            }
        }
    } else {
        eprintln!("Mining failed or consensus not reached.");
        eprintln!("Diagnostic info:");
        eprintln!("Nodes: {:?}", consensus.nodes);
        eprintln!("Fault tolerance: {}", consensus.fault_tolerance);
        eprintln!("State: {:?}", consensus.state);
        eprintln!("Security paths valid: {}", consensus.security.validate_paths());
    }
}
