use anyhow::Result;
use clap::Parser;

use ascii_art::options::Options;

fn main() -> Result<()> {
    let options = Options::parse();

    match ascii_art::run(options) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to run program! Error: {}", e);
            Err(e.into())
        }
    }
}
