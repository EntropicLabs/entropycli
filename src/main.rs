mod args;
mod commands;
mod config;
mod wasm_fetch;

use clap::Parser;
use wasm_fetch::get_latest_release;

use crate::args::{Cli, Command};
use crate::commands::init::init;
fn main() {
    get_latest_release();
    let args = Cli::parse();
    match args.command {
        Command::Init(options) => init(options),
    }
    // println!("{:?}", args);
}
