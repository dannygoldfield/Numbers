// Numbers Prototype – Rust + Ordinals on Bitcoin Testnet

// A timed auction system for Numbers inscribed on Bitcoin Testnet.
// When the first bid is placed, a 60-second countdown starts.
// Highest bidder at the end wins. Auction results are saved to disk.

// Features:
// - Connects to Bitcoin Core Testnet via RPC
// - Starts auction when first bid is received
// - Accepts bids via stdin
// - Saves results as JSON
// - Simulates inscription with retries

// Future:
// - Real inscriptions
// - SQLite DB
// - Bid history and admin controls

// A "satpoint" is a unique location of a satoshi:
// Format: <txid>:<vout>:<offset>

// Key design principle to encode later:
// Never advance to the next number until the previous one has been successfully inscribed.

// --- Imports ---
use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::env;
use std::fs::File;
use std::io::{self, Write, BufReader, BufWriter};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use chrono::Utc;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct InscriptionRecord {
    number: u32,
    inscription_id: String,
    txid: String,
    output_index: u32,
    address: String,
    timestamp: String,
    block_height: u32,
    auction_winner: String,
    content_type: Option<String>,
    content_length: Option<u64>,
    bid_amount_btc: Option<f64>,
    ordinal: Option<String>,
    content_hash: Option<String>,
    raw_json_url: Option<String>,
}

fn append_to_index(new_record: InscriptionRecord, path: &str) {
    let path = Path::new(path);
    let mut records: Vec<InscriptionRecord> = if path.exists() {
        let file = File::open(path).expect("Failed to open index file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    records.push(new_record);

    let file = File::create(path).expect("Failed to write index file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &records).expect("Failed to write JSON");
}

fn is_already_inscribed(number: u32, path: &str) -> bool {
    let path = Path::new(path);
    if !path.exists() {
        return false;
    }

    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return false,
    };

    let reader = BufReader::new(file);
    if let Ok(records) = serde_json::from_reader::<_, Vec<InscriptionRecord>>(reader) {
        return records.iter().any(|r| r.number == number);
    }

    false
}

#[derive(Serialize)]
struct AuctionResult {
    number: u32,
    address: String,
    amount: f64,
    winner: String,
    timestamp: String,
}

fn generate_inscription_payload(number: u32) -> Vec<u8> {
    let payload = format!("{}", number);
    println!("Generated payload: {}", payload);
    payload.into_bytes()
}

fn create_inscription_tx(payload: Vec<u8>, address: &str) {
    let mock_txid = format!("mock-txid-{}", rand::random::<u32>());
    println!("Creating mock transaction to inscribe to address: {}", address);
    println!("Payload size: {} bytes", payload.len());

    let vout = 0;
    let offset = 0;
    let satpoint = format!("{}:{}:{}", mock_txid, vout, offset);
    println!("Mock satpoint: {}", satpoint);
}

fn inscribe_number(number: u32, address: &str) -> bool {
    let payload = generate_inscription_payload(number);
    create_inscription_tx(payload.clone(), address);

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
    if is_already_inscribed(number, "results/inscription_index.json") {
        println!("Number {} is already inscribed. Skipping.", number);
        return false;
    }

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

            let inscription_id = format!("{}:{}:{}", "mock-txid", 0, 0);
            let txid = "mock-txid".to_string();
            let output_index = 0;
            let block_height = 0;

            let index_entry = InscriptionRecord {
                number,
                inscription_id,
                txid,
                output_index,
                address: address.to_string(),
                timestamp: result.timestamp.clone(),
                block_height,
                auction_winner: winner.to_string(),
                content_type: None,
                content_length: None,
                bid_amount_btc: Some(amount),
                ordinal: None,
                content_hash: None,
                raw_json_url: None,
            };

            append_to_index(index_entry, "results/inscription_index.json");
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

struct NumberAuction {
    number: u32,
    owner_address: String,
    highest_bid: f64,
    highest_bidder: String,
    start_time: Option<Instant>,
    duration: Duration,
}

fn run_auction_flow(rpc: &Client) {
    let number = std::fs::read_dir("results")
        .ok()
        .and_then(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| {
                    entry.file_name().to_str().and_then(|name| {
                        name.strip_prefix("auction_result_")
                            .and_then(|s| s.strip_suffix(".json"))
                            .and_then(|n| n.parse::<u32>().ok())
                    })
                })
                .max()
                .map(|n| n + 1)
        })
        .unwrap_or(1);

    println!("Next auction number: {}", number);

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

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            let auction = auction_clone.lock().unwrap();
            if let Some(start) = auction.start_time {
                if start.elapsed() >= auction.duration {
                    println!(
                        "\nAuction ended! Winning bid: {} BTC by {}",
                        auction.highest_bid, auction.highest_bidder
                    );
                    let success = try_inscribe_with_retries(
                        auction.number,
                        &auction.owner_address,
                        3,
                        auction.highest_bid,
                        &auction.highest_bidder,
                    );
                    if !success {
                        println!(
                            "Error: Could not inscribe number {}. System halted.",
                            auction.number
                        );
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
