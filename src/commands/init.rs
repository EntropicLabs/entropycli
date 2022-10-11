use std::{collections::HashMap, io::Write, path::PathBuf, str::FromStr};

use crate::{
    args::InitCommandOptions,
    config::Config,
    cosmos::{
        network::{Network, NetworkAccountInfo, NetworkGasInfo},
        wallet::Wallet,
    },
    wasm_fetch::{download_file, fetch_release_url},
};

use cosmrs::{cosmwasm::{AccessConfig, AccessType, MsgStoreCode, MsgInstantiateContract, MsgStoreCodeResponse}};

use cosmrs::tendermint::chain::Id as ChainId;

use bip32::DerivationPath;
use colored::*;
use serde_json::json;

pub async fn init(options: InitCommandOptions) {
    println!("Entropy CLI: Initializing new beacon.");

    prompt_config_creation(options.clone());

    let config = Config::load(options.config.clone());

    loop {
        print!(
            "Would you like to deploy a new instance of Beacon? {}",
            "[Y/n]".yellow()
        );
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "y" | "Y" | "yes" | "Yes" | "" => {
                deploy_beacon(options, config).await;
                return;
            }
            "n" | "N" | "no" | "No" => {
                println!("{}", "Skipping Beacon deployment...".green());
                return;
            }
            _ => {
                println!("{}", "Invalid input.".red());
                continue;
            }
        }
    }
}

pub async fn deploy_beacon(options: InitCommandOptions, config: Config) {
    let network = match options.network {
        Some(network) => network,
        None => match config.default_network {
            Some(network) => network,
            None => {
                println!(
                    "{}",
                    "[E002] No network specified, did you forget to use the --network option?"
                        .red()
                );
                println!("{}", "Aborting...".red());
                std::process::exit(1);
            }
        },
    };
    let network = match network.as_str() {
        "localterra" => Network::default_localterra(),
        "localkujira" => Network::default_localkujira(),
        name => config
            .networks
            .as_ref()
            .and_then(|networks| networks.get(name))
            .unwrap_or_else(|| {
                println!(
                    "{} {} {}",
                    "[E003] Network".red(),
                    network.red(),
                    "not found in config.".red()
                );
                println!("{}", "Aborting...".red());
                std::process::exit(1);
            })
            .clone(),
    };

    let wallet_name = match options.wallet {
        Some(wallet_name) => wallet_name,
        None => match config.default_wallet {
            Some(wallet_name) => wallet_name,
            None => {
                println!(
                    "{}",
                    "[E004] No wallet specified, did you forget to use the --wallet option?".red()
                );
                println!("{}", "Aborting...".red());
                std::process::exit(1);
            }
        },
    };

    let wallet = config
        .wallets
        .as_ref()
        .and_then(|wallets| wallets.get(&wallet_name));

    let wallet = match wallet {
        Some(Some(mnemonic)) => Wallet::new(mnemonic.clone(), network.clone()),
        Some(None) => {
            let mnemonic = std::env::var("MNEMONIC").unwrap_or_else(|_| {
                println!(
                    "{} {} {}",
                    "[E005] Wallet".red(),
                    wallet_name.red(),
                    "mnemonic not found in config, and no MNEMONIC environment variable found."
                        .red()
                );
                println!("{}", "Aborting...".red());
                std::process::exit(1);
            });
            Wallet::new(mnemonic, network.clone())
        }
        None => {
            println!(
                "{} {} {}",
                "[E006] Wallet".red(),
                wallet_name.red(),
                "not found in config.".red()
            );
            println!("{}", "Aborting...".red());
            std::process::exit(1);
        }
    }
    .unwrap_or_else(|e| {
        println!(
            "{} {} {} {}",
            "[E007] Wallet".red(),
            wallet_name.red(),
            "failed to initialize:".red(),
            e.to_string().red()
        );
        println!("{}", "Aborting...".red());
        std::process::exit(1);
    });

    println!("Fetching latest release...");
    let wasm_download_url = fetch_release_url().await.unwrap_or_else(|err| {
        println!("{} {}", "[E005] ".red(), format!("Error: {}", err).red());
        println!("{}", "Aborting...".red());
        std::process::exit(1);
    });
    print!("Downloading latest release...");
    let download_path = std::env::temp_dir().join("mock_beacon.wasm");
    let wasm_file = download_file(wasm_download_url, download_path)
        .await
        .unwrap_or_else(|err| {
            println!("{} {}", "[E006] ".red(), format!("Error: {}", err).red());
            println!("{}", "Aborting...".red());
            std::process::exit(1);
        });
    println!("{}", " Done.".green());

    println!("{}", "Deploying Beacon...".green());

    let wasm_bytes = std::fs::read(wasm_file).unwrap_or_else(|err| {
        println!(
            "{} {}",
            "[E007] ".red(),
            format!("Error Reading WASM file: {}", err).red()
        );
        println!("{}", "Aborting...".red());
        std::process::exit(1);
    });

    let msg = MsgStoreCode {
        sender: wallet.address.clone(),
        wasm_byte_code: wasm_bytes,
        instantiate_permission: Some(AccessConfig {
            permission: AccessType::OnlyAddress,
            address: wallet.address.clone(),
        }),
    };

    println!("Uploading mock beacon contract...");
    let hash = wallet.broadcast_msg(msg).await.unwrap_or_else(|err| {
        println!(
            "{} {}",
            "[E008] ".red(),
            format!("Error uploading WASM file: {}", err).red()
        );
        println!("{}", "Aborting...".red());
        std::process::exit(1);
    });
    print!("Waiting for transaction to be included in a block...");
    std::io::stdout().flush().unwrap();
    let res = wallet.wait_for_hash(hash).await.unwrap_or_else(|err| {
        println!(
            "{} {}",
            "[E009] ".red(),
            format!("Error waiting for transaction: {}", err).red()
        );
        println!("{}", "Aborting...".red());
        std::process::exit(1);
    });

    let res = MsgStoreCodeResponse::try_from(res).unwrap_or_else(|err| {
        println!(
            "{} {}",
            "[E010] ".red(),
            format!("Error parsing StoreCode transaction response: {}", err).red()
        );
        println!("{}", "Aborting...".red());
        std::process::exit(1);
    });

    println!("{}", " Done.".green());

    println!("Instantiating mock beacon contract...");
    let msg = MsgInstantiateContract {
        sender: wallet.address.clone(),
        admin: Some(wallet.address.clone()),
        code_id: res.code_id,
        label: Some("Entropy Beacon (MOCK)".to_string()),
        msg: json!({
            "protocol_fee": 0,
            "native_denom": wallet.network.gas_info.denom.clone(),
        }).to_string().into_bytes(),
        funds: vec![]
    };

    let hash = wallet.broadcast_msg(msg).await.unwrap_or_else(|err| {
        println!(
            "{} {}",
            "[E011] ".red(),
            format!("Error instantiating contract: {}", err).red()
        );
        println!("{}", "Aborting...".red());
        std::process::exit(1);
    });
    print!("Waiting for transaction to be included in a block...");
    std::io::stdout().flush().unwrap();
    let res = wallet.wait_for_hash(hash).await.unwrap_or_else(|err| {
        println!(
            "{} {}",
            "[E012] ".red(),
            format!("Error waiting for transaction: {}", err).red()
        );
        println!("{}", "Aborting...".red());
        std::process::exit(1);
    });
    println!("{}", " Done.".green());
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

pub fn prompt_config_creation(options: InitCommandOptions) {
    if check_config(options.config.clone()) {
        println!("{}", "Config file found. Skipping config creation.".green());
    } else {
        println!("{}", "Config file not found.".yellow());
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
                    create_config(options.config);
                    break;
                }
                "n" | "N" | "no" | "No" => {
                    println!(
                        "{}",
                        "[E001] No config found, did you forget to specify --config?".red()
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
}

// Check whether a configuration file exists in the current directory
// If it does, return the path to the file
// If it doesn't, create a new configuration file and return the path to the file
pub fn check_config(path: Option<String>) -> bool {
    let path = match path {
        Some(path) => PathBuf::from(path),
        None => std::env::current_dir().unwrap().join("entropy.json"),
    };
    path.exists()
}

pub fn create_config(path: Option<String>) {
    let network_name = loop {
        println!("Please select a default network.");
        println!("\t1. localterra");
        println!("\t2. localkujira");
        println!("\t3. Manual network setup");
        print!("Network {}: ", "[1, 2, 3]".yellow());
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => break "localterra".to_string(),
            "2" => break "localkujira".to_string(),
            "3" => {
                break loop {
                    print!("{}: ", "Network Name".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut name = String::new();
                    std::io::stdin().read_line(&mut name).unwrap();
                    println!("Using Name \"{}\"", name.trim().yellow());
                    print!("Confirm Name {}", "[Y/n]".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    match input.trim() {
                        "y" | "Y" | "yes" | "Yes" | "" => break String::from(name.trim()),
                        _ => continue,
                    }
                }
            }
            _ => {
                println!("{}", "Invalid input.".red());
                continue;
            }
        }
    };

    let network_info = match network_name.as_str() {
        "localterra" => Network::default_localterra(),
        "localkujira" => Network::default_localkujira(),
        _ => Network::new(
            loop {
                print!("{}: ", "LCD URL".yellow());
                std::io::stdout().flush().unwrap();
                let mut lcd_url = String::new();
                std::io::stdin().read_line(&mut lcd_url).unwrap();
                println!("Using LCD URL \"{}\"", lcd_url.trim().yellow());
                print!("Confirm LCD URL {}", "[Y/n]".yellow());
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                match input.trim() {
                    "y" | "Y" | "yes" | "Yes" | "" => break lcd_url.trim().to_string(),
                    _ => continue,
                }
            },
            loop {
                print!("{}: ", "Chain ID".yellow());
                std::io::stdout().flush().unwrap();
                let mut chain_id = String::new();
                std::io::stdin().read_line(&mut chain_id).unwrap();
                let chain_id = ChainId::try_from(chain_id.trim());
                if chain_id.is_err() {
                    println!("{}", "Invalid Chain ID.".red());
                    continue;
                }
                let chain_id = chain_id.unwrap();
                println!("Using Chain ID \"{}\"", chain_id.to_string().yellow());
                print!("Confirm Chain ID {}", "[Y/n]".yellow());
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                match input.trim() {
                    "y" | "Y" | "yes" | "Yes" | "" => break chain_id,
                    _ => continue,
                }
            },
            NetworkAccountInfo {
                derivation_path: loop {
                    print!("{}: ", "Derivation Path".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut derivation_path = String::new();
                    std::io::stdin().read_line(&mut derivation_path).unwrap();
                    let derivation_path = DerivationPath::from_str(derivation_path.trim());
                    if derivation_path.is_err() {
                        println!("{}", "Invalid derivation path.".red());
                        continue;
                    }
                    let derivation_path = derivation_path.unwrap();
                    println!(
                        "Using Derivation Path \"{}\"",
                        derivation_path.to_string().yellow()
                    );
                    print!("Confirm Derivation Path {}", "[Y/n]".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    match input.trim() {
                        "y" | "Y" | "yes" | "Yes" | "" => break derivation_path,
                        _ => continue,
                    }
                },
                chain_prefix: loop {
                    print!("{}: ", "Chain Prefix".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut chain_prefix = String::new();
                    std::io::stdin().read_line(&mut chain_prefix).unwrap();
                    println!("Using Chain Prefix \"{}\"", chain_prefix.trim().yellow());
                    print!("Confirm Chain Prefix {}", "[Y/n]".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    match input.trim() {
                        "y" | "Y" | "yes" | "Yes" | "" => break chain_prefix.trim().to_string(),
                        _ => continue,
                    }
                },
            },
            NetworkGasInfo {
                denom: loop {
                    print!("{}: ", "Gas Denom".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut denom = String::new();
                    std::io::stdin().read_line(&mut denom).unwrap();
                    println!("Using Gas Denom \"{}\"", denom.trim().yellow());
                    print!("Confirm Gas Denom {}", "[Y/n]".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    match input.trim() {
                        "y" | "Y" | "yes" | "Yes" | "" => break denom.trim().to_string(),
                        _ => continue,
                    }
                },
                gas_price: loop {
                    print!("{}: ", "Gas Price".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut gas_price = String::new();
                    std::io::stdin().read_line(&mut gas_price).unwrap();
                    let gas_price = gas_price.trim().parse::<f64>();
                    if gas_price.is_err() {
                        println!("{}", "Invalid gas price.".red());
                        continue;
                    }
                    let gas_price = gas_price.unwrap();
                    println!("Using Gas Price \"{}\"", gas_price.to_string().yellow());
                    print!("Confirm Gas Price {}", "[Y/n]".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    match input.trim() {
                        "y" | "Y" | "yes" | "Yes" | "" => break gas_price,
                        _ => continue,
                    }
                },
                gas_adjustment: loop {
                    print!("{}: ", "Gas Adjustment".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut gas_adjustment = String::new();
                    std::io::stdin().read_line(&mut gas_adjustment).unwrap();
                    let gas_adjustment = gas_adjustment.trim().parse::<f64>();
                    if gas_adjustment.is_err() {
                        println!("{}", "Invalid gas adjustment.".red());
                        continue;
                    }
                    let gas_adjustment = gas_adjustment.unwrap();
                    println!(
                        "Using Gas Adjustment \"{}\"",
                        gas_adjustment.to_string().yellow()
                    );
                    print!("Confirm Gas Adjustment {}", "[Y/n]".yellow());
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    match input.trim() {
                        "y" | "Y" | "yes" | "Yes" | "" => break gas_adjustment,
                        _ => continue,
                    }
                },
            },
        ),
    };

    let path = path.map_or(
        std::env::current_dir().unwrap().join("entropy.json"),
        PathBuf::from,
    );
    println!("Creating new config file at {}", path.to_str().unwrap());
    let mut file = std::fs::File::create(path).unwrap();

    let config = Config {
        networks: Some(HashMap::from_iter(vec![(
            network_name.to_string(),
            network_info,
        )])),
        default_network: Some(network_name),
        default_wallet: Some("<WALLET_NAME>".to_string()),
        wallets: Some(HashMap::from_iter(vec![(
            "<WALLET_NAME>".to_string(),
            Some("<WALLET_MNEMONIC>".to_string()),
        )])),
    };

    let json = serde_json::to_string_pretty(&config).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    println!("{}", "Configuration saved to file.".green());
    println!(
        "{}",
        "Please edit the file to add your wallet information.".green()
    );
}
