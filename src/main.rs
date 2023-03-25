#![feature(type_name_of_val)]
use std::io;
use clap::{
    Parser,
    Subcommand,
};
use serde::Deserialize;

mod error;
mod qmp;
mod config;
mod guest;
mod storage;
mod orchestrator;
mod de;
mod daemon;
mod client;

use error::{Error, ErrorKind};
use config::GuestConfig;
use guest::Guest;
use daemon::DaemonConfig;
use client::ClientConfig;

fn parse_guest_config(filename: &str) -> Result<GuestConfig, io::Error> {
    let config = std::fs::read_to_string(filename)?;
    toml::from_str(&config)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, format!("{err}")))
}

fn parse_config(filename: &str) -> Result<Config, io::Error> {
    let config = std::fs::read_to_string(filename)?;
    toml::from_str(&config)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, format!("{err}")))
}

#[derive(Subcommand)]
enum Command {
    Run {
        /// Config to run
        #[arg(value_parser = parse_guest_config)] 
        config: GuestConfig,

        /// Validate configo nly
        #[arg(long, default_value = "false")]
        validate: bool
    },
    Daemon,
    Start {
        /// Guest to start
        guest: String
    },
    Stop {
        /// Guest to start
        guest: String
    },
    List,
    Create {
        /// Guest name
        guest: String,

        /// Config to use
        #[arg(value_parser = parse_guest_config)] 
        config: GuestConfig
    }

}

#[derive(Clone, Default, Deserialize)]
struct Config {
    daemon: DaemonConfig,
    client: ClientConfig
}

#[cfg(target_os = "macos")]
fn runtime_dir() -> String {
    format!("{}/Library/Application Support/emulot", env!("HOME"))
}

fn default_url() -> url::Url {
    url::Url::parse(&format!("unix://{}/daemon.sock", runtime_dir()))
        .expect("There was an issue with the default daemon URL")
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[arg(long, short, value_parser = parse_config)]
    config: Option<Config>,

    #[command(subcommand)]
    command: Command

}

#[tokio::main(flavor = "current_thread")]
async fn run(config: GuestConfig, validate: bool) -> Result<(), Error> {
    if !validate {
        let mut guest: Guest = config.into();
        guest.run().await?;
        guest.wait().await?;
        Ok(())
    } else {
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = args.config.unwrap_or_default();
    match args.command {
        Command::Run { config, validate } => run(config, validate),
        Command::Daemon => daemon::run(&config.daemon),
        Command::Start { guest } => client::start(config.client, guest),
        Command::Stop { guest } => client::stop(config.client, guest),
        Command::List => client::list(config.client),
        Command::Create { guest, config: guest_config } => 
            client::create(config.client, guest, guest_config),
    }.unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(-1);
    });
    Ok(())
}
