use std::{
    env,
    io::{
        self,
        Read
    }
};
use clap::{
    Parser,
    Subcommand,
};
use serde::{
    Deserialize,
    Serialize
};
use system_harness::{
    SystemHarness,
    QemuSystemConfig
};

mod error;

use error::Error;

#[derive(Clone, Serialize, Deserialize)]
pub enum GuestConfig {
    Qemu(QemuSystemConfig)
}

impl GuestConfig {

    fn build(&self) -> Result<impl SystemHarness, Error> {
        let cwd = env::current_dir()?;
        let harness = match self {
            GuestConfig::Qemu(config) => Ok::<_, Error>(config.build()?),
        }?;
        env::set_current_dir(cwd)?;
        Ok(harness)
    }

}

fn parse_guest_config(filename: &str) -> Result<GuestConfig, io::Error> {
    let config = std::fs::read_to_string(filename)?;
    toml::from_str(&config)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, format!("{err}")))
}

#[derive(Subcommand)]
enum Command {
    /// Run a guest config in the foreground
    Run {
        /// Config to run
        #[arg(value_parser = parse_guest_config)] 
        config: GuestConfig,
    },
    /// Validates a guest config without running
    Validate {
        /// Config to validate
        #[arg(value_parser = parse_guest_config)] 
        config: GuestConfig
    }
}

#[cfg(unix)]
fn home() -> String {
    std::env::var("HOME")
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            std::process::exit(1);
        })
}

#[cfg(target_os = "macos")]
fn runtime_dir() -> String {
    format!("{}/Library/Application Support/emulot", home())
}

#[cfg(target_os = "macos")]
fn data_dir() -> String {
    format!("{}/Library/Application Support/emulot", home())
}

#[cfg(target_os = "linux")]
fn runtime_dir() -> String {
    std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            std::process::exit(1);
        })
}


#[cfg(target_os = "linux")]
fn data_dir() -> String {
    format!("{}/.config/emulot", home())
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            std::process::exit(1);
        })
}




#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[command(subcommand)]
    command: Command

}

impl TryFrom<Option<GuestConfig>> for GuestConfig {
    type Error = Error;

    fn try_from(config: Option<GuestConfig>) -> Result<Self, Self::Error> {
        if let Some(config) = config {
            Ok(config)
        } else {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            toml::from_str(&buf).map_err(|err| err.into())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Command::Run { config } => {
            let mut harness = config.build()?;
            while harness.running()? {
                
            }
            Ok(())
        },
       Command::Validate { config: _ } => Ok(())
    }.unwrap_or_else(|err: Error| {
        eprintln!("{err}");
        std::process::exit(-1);
    });
    Ok(())
}
