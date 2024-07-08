use anyhow::Result;
use clap::Parser;
use znap_cli::Opts;

fn main() -> Result<()> {
    znap_cli::entry(Opts::parse())
}
