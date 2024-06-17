use znap_cli::Opts;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    znap_cli::entry(Opts::parse())
}