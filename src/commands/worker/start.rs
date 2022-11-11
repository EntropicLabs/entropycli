#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::str::FromStr;

use clap::Parser;
use cosmrs::{tx::Gas, AccountId};
use ecvrf_rs::{decode_hex, Proof};
use entropy_beacon_cosmos::beacon::BEACON_BASE_GAS;

use crate::{
    cosmos::{utils::mul_gas_float, wallet::Wallet},
    utils::{
        beacon_interface::Beacon,
        config::{ConfigType, ConfigUtils},
        webhook, CLITheme,
    },
};

#[derive(Debug, Parser, Clone)]
pub struct StartCommandOptions {
    /// Path to the configuration file
    #[clap(short, long)]
    #[clap(default_value = "config.json")]
    config: String,
    /// Verbose mode
    #[clap(short, long)]
    #[clap(default_value = "false")]
    verbose: bool,
    /// Fee granter address
    #[clap(long)]
    fee_granter: Option<String>,
}

#[allow(clippy::too_many_lines)]
pub async fn start_cmd(options: StartCommandOptions) {
    let theme = CLITheme::default();
    println!(
        "{}",
        dialoguer::console::style(format!("entropy worker v{}", env!("CARGO_PKG_VERSION"))).bold()
    );
    let config = ConfigUtils::load(&options.config).unwrap_or_else(|e| {
        eprintln!(
            "{} {}",
            theme.error.apply_to("Error loading config file: "),
            theme.error.apply_to(e.to_string())
        );
        std::process::exit(1);
    });
    let config = if let ConfigType::Worker(config) = config {
        config
    } else {
        eprintln!(
            "{}",
            theme
                .error
                .apply_to("Config file is not a worker config file.")
        );
        std::process::exit(1);
    };

    if config.registered_keys.is_empty() {
        eprintln!(
            "{}",
            theme.error.apply_to("No keys registered, please create and whitelist keys using entropycli, or add existing whitelisted keys to the config file")
        );
        std::process::exit(1);
    }

    let network_name = config.default_network.unwrap_or_else(||
        std::env::var("NETWORK").unwrap_or_else(|_|{
            eprintln!(
                "{}",
                theme.error.apply_to("No default network set, please set the default network in the config file, or set the NETWORK environment variable")
            );
            std::process::exit(1);
        })
    );

    let network_info = config.networks.get(&network_name).unwrap_or_else(|| {
        eprintln!(
            "{} {}, {}",
            theme
                .error
                .apply_to("No network configuration found with the name"),
            theme.error.apply_to(&network_name),
            theme
                .error
                .apply_to("please add the network to the config file manually or with entropycli")
        );
        std::process::exit(1);
    });

    let gas_info = network_info.network.gas_info.clone();

    let beacon_address = network_info
        .network
        .deployed_beacon_address
        .clone()
        .unwrap_or_else(|| {
            eprintln!(
                "{} {} {}",
                theme.error.apply_to("No beacon address found for network"),
                theme.error.apply_to(&network_name),
                theme.error.apply_to(
                    "please add the beacon address to the config file manually or with entropycli"
                )
            );
            std::process::exit(1);
        });

    let mnemonic = network_info.signer_mnemonic.clone().unwrap_or_else(||
        std::env::var("MNEMONIC").unwrap_or_else(|_|{
            eprintln!(
                "{}",
                theme.error.apply_to("No mnemonic set, please set the mnemonic in the config file, or set the MNEMONIC environment variable")
            );
            std::process::exit(1);
        })
    );

    let beacon = Beacon::new(
        network_info.network.clone(),
        Wallet::new(mnemonic, network_info.network.clone()).unwrap_or_else(|_| {
            eprintln!(
                "{}",
                theme.error.apply_to("Failed to create wallet from mnemonic, please check the mnemonic in the config file")
            );
            std::process::exit(1);
        }),
        beacon_address,
    );

    let webhook_url = std::env::var("WEBHOOK_URL").ok();

    let fee_granter = options
        .fee_granter
        .map_or(std::env::var("FEE_GRANTER").ok(), Some)
        .map(|fee_granter| {
            AccountId::from_str(fee_granter.as_str()).unwrap_or_else(|_| {
                eprintln!(
                    "{} {}",
                    theme.error.apply_to("Invalid fee granter address: "),
                    theme.error.apply_to(fee_granter)
                );
                std::process::exit(1);
            })
        });

    let mut current_key = 0;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let active_requests = beacon.fetch_active_requests().await;
        if active_requests.is_err() {
            let message = format!(
                "Failed to fetch active requests: {}",
                active_requests.err().unwrap()
            );
            eprintln!("[WARN] {}", message);
            if let Some(webhook_url) = &webhook_url {
                let res = webhook::error(webhook_url, message).await;
                if res.is_err() {
                    eprintln!("[WARN] Failed to send webhook: {}", res.err().unwrap());
                }
            }
            continue;
        }
        let active_requests = active_requests.unwrap();
        let requests = active_requests.requests;
        if requests.is_empty() {
            continue;
        }
        let total_payout = requests
            .iter()
            .map(|r| r.submitted_bounty_amount.u128())
            .sum::<u128>();

        let total_callback_gas =
            BEACON_BASE_GAS + requests.iter().map(|r| r.callback_gas_limit).sum::<u64>();

        let total_gas_cost = mul_gas_float(total_callback_gas, gas_info.gas_price).value();

        if total_payout < total_gas_cost.into() {
            eprintln!(
                "[WARN] Not enough funds to pay for gas, skipping ({} < {})",
                total_payout, total_gas_cost
            );
            continue;
        }

        if options.verbose {
            println!(
                "[INFO] {} active requests, callback gas: {}, total payout: {}",
                requests.len(),
                total_callback_gas,
                total_payout
            );
        }

        let last_entropy = beacon.fetch_last_entropy().await;
        if last_entropy.is_err() {
            let message = format!(
                "Failed to fetch last entropy: {}",
                last_entropy.err().unwrap()
            );
            eprintln!("[WARN] {}", message);

            if let Some(webhook_url) = &webhook_url {
                let res = webhook::error(webhook_url, message).await;
                if res.is_err() {
                    eprintln!("[WARN] Failed to send webhook: {}", res.err().unwrap());
                }
            }
            continue;
        }

        if options.verbose {
            println!(
                "[INFO] Last entropy: {}",
                last_entropy.as_ref().unwrap().entropy
            );
        }

        let last_entropy = decode_hex(last_entropy.unwrap().entropy.as_str()).unwrap();
        let secret_key = &config.registered_keys[current_key];
        let proof = Proof::new(secret_key, &last_entropy).unwrap();
        println!(
            "[INFO] Submitting entropy with proof {}",
            serde_json::to_string(&proof).unwrap()
        );
        let res = beacon
            .submit_entropy(
                &proof,
                Gas::from(total_callback_gas),
                vec![],
                fee_granter.clone(),
            )
            .await;
        if res.is_err() {
            let message = format!("Failed to submit entropy: {}", res.err().unwrap());
            eprintln!("[WARN] {}", message);
            if let Some(webhook_url) = &webhook_url {
                let res = webhook::error(webhook_url, message).await;
                if res.is_err() {
                    eprintln!("[WARN] Failed to send webhook: {}", res.err().unwrap());
                }
            }
            continue;
        }
        let res = res.unwrap();
        let message = format!("Submitted entropy with hash {}", res.txhash,);

        println!("[INFO] {}", message);
        if let Some(webhook_url) = &webhook_url {
            let res = webhook::info(webhook_url, message).await;
            if res.is_err() {
                eprintln!("[WARN] Failed to send webhook: {}", res.err().unwrap());
            }
        }

        if options.verbose {
            println!("[INFO] Response: {:?}", res);
        }

        current_key = (current_key + 1) % config.registered_keys.len();
    }
}
