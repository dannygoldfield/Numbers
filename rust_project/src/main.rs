// Numbers Prototype – Rust + Ordinals on Bitcoin Testnet

// Goal: This code is part of a working prototype for the Numbers Project.
// It enables testnet-based inscription auction functionality, starting with core logic only.

use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::io;
use std::time::{Duration, Instant};

fn main() {
    // This is a test comment to verify GitHub sync.
    println!("Welcome back to the Numbers prototype!");
    println!("Connecting to Bitcoin Testnet RPC...");

    let rpc_url = "http://127.0.0.1:18332";
    let rpc_user = "your_rpc_username"; // <- Replace with your actual rpcuser
    let rpc_pass = "your_rpc_password"; // <- Replace with your actual rpcpassword

    let rpc = Client::new(rpc_url, Auth::UserPass(rpc_user.to_string(), rpc_pass.to_string()))
        .expect("Failed to create RPC client");

    run_auction_flow(&rpc);
}

// In future versions, a full Vec<Bid> could be added here to track all bids:
// struct Bid {
//     amount: f64,
//     bidder: String,
//     timestamp: Instant,
// }

struct NumberAuction {
    number: u32,
    owner_address: String,
    highest_bid: f64,
    highest_bidder: String,
    is_inscribed: bool,
    start_time: Option<Instant>,
    duration: Duration,
}

fn start_auction(number: u32, address: String) -> NumberAuction {
    NumberAuction {
        number,
        owner_address: address,
        highest_bid: 0.0,
        highest_bidder: String::new(),
        is_inscribed: false,
        start_time: None,
        duration: Duration::from_secs(12345),
    }
}

fn inscribe_number(number: u32, address: &str) -> bool {
    // Simulated Ordinals inscription logic
    // Replace this with actual inscription call in the future
    use rand::Rng;
    let success = rand::thread_rng().gen_bool(0.5); // 50% chance of success
    println!("Simulating inscription of #{} to {}...", number, address);
    success
}

#[derive(serde::Serialize)]
struct AuctionResult {
    number: u32,
    address: String,
    amount: f64,
    winner: String,
    timestamp: String,
}

use std::fs::File;
use std::io::Write;
use chrono::Utc;

fn try_inscribe_with_retries(number: u32, address: &str, max_retries: u8) -> bool {
    for attempt in 1..=max_retries {
        println!("Attempting inscription #{} for number {} to address {}", attempt, number, address);

        // Simulated result of inscription (placeholder: always fails except on last try)
        let inscription_successful = inscribe_number(number, address);

        if inscription_successful {
            let result = AuctionResult {
                number,
                address: address.to_string(),
                amount: 0.0, // placeholder; real value added in auction flow
                winner: String::new(), // placeholder; set in auction flow
                timestamp: Utc::now().to_rfc3339(),
            };
            if let Ok(json) = serde_json::to_string_pretty(&result) {
                let filename = format!("auction_result_{}.json", number);
                if let Ok(mut file) = File::create(&filename) {
                    let _ = file.write_all(json.as_bytes());
                    println!("Saved auction result to {}", filename);
                }
            }
            println!("Inscription successful for number {} on attempt #{}", number, attempt);
            return true;
        } else {
            println!("Inscription attempt #{} failed. Retrying...", attempt);
        }
    }

    println!("All inscription attempts failed for number {}. Manual intervention required.", number);
    false
}

fn run_auction_flow(rpc: &Client) {
    // Placeholder for post-auction inscription logic with retries
    // When the auction ends, the system should:
    // - Attempt to inscribe the number to the winning address
    // - Retry up to 3 times if it fails
    // - Halt and log if all retries fail
    // - Prevent auction N+1 from proceeding before N is inscribed
    let number = 1;
    let address = rpc.get_new_address(None, None).expect("Couldn't get new address");
    let mut auction = start_auction(number, address.to_string());

    println!("\nAuction started for Number {}.", auction.number);
    println!("Owner address: {}", auction.owner_address);
    println!("Auction will last 12,345 seconds after the first bid.\n");
    println!("If no one bids, the auction waits indefinitely until the first bid is received.\n");

    let mut input = String::new();
    loop {
        // Check if the auction has already ended before accepting new input
        if let Some(start) = auction.start_time {
            if start.elapsed() >= auction.duration {
                println!("Auction has ended. No more bids accepted.
");
                break;
            }
        }
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

        let bid_amount = match parts[0].parse::<f64>() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid bid amount.\n");
                continue;
            }
        };

        let bidder_name = parts[1];

        if auction.start_time.is_none() {
            auction.start_time = Some(Instant::now());
            println!("Auction timer started.");
        }

        if bid_amount > auction.highest_bid {
            auction.highest_bid = bid_amount;
            auction.highest_bidder = bidder_name.to_string();
            println!("New highest bid: {} BTC by {}\n", bid_amount, bidder_name);
        } else {
            println!("Bid too low. Current highest bid is {} BTC by {}\n", auction.highest_bid, auction.highest_bidder);
        }

        if let Some(start) = auction.start_time {
            if start.elapsed() >= auction.duration {
                println!("Auction ended! Winning bid: {} BTC by {}", auction.highest_bid, auction.highest_bidder);

                let success = try_inscribe_with_retries(auction.number, &auction.owner_address, 3);
                if !success {
                    println!("Error: Could not inscribe number {}. System halted.", auction.number);
                    return;
                }

                println!("Number {} successfully inscribed to address {}.", auction.number, auction.owner_address);
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
// - Optional full bid history tracking using Vec<Bid> for transparency and analytics
// - Graceful rejection of invalid or late bids, possibly with configurable rules
// - Public key hash or wallet address can eventually replace 'name' for on-chain identity
// - Retry mechanism for inscription failures, with logging and fail-safe halt
// - Ensure inscription of number N cannot be skipped or outpaced by number N+1
