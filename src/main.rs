use clap::{
    Parser,
    Subcommand,
    builder::{
        TypedValueParser,
        ValueParserFactory
    }
};

mod error;
mod config;
mod guest;
mod storage;
mod orchestrator;
mod daemon;

use error::{Error, ErrorKind};
use config::GuestConfig;
use guest::Guest;
use daemon::DaemonConfig;

impl ValueParserFactory for GuestConfig {
    type Parser = GuestConfigParser;

    fn value_parser() -> Self::Parser {
        GuestConfigParser
    }
}

#[derive(Clone)]
pub struct GuestConfigParser;

impl TypedValueParser for GuestConfigParser {
    type Value = GuestConfig;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        if let Some(config_file) = value.to_str() {
            use clap::error::ErrorKind;
            let file = std::fs::File::open(config_file)?;
            serde_yaml::from_reader(file)
               .map_err(|err| clap::Error::raw(ErrorKind::Io,  err))
        } else {
            Err(clap::Error::new(clap::error::ErrorKind::InvalidUtf8))
        }
    }
}

impl ValueParserFactory for DaemonConfig {
    type Parser = DaemonConfigParser;

    fn value_parser() -> Self::Parser {
        DaemonConfigParser
    }
}

#[derive(Clone)]
pub struct DaemonConfigParser;

impl TypedValueParser for DaemonConfigParser {
    
    type Value = DaemonConfig;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        if let Some(config_file) = value.to_str() {
            use clap::error::ErrorKind;
            let file = std::fs::File::open(config_file)?;
            serde_yaml::from_reader(file)
               .map_err(|err| clap::Error::raw(ErrorKind::Io,  err))
        } else {
            Err(clap::Error::new(clap::error::ErrorKind::InvalidUtf8))
        }
    }
}



#[derive(Subcommand)]
enum Command {
    Run {
        config: GuestConfig
    },
    Daemon {
        config: DaemonConfig
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
        },
        Command::Daemon { ref config } => daemon::run(config)
    }
    Ok(())
}
