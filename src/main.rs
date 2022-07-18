use anyhow::anyhow;
use chell::{client::connect, server::listen};
use clap::{Parser, Subcommand};
use log::info;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Verbosity level
    #[clap(short, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Host to connect to. Should be of form ip:port
    #[clap(value_parser)]
    host: Option<String>, // TODO: make exclusive to no subcommand

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a server
    Serve {
        /// Which port to host on
        #[clap(value_parser)]
        port: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    env_logger::builder()
        .filter_level(match cli.verbosity {
            x if x >= 3 => log::LevelFilter::Trace,
            2 => log::LevelFilter::Debug,
            1 => log::LevelFilter::Info,
            0 => log::LevelFilter::Warn,
            _ => unreachable!(),
        })
        .init();
    info!("Verbosity at {}", cli.verbosity);

    match cli.command {
        None => {
            // Client
            #[rustfmt::skip] // for now. to fix later
            let rhost = cli.host
                .ok_or_else(|| anyhow!("Client host must be provided"))?;
            info!("Client connecting to {}", rhost);
            connect(rhost).await?;
        }
        Some(Commands::Serve { port: p }) => {
            // Server
            let localhost = format!("127.0.0.1:{p}");
            info!("Starting server at {}", localhost);
            listen(localhost).await?;
        }
    }

    Ok(())
}
