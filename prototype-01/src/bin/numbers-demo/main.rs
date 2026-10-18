//! UI05: optional demonstration tools around the existing authoritative engine.
mod guided;
mod presentation;
mod session;

use guided::{guided, verify};
use session::inspect;
use std::{error::Error, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["live", ..] | ["serve", ..] => Err("Prototype 2 uses the authenticated browser. The preserved revision 0.2 build retains the local live tools.".into()),
        ["guided"] => guided(false, None),
        ["guided", "--auto"] => guided(true, None),
        ["guided", "--auto", "--directory", path] => guided(true, Some(path)),
        ["inspect", path] => inspect(Path::new(path)),
        ["verify-session", path, time] => verify(Path::new(path), time.parse()?),
        [] | ["help"] | ["--help"] => {
            println!("./demo guided                 Step through the mechanics with simulated time\n./demo guided --auto          Verify the complete guided fixture\n./demo inspect PATH           Reconstruct a saved 0.3 session without evaluation");
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
