use clap::Parser;
use clap::Subcommand;
use system_harness::QemuSystemConfig;
use system_harness::SystemHarness;
use system_harness::SystemConfig;
use rusqlite::Connection as SqliteConnection;
mod error;

use error::Error;

mod terminal;

fn parse_guest_config(filename: &str) -> Result<QemuSystemConfig, std::io::Error> {
    let config = std::fs::read_to_string(filename)?;
    toml::from_str(&config)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{err}")))
}

#[derive(Subcommand)]
enum Command {
    /// Import a configuration
    Import {
        /// System name
        name: String,

        /// Config to run
        #[arg(value_parser = parse_guest_config)]
        config: QemuSystemConfig,
    },
    /// Run machine
    Run {
        /// System name
        name: String
    },
    /// Dump machine config
    Dump {
        /// System name
        name: String
    },
    /// List machines
    List,
    /// Remove machine
    Remove {
        /// System name
        name: String
    },
    /// Validates an emulator config
    Validate {
        /// Config to validate
        #[arg(value_parser = parse_guest_config)]
        config: QemuSystemConfig,
    },
}

macro_rules! expected_dir {
    ($path:expr) => {
        {
            let dir = $path;
            if !std::path::Path::new(&dir).exists() {
                std::fs::create_dir_all(&dir)
                    .unwrap_or_else(|err| {
                        eprintln!("{err}");
                        std::process::exit(1);
                    });
            }
            dir
        }
    };
}

#[cfg(unix)]
fn home() -> String {
    std::env::var("HOME").unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    })
}

#[cfg(target_os = "macos")]
fn data_dir() -> String {
    expected_dir!(format!("{}/Library/Application Support/emulot/cache", home()))
}

#[cfg(target_os = "linux")]
fn data_dir() -> String {
    expected_dir!(
        format!("{}/.config/emulot", home())
    )
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}


fn db() -> Result<SqliteConnection, Error> {
    SqliteConnection::open(format!("{}/state.db", data_dir()))
        .map_err(Into::into)
        .and_then(|conn| {
            conn.execute_batch(include_str!("schema.sql"))?;
            Ok(conn)
        })
}

fn get_config(name: &str) -> impl FnOnce(SqliteConnection) -> Result<QemuSystemConfig, Error> {
    let name = name.to_string();
    |conn: SqliteConnection| {
        let json_config: String = conn.query_row("SELECT config FROM system_config WHERE name = ?", [name], |row| {
            row.get("config")
        })?;
        serde_json::from_str(&json_config)
            .map_err(Into::into)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    match args.command {
        Command::Import { name, config } => db()
            .and_then(|connection| {
                connection.execute("INSERT INTO system_config (name, config) VALUES (?, ?)", 
                    [name, serde_json::to_string(&config)?])?;
                Ok(())
            }),
        Command::Run { name } => match db() 
            .and_then(get_config(&name)) {
                Ok(config) => {
                    let mut system = config.spawn().await?;
                    let mut terminal = system.terminal().await?;
                    crate::terminal::attach_to_stdin(&mut terminal).await?;
                    Ok(())
                },
                Err(err) => Err(err)
            },
        Command::Dump { name } => db()
            .and_then(get_config(&name))
            .and_then(|config| {
                print!("{}", toml::to_string_pretty(&config).unwrap());
                Ok(())
            }),
        Command::List => db()
            .and_then(|conn| {
                let mut stmt = conn.prepare("SELECT name FROM system_config")?;
                let mut rows = stmt.query([])?;
                while let Some(row) = rows.next()? {
                    println!("{}", row.get::<usize, String>(0)?);
                }
                Ok(())
            }),
        Command::Remove { name } => db()
            .and_then(|conn| {
                conn.execute("DELETE FROM system_config WHERE name = ?", [name])?;
                Ok(())
            }),
        Command::Validate { config: _ } => Ok(()),
    }
    .unwrap_or_else(|err: Error| {
        eprintln!("{err}");
        std::process::exit(-1);
    });
    Ok(())
}
