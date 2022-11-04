#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
mod commands;
mod cosmos;
mod types;
mod utils;

use clap::{Parser, Subcommand};
use commands::{
    beacon::{beacon_cmd, BeaconCommandOptions},
    network::network_cmd,
    worker::{worker_cmd, WorkerCommandOptions},
};

use crate::commands::network::NetworkCommandOptions;

#[derive(Debug, Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "A CLI tool to develop applications using the Entropic Labs ecosystem")]
#[clap(name = "entropy")]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(about = "Manage networks")]
    Network(NetworkCommandOptions),
    #[clap(about = "Manage local beacon projects")]
    Beacon(BeaconCommandOptions),
    #[clap(about = "Manage worker deployments")]
    Worker(WorkerCommandOptions),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Network(options) => network_cmd(options),
        Command::Beacon(options) => beacon_cmd(options).await,
        Command::Worker(options) => worker_cmd(options).await,
    }
}
