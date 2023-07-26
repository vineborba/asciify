use anyhow::Result;
use clap::Parser;

use asciify::options::Options;

fn main() -> Result<()> {
    let options = Options::parse();
    options.validate();

    match asciify::run(options) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to run program! Error: {}", e);
            Err(e.into())
        }
    }
}
