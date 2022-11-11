use clap::Parser;
use cosmrs::tx::Gas;
use cosmwasm_std::Uint128;
use dialoguer::Select;
use ecvrf_rs::Proof;
use entropy_beacon_cosmos::beacon::BEACON_BASE_GAS;
use rand::Rng;

use crate::{
    cosmos::{network::Network, wallet::Wallet},
    utils::{
        beacon_interface::{test_pk, Beacon},
        config::{ConfigType, ConfigUtils},
        CLITheme,
    },
};

use super::project_config::ProjectConfig;

#[derive(Debug, Parser, Clone)]
pub struct DevCommandOptions {
    /// Optional path to the configuration file
    #[clap(short, long)]
    #[clap(default_value = "entropy.json")]
    pub config: String,
    /// Network to use (defined in config). Optional if default network is set in config
    #[clap(short, long)]
    pub network: Option<String>,
    /// Wallet to use (defined in config). Optional if default wallet is set in config
    #[clap(short, long)]
    pub wallet: Option<String>,
}

#[allow(clippy::too_many_lines)]
pub async fn dev_cmd(options: DevCommandOptions) {
    println!(
        "{}",
        dialoguer::console::style(format!("entropy beacon dev v{}", env!("CARGO_PKG_VERSION")))
            .bold()
    );
    let theme = CLITheme::default();

    let (_config, network, wallet) = init_dev_cmd(options.clone());

    let beacon_address = network.deployed_beacon_address.clone().unwrap_or_else(|| {
        println!(
            "{}",
            theme
                .error
                .apply_to("No deployed beacon found in config file, please deploy a beacon first.")
        );
        std::process::exit(1);
    });

    let beacon = Beacon::new(network.clone(), wallet.clone(), beacon_address);

    let mode = Select::with_theme(&theme)
        .with_prompt("Select mode")
        .items(&[
            "Auto-submit Entropy",
            "Manual-submit Entropy",
            "Fetch Active Requests",
        ])
        .default(0)
        .interact()
        .unwrap();

    println!("{}\n", theme.dimmed.apply_to("Starting dev mode..."));

    let mut seen_requests = vec![];

    loop {
        std::thread::sleep(std::time::Duration::from_millis(200));
        let active_requests = beacon
            .fetch_active_requests()
            .await
            .unwrap_or_else(|e| {
                println!(
                    "{} {}",
                    theme.error.apply_to("Error fetching active requests:"),
                    theme.highlight.apply_to(e)
                );
                std::process::exit(1);
            })
            .requests;

        match mode {
            0 => {
                if active_requests.is_empty() {
                    continue;
                }

                let request_ids = active_requests
                    .iter()
                    .map(|r| r.id)
                    .collect::<Vec<Uint128>>();

                println!(
                    "Submitting entropy for requests: [{}]",
                    request_ids
                        .iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(", ")
                );

                let mut rng = rand::thread_rng();
                let mut entropy = [0u8; 64];
                rng.fill(&mut entropy);

                let proof = Proof {
                    signer: test_pk(),
                    message_bytes: entropy.to_vec(),
                    proof_bytes: vec![],
                };

                print!("Entropy: \"");
                for x in entropy {
                    print!("{:02x}", x);
                }
                println!("\"");

                let total_callback_gas = BEACON_BASE_GAS
                    + active_requests
                        .iter()
                        .map(|r| r.callback_gas_limit)
                        .sum::<u64>();

                let res = beacon
                    .submit_entropy(&proof, Gas::from(total_callback_gas), request_ids, None)
                    .await;
                if res.is_err() {
                    println!(
                        "{} {}",
                        theme.error.apply_to("Error submitting entropy:"),
                        theme.highlight.apply_to(res.unwrap_err())
                    );
                    continue;
                }
                let res = res.unwrap();
                println!(
                    "{} {}\n",
                    theme.success.apply_to("Entropy submitted successfully!"),
                    theme.highlight.apply_to(res.txhash.to_string())
                );
            }
            1 => {
                if active_requests.is_empty() {
                    continue;
                }

                let request_ids = active_requests
                    .iter()
                    .map(|r| r.id)
                    .collect::<Vec<Uint128>>();

                println!(
                    "Submitting entropy for requests: [{}]",
                    request_ids
                        .iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(", ")
                );

                let entropy = dialoguer::Input::with_theme(&theme)
                    .with_prompt("Enter entropy (hex)")
                    .validate_with(|input: &String| {
                        if !input.chars().all(|c| c.is_ascii_hexdigit()) {
                            return Err("Entropy must be hex".to_string());
                        }
                        if !input.len() % 2 == 0 {
                            return Err("Entropy must be even length".to_string());
                        }
                        Ok(())
                    })
                    .interact()
                    .unwrap();
                let entropy = hex::decode(entropy).unwrap();

                let proof = Proof {
                    signer: test_pk(),
                    message_bytes: entropy.clone(),
                    proof_bytes: vec![],
                };

                println!("\tEntropy: \"{:x?}\"", entropy);
                let total_callback_gas = BEACON_BASE_GAS
                    + active_requests
                        .iter()
                        .map(|r| r.callback_gas_limit)
                        .sum::<u64>();

                let res = beacon
                    .submit_entropy(&proof, Gas::from(total_callback_gas), request_ids, None)
                    .await;
                if res.is_err() {
                    println!(
                        "{} {}",
                        theme.error.apply_to("Error submitting entropy:"),
                        theme.highlight.apply_to(res.unwrap_err())
                    );
                    continue;
                }
                let res = res.unwrap();
                println!(
                    "{} {}\n",
                    theme.success.apply_to("Entropy submitted successfully!"),
                    theme.highlight.apply_to(res.txhash.to_string())
                );
            }
            2 => {
                active_requests
                    .iter()
                    .filter(|r| !&seen_requests.contains(&r.id))
                    .for_each(|r| {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&r).unwrap_or_else(|e| {
                                println!(
                                    "{} {}",
                                    theme.error.apply_to("Error serializing request:"),
                                    theme.highlight.apply_to(e)
                                );
                                std::process::exit(1);
                            })
                        );
                    });
                for r in &active_requests {
                    if !seen_requests.contains(&r.id) {
                        seen_requests.push(r.id);
                    }
                }
            }
            _ => unreachable!(),
        };
    }
}

fn init_dev_cmd(options: DevCommandOptions) -> (ProjectConfig, Network, Wallet) {
    let theme = CLITheme::default();
    let config = ConfigUtils::load(&options.config).unwrap_or_else(|e| {
        println!(
            "{} {}",
            theme.error.apply_to("Error loading config file: "),
            theme.error.apply_to(e.to_string())
        );
        std::process::exit(1);
    });
    let config = if let ConfigType::Project(config) = config {
        config
    } else {
        println!(
            "{}",
            theme.error.apply_to("Config file is not a project config")
        );
        std::process::exit(1);
    };
    let network = match config.get_network(&options.network) {
        Ok((_, Some(network))) => network,
        Ok((name, None)) => {
            println!(
                "{} {} {}",
                theme.error.apply_to("Network"),
                theme.highlight.apply_to(name),
                theme.error.apply_to("not found in config file.")
            );
            std::process::exit(1);
        }
        Err(_) => {
            println!(
                "{}",
                theme.error.apply_to("No network specified. Please specify a network with the --network flag or set a default network in the config file.")
            );
            std::process::exit(1);
        }
    };

    let wallet_name = options.wallet.or_else(|| config.default_wallet.clone()).unwrap_or_else(|| {
            println!(
                "{}",
                theme.error.apply_to("No wallet specified. Please specify a wallet with the --wallet flag or set a default wallet in the config file.")
            );
            std::process::exit(1);
        });

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
                    theme.error.apply_to("Mnemonic for wallet"),
                    theme.highlight.apply_to(&wallet_name),
                    theme
                        .error
                        .apply_to("not found in config file or MNEMONIC environment variable.")
                );
                std::process::exit(1);
            });
            Wallet::new(mnemonic, network.clone())
        }
        None => {
            println!(
                "{} {} {}",
                theme.error.apply_to("Wallet"),
                theme.highlight.apply_to(&wallet_name),
                theme.error.apply_to("not found in config file.")
            );
            std::process::exit(1);
        }
    }
    .unwrap_or_else(|e| {
        println!(
            "{} {}",
            theme.error.apply_to("Error creating wallet:"),
            theme.highlight.apply_to(e)
        );
        std::process::exit(1);
    });
    (config, network, wallet)
}
