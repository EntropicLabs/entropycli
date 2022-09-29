use std::{io::Write, path::PathBuf};

use crate::{
    args::InitCommandOptions,
    config::{Config, NetworkInfo},
};

use colored::*;

pub fn init(options: InitCommandOptions) {
    println!("Entropy CLI: Initializing new beacon.");
    if check_config(options.config.clone()) {
        println!("{}", "Config file found. Skipping config creation.".green());
    } else {
        println!("{}", "Config file not found.".red());
        loop {
            print!(
                "Would you like to create a new config file? {}: ",
                "[Y/n]".yellow()
            );
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "y" | "Y" | "yes" | "Yes" | "" => {
                    create_config(options.config.clone());
                    break;
                }
                "n" | "N" | "no" | "No" => {
                    println!(
                        "{}",
                        "No config found, did you forget to specify --config?".red()
                    );
                    println!("{}", "Aborting...".red());
                    std::process::exit(1);
                }
                _ => {
                    println!("{}", "Invalid input.".red());
                    continue;
                }
            }
        }
    }
    let config = Config::load(options.config);
    if loop {
        print!(
            "Would you like to deploy a new instance of Beacon? {}",
            "[Y/n]".yellow()
        );
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "y" | "Y" | "yes" | "Yes" | "" => {
                break true;
            }
            "n" | "N" | "no" | "No" => {
                println!("{}", "Skipping Beacon deployment...".green());
                break false;
            }
            _ => {
                println!("{}", "Invalid input.".red());
                continue;
            }
        }
    } {
        let network = match options.network {
            Some(network) => network,
            None => {
                if config.network.len() == 1 {
                    config.network[0].name.clone()
                } else {
                    println!(
                        "{}",
                        "No network specified, did you forget to use the --network option?".red()
                    );
                    println!("{}", "Aborting...".red());
                    std::process::exit(1);
                }
            }
        };
        let network_info = config
            .network
            .iter()
            .find(|network_info| network_info.name.eq_ignore_ascii_case(&network))
            .unwrap_or_else(|| {
                println!(
                    "{} {} {}",
                    "Network".red(),
                    network.red(),
                    "not found in config.".red()
                );
                println!("{}", "Aborting...".red());
                std::process::exit(1);
            });

        let wallet_name = match options.wallet {
            Some(wallet_name) => wallet_name,
            None => network_info.default_wallet.clone(),
        };

        let wallet_info = config
            .wallet
            .iter()
            .find(|wallet_info| wallet_info.name.eq_ignore_ascii_case(&wallet_name))
            .unwrap_or_else(|| {
                println!(
                    "{} {} {}",
                    "Wallet".red(),
                    wallet_name.red(),
                    "not found in config.".red()
                );
                println!("{}", "Aborting...".red());
                std::process::exit(1);
            });
        
        

        println!("{}", "Deploying Beacon...".green());
    }
}

// Check whether a configuration file exists in the current directory
// If it does, return the path to the file
// If it doesn't, create a new configuration file and return the path to the file
pub fn check_config(path: Option<String>) -> bool {
    let path = match path {
        Some(path) => PathBuf::from(path),
        None => std::env::current_dir().unwrap().join("entropy.toml"),
    };
    path.exists()
}

pub fn create_config(path: Option<String>) {
    let network_info = loop {
        println!("Please select a network type: ");
        println!("\t1. LocalTerra");
        println!("\t2. Local Kujira Core (use kujirad)");
        print!("Network {}: ", "[1, 2]".yellow());
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => break NetworkInfo::localterra(),
            "2" => break NetworkInfo::localkujira(),
            _ => {
                println!("{}", "Invalid input.".red());
                continue;
            }
        }
    };
    let path = path.map_or(
        std::env::current_dir().unwrap().join("entropy.toml"),
        PathBuf::from,
    );
    println!("Creating new config file at {}", path.to_str().unwrap());
    let mut file = std::fs::File::create(path).unwrap();
    let config = Config {
        network: vec![network_info],
        wallet: vec![],
    };
    let toml = toml::to_string(&config).unwrap();
    file.write_all(toml.as_bytes()).unwrap();
    println!("{}", "Configuration saved to file.".green());
}
