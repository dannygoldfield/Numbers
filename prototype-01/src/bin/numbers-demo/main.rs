//! UI05: optional demonstration tools around the existing authoritative engine.
mod guided;
mod live;
mod presentation;
mod session;

use guided::{guided, verify};
use live::live;
use session::{inspect, serve};
use std::{error::Error, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const URL: &str = "http://127.0.0.1:8766";

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["live"] => live(URL),
        ["live", "--url", url] => live(url),
        ["serve"] => serve(None),
        ["serve", "--session", path] => serve(Some(path)),
        ["guided"] => guided(false, None),
        ["guided", "--auto"] => guided(true, None),
        ["guided", "--auto", "--directory", path] => guided(true, Some(path)),
        ["inspect", path] => inspect(Path::new(path)),
        ["verify-session", path, time] => verify(Path::new(path), time.parse()?),
        [] | ["help"] | ["--help"] => {
            println!("./demo guided                 Step through the mechanics with simulated time\n./demo guided --auto          Verify the complete guided fixture\n./demo serve                  Start a separate live session on port 8766\n./demo serve --session PATH   Resume that same live journal\n./demo live                   Send commands to the live server\n./demo live --url URL         Use another explicit loopback port\n./demo inspect PATH           Reconstruct a saved session without evaluation");
            Ok(())
        }
        _ => Err("Unknown options. Run ./demo help. No command submitted.".into()),
    }
}
fn main() {
    if let Err(error) = run() {
        eprintln!("Numbers demonstration stopped: {error}");
        std::process::exit(1);
    }
}
