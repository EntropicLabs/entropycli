use clap::{Parser, Subcommand};

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
    #[clap(about = "Initialize a new instance of Beacon")]
    Init(InitCommandOptions),
}

#[derive(Debug, Parser, Clone)]
pub struct InitCommandOptions {
    /// Optional path to the configuration file
    #[clap(short, long)]
    pub config: Option<String>,
    #[clap(short, long)]
    pub network: Option<String>,
    #[clap(short, long)]
    pub wallet: Option<String>,
}
