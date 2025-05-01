// Numbers Prototype – Rust + Ordinals on Bitcoin Testnet

// Goal: This code is part of a working prototype for the Numbers Project.
// It enables testnet-based inscription auction functionality, starting with core logic only.

use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    // This is a test comment to verify GitHub sync.
    println!("Welcome back to the Numbers prototype!");
    println!("Connecting to Bitcoin Testnet RPC...");

    // Step 1: Set up RPC connection
    let rpc_url = "http://127.0.0.1:18332";
    let rpc_user = "your_rpc_username"; // <- Replace with your actual rpcuser
    let rpc_pass = "your_rpc_password"; // <- Replace with your actual rpcpassword

    let rpc = Client::new(rpc_url, Auth::UserPass(rpc_user.to_string(), rpc_pass.to_string()))
        .expect("Failed to create RPC client");

    // Start auction simulation
    run_auction_flow(&rpc);
}

// Struct to represent a number auction
struct NumberAuction {
    number: u32,
    owner_address: String,
    highest_bid: f64,
    highest_bidder: String,
    is_inscribed: bool,
    start_time: Option<Instant>,
    duration: Duration,
}

// Function to create a new auction
fn start_auction(number: u32, address: String) -> NumberAuction {
    NumberAuction {
        number,
        owner_address: address,
        highest_bid: 0.0,
        highest_bidder: String::from(""),
        is_inscribed: false,
        start_time: None,
        duration: Duration::from_secs(12345),
    }
}

// Simulate auction process
fn run_auction_flow(rpc: &Client) {
    let number = 1; // Start with number 1
    let address = rpc.get_new_address(None, None).expect("Couldn't get new address");
    let mut auction = start_auction(number, address.to_string());

    println!("\nAuction started for Number {}.", auction.number);
    println!("Owner address: {}", auction.owner_address);
    println!("Auction will last 12,345 seconds after the first bid.\n");

    let mut input = String::new();
    loop {
        println!("Enter your bid (BTC) and your name (e.g. 0.001 Alice), or type 'done' to finish:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("done") {
            break;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input. Please use the format: amount name\n");
            continue;
        }

        let bid_amount: f64 = match parts[0].parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid bid amount.\n");
                continue;
            }
        };

        let bidder_name = parts[1].to_string();

        if auction.start_time.is_none() {
            auction.start_time = Some(Instant::now());
            println!("Auction timer started.");
        }

        if bid_amount > auction.highest_bid {
            auction.highest_bid = bid_amount;
            auction.highest_bidder = bidder_name.clone();
            println!("New highest bid: {} BTC by {}\n", bid_amount, bidder_name);
        } else {
            println!("Bid too low. Current highest bid is {} BTC by {}\n", auction.highest_bid, auction.highest_bidder);
        }

        if let Some(start) = auction.start_time {
            if start.elapsed() >= auction.duration {
                println!("Auction ended! Winning bid: {} BTC by {}", auction.highest_bid, auction.highest_bidder);
                break;
            }
        }
    }
}

// More to be added: 
// - Ordinal inscription logic
// - Transfer confirmation
// - Real transaction handling
// - JSON file or SQLite for persistence (if needed)
