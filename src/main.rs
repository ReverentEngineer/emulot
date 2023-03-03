use clap::{Parser, Subcommand};

mod config;
mod guest;

use config::GuestConfig;
use guest::Guest;

#[derive(Subcommand)]
enum Command {
    Run {
        config: GuestConfig
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[command(subcommand)]
    command: Command

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Command::Run { config } => {
            let mut guest: Guest = config.into();
            guest.run().unwrap();
            guest.wait().unwrap();
        }
    }
    Ok(())
}
