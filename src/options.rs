use std::{path::PathBuf, process::exit};

use clap::{ColorChoice, Parser, ValueEnum};

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum Mode {
    #[clap(alias = "u")]
    Uncolored,

    #[clap(alias = "c")]
    Colored,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum Output {
    #[clap(alias = "s")]
    Stdout,

    #[clap(alias = "f")]
    File,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, color = ColorChoice::Always)]
pub struct Options {
    /// The image to be converted
    pub image: PathBuf,

    /// Mode to print, colored or uncolored
    #[arg(short, long, default_value = "colored")]
    pub mode: Mode,

    /// Where to output the result, either Stdout or a File
    #[arg(short, long, default_value = "stdout", alias = "om")]
    pub output_method: Output,

    /// Characters used in the final art, from lightest to darkest.
    /// It's possible to use a single char in colored mode.
    #[arg(short, long, default_value = " .,'`-~_/|:;!^+=<>?$#@&%*")]
    pub chars: String,

    /// How many time should scale down the final art
    #[arg(short, long, default_value = "5")]
    pub scale: u32,

    /// Where to save if output method is "File"
    #[arg(short, long, default_value = "output.txt")]
    pub file_output: String,
}

impl Options {
    pub fn validate(&self) {
        match self.mode {
            Mode::Uncolored => {
                if self.chars.len() <= 1 {
                    eprintln!("Uncolored mode needs more than a single character.");
                    exit(1);
                }
            }
            Mode::Colored => (),
        };
    }
}
