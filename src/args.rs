use clap::{Parser, Subcommand};

use crate::commands::{network::NetworkCommandOptions, init::InitCommandOptions, deploy::DeployCommandOptions};

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
    #[clap(about = "Initialize a new project")]
    Init(InitCommandOptions),
    #[clap(about = "Manage networks")]
    Network(NetworkCommandOptions),
    #[clap(about = "Deploy a new instance of Beacon")]
    Deploy(DeployCommandOptions),
}