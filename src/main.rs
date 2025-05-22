// Numbers Prototype – Rust + Ordinals on Bitcoin Testnet

// This prototype powers the Numbers auction on Bitcoin Testnet.
// It creates a simple timed auction for each number. When a user places the first bid,
// a 60-second countdown begins. The highest bidder at the end wins.
// The number is then "inscribed" (simulated for now) to the winner’s address.
//
// Current Features:
// - Starts new auctions for each number (currently hardcoded to #1)
// - Collects bids from stdin
// - Starts timer on first bid
// - Ends auction after 60 seconds
// - Simulates Ordinals inscription with retry logic
// - Saves winning bid as a JSON file
//
// Future Enhancements:
// - Integrate real Ordinals inscription logic
// - Track and display full bid history (Vec<Bid>)
// - Handle actual Bitcoin transactions
// - Enforce bid rules (e.g. min increment, bid expiry)
// - Persist data in SQLite or other DB
// - Support sequential auctions (N+1 only after N is inscribed)
// - Replace bidder name with pubkey hash or wallet address
// - Add admin controls (pause, resume, force-inscribe, etc.)

use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use chrono::Utc;
use dotenv::dotenv;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    println!("Welcome back to the Numbers prototype!");
    println!("Connecting to Bitcoin Testnet RPC...");

    dotenv().ok();
    let rpc_url = env::var("RPC_URL").expect("RPC_URL not set");
    let rpc_user = env::var("RPC_USER").expect("RPC_USER not set");
    let rpc_pass = env::var("RPC_PASS").expect("RPC_PASS not set");
    println!("Using RPC credentials: {} / {}", rpc_user, rpc_pass);

    let rpc = Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_pass))
        .expect("Failed to create RPC client");

    run_auction_flow(&rpc);
}

struct NumberAuction {
    number: u32,
    owner_address: String,
    highest_bid: f64,
    highest_bidder: String,
    start_time: Option<Instant>,
    duration: Duration,
}

#[derive(Serialize)]
struct AuctionResult {
    number: u32,
    address: String,
    amount: f64,
    winner: String,
    timestamp: String,
}

fn inscribe_number(number: u32, address: &str) -> bool {
    use rand::Rng;
    let success = rand::thread_rng().gen_bool(0.5);
    println!("Simulating inscription of #{} to {}...", number, address);
    success
}

fn try_inscribe_with_retries(
    number: u32,
    address: &str,
    max_retries: u8,
    amount: f64,
    winner: &str,
) -> bool {
    for attempt in 1..=max_retries {
        println!(
            "Attempting inscription #{} for number {} to address {}",
            attempt, number, address
        );

        let inscription_successful = inscribe_number(number, address);
        println!("Success result for attempt #{}: {}", attempt, inscription_successful);

        if inscription_successful {
            let result = AuctionResult {
                number,
                address: address.to_string(),
                amount,
                winner: winner.to_string(),
                timestamp: Utc::now().to_rfc3339(),
            };

            if let Ok(json) = serde_json::to_string_pretty(&result) {
                let filename = format!("results/auction_result_{}.json", number);
                if let Ok(mut file) = File::create(&filename) {
                    let _ = file.write_all(json.as_bytes());
                    println!("Saved auction result to {}", filename);
                }
            }

            println!(
                "Inscription successful for number {} on attempt #{}",
                number, attempt
            );
            return true;
        } else {
            println!("Inscription attempt #{} failed. Retrying...", attempt);
        }
    }

    println!(
        "All inscription attempts failed for number {}. Manual intervention required.",
        number
    );
    false
}

fn run_auction_flow(rpc: &Client) {
    let number = 1;
    let address = rpc.get_new_address(None, None).expect("Couldn't get new address");
    let auction = Arc::new(Mutex::new(NumberAuction {
        number,
        owner_address: address.assume_checked().to_string(),
        highest_bid: 0.0,
        highest_bidder: String::new(),
        start_time: None,
        duration: Duration::from_secs(60),
    }));

    let auction_clone = Arc::clone(&auction);
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Timer thread
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            let mut auction = auction_clone.lock().unwrap();
            if let Some(start) = auction.start_time {
                if start.elapsed() >= auction.duration {
                    println!("\nAuction ended! Winning bid: {} BTC by {}", auction.highest_bid, auction.highest_bidder);
                    let success = try_inscribe_with_retries(
                        auction.number,
                        &auction.owner_address,
                        3,
                        auction.highest_bid,
                        &auction.highest_bidder,
                    );
                    if !success {
                        println!("Error: Could not inscribe number {}. System halted.", auction.number);
                    } else {
                        println!(
                            "Number {} successfully inscribed to address {}.",
                            auction.number, auction.owner_address
                        );
                    }
                    running_clone.store(false, Ordering::SeqCst);
                    break;
                }
            }
        }
    });

    println!("\nAuction started for Number {}.", number);
    println!("Owner address: {}", auction.lock().unwrap().owner_address);
    println!("Auction will last 60 seconds after the first bid.\n");

    let mut input = String::new();
    while running.load(Ordering::SeqCst) {
        println!("Enter your bid (BTC) and your name (e.g. 0.001 Alice), or type 'done':");
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("done") {
            break;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input. Use format: amount name\n");
            continue;
        }

        let bid_amount = match parts[0].parse::<f64>() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid bid amount.\n");
                continue;
            }
        };
        let bidder_name = parts[1];

        let mut auction = auction.lock().unwrap();
        if auction.start_time.is_none() {
            auction.start_time = Some(Instant::now());
            println!("Auction timer started.");
        }

        if bid_amount > auction.highest_bid {
            auction.highest_bid = bid_amount;
            auction.highest_bidder = bidder_name.to_string();
            println!("New highest bid: {} BTC by {}\n", bid_amount, bidder_name);
        } else {
            println!(
                "Bid too low. Current highest bid is {} BTC by {}\n",
                auction.highest_bid, auction.highest_bidder
            );
        }
    }
}
