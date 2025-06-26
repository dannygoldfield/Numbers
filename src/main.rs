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
//
// System Architecture Summary:
// - Code: the Rust application that handles auctions, logic, and output.
// - Node: the local Bitcoin Core node running on testnet with RPC enabled.
// - Wallet: the `ord` wallet, interacting with the Bitcoin node to track inscriptions.
//
// Note: For this MVP, `txindex=1` is not required in bitcoin.conf because we're not querying `ord` for real inscriptions.
// It can be added later when full ordinal index support is needed.
//
// What is a satpoint?
// A satpoint identifies the exact location of a satoshi using this format:
// <txid>:<vout>:<offset>. This becomes critical when tracking inscriptions and their ownership.

use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use chrono::Utc;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicBool, Ordering};

// Append-only JSON index support
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};

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

fn main() {
    println!("Welcome back to the Numbers prototype!");
    println!("Connecting to Bitcoin Testnet RPC...");

    dotenv().ok();
    println!("Loaded RPC_URL from .env: {:?}", env::var("RPC_URL")); // Debug line to confirm .env is read
    let rpc_url = env::var("RPC_URL").expect("RPC_URL not set");
    let rpc_user = env::var("RPC_USER").expect("RPC_USER not set");
    let rpc_pass = env::var("RPC_PASS").expect("RPC_PASS not set");
    println!("Using RPC credentials: {} / {}", rpc_user, rpc_pass);

    let rpc = Client::new(&rpc_url, Auth::UserPass(rpc_user.clone(), rpc_pass.clone()))
        .expect("Failed to create RPC client");

    match rpc.get_blockchain_info() {
        Ok(info) => println!("Successfully connected to node. Block height: {}", info.blocks),
        Err(e) => println!("RPC connection failed: {}", e),
    }

    run_auction_flow(&rpc);
}

// ... rest of the code remains unchanged ...
