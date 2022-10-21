use std::time::Duration;

use cosmrs::cosmwasm::{
    AccessConfig, AccessType, MsgInstantiateContract, MsgStoreCode, MsgStoreCodeResponse,
};
use indicatif::ProgressBar;
use serde_json::json;

use crate::{
    commands::beacon::project_config::ProjectConfig,
    cosmos::wallet::Wallet,
    utils::wasm_fetch::{download_file, fetch_release_url},
    utils::CLITheme,
};

#[allow(clippy::too_many_lines)]
pub async fn deploy_beacon(network: Option<String>, wallet: Option<String>, config: &mut ProjectConfig) {
    let theme = CLITheme::default();
    let mut network = match config.get_network(&network) {
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

    let wallet_name = wallet.or_else(|| config.default_wallet.clone()).unwrap_or_else(|| {
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
    let pb = ProgressBar::new(1);
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(CLITheme::spinner());
    pb.set_message("Fetching latest release...");
    let wasm_download_url = fetch_release_url().await.unwrap_or_else(|err| {
        pb.set_style(CLITheme::failed_spinner());
        pb.set_prefix("✗");
        pb.finish_with_message(format!("{} {}", "Error fetching latest release:", err));
        std::process::exit(1);
    });
    pb.set_message("Downloading latest release...");
    let download_path = std::env::temp_dir().join("mock_beacon.wasm");
    let wasm_file = download_file(wasm_download_url, download_path)
        .await
        .unwrap_or_else(|err| {
            pb.set_style(CLITheme::failed_spinner());
            pb.set_prefix("✗");
            pb.finish_with_message(format!("{} {}", "Error downloading latest release:", err));
            std::process::exit(1);
        });
    pb.set_message("Uploading beacon contract...");

    let wasm_bytes = std::fs::read(wasm_file).unwrap_or_else(|err| {
        pb.set_style(CLITheme::failed_spinner());
        pb.set_prefix("✗");
        pb.finish_with_message(format!("{} {}", "Error reading WASM file:", err));
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

    let hash = wallet.broadcast_msg(msg).await.unwrap_or_else(|err| {
        pb.set_style(CLITheme::failed_spinner());
        pb.set_prefix("✗");
        pb.finish_with_message(format!("{} {}", "Error uploading beacon contract:", err));
        std::process::exit(1);
    });
    pb.set_message("Waiting for transaction to be included in block...");
    let res = wallet.wait_for_hash(hash).await.unwrap_or_else(|err| {
        pb.set_style(CLITheme::failed_spinner());
        pb.set_prefix("✗");
        pb.finish_with_message(format!(
            "{} {}",
            "Error waiting for transaction to be included in block:", err
        ));
        std::process::exit(1);
    });

    let res = MsgStoreCodeResponse::try_from(res).unwrap_or_else(|err| {
        pb.set_style(CLITheme::failed_spinner());
        pb.set_prefix("✗");
        pb.finish_with_message(format!(
            "{} {}",
            "Error decoding transaction response:", err
        ));
        std::process::exit(1);
    });

    pb.set_message("Instantiating mock beacon contract...");
    let msg = MsgInstantiateContract {
        sender: wallet.address.clone(),
        admin: Some(wallet.address.clone()),
        code_id: res.code_id,
        label: Some("Entropy Beacon (MOCK)".to_string()),
        msg: json!({
            "protocol_fee": 0,
            "native_denom": wallet.network.gas_info.denom.clone(),
        })
        .to_string()
        .into_bytes(),
        funds: vec![],
    };

    let hash = wallet.broadcast_msg(msg).await.unwrap_or_else(|err| {
        pb.set_style(CLITheme::failed_spinner());
        pb.set_prefix("✗");
        pb.finish_with_message(format!(
            "{} {}",
            "Error instantiating mock beacon contract:", err
        ));
        std::process::exit(1);
    });

    pb.set_message("Waiting for transaction to be included in block...");
    let res = wallet.wait_for_hash(hash).await.unwrap_or_else(|err| {
        pb.set_style(CLITheme::failed_spinner());
        pb.set_prefix("✗");
        pb.finish_with_message(format!(
            "{} {}",
            "Error waiting for transaction to be included in block:", err
        ));
        std::process::exit(1);
    });

    let deployed_address = res.logs[0]
        .events
        .iter()
        .find(|e| e.type_ == "instantiate")
        .and_then(|e| e.attributes.get("_contract_address"))
        .unwrap_or_else(|| {
            pb.set_style(CLITheme::failed_spinner());
            pb.set_prefix("✗");
            pb.finish_with_message("Error decoding transaction response.");
            std::process::exit(1);
        });

    pb.set_style(CLITheme::success_spinner());
    pb.set_prefix("✓");
    pb.finish_with_message(format!(
        "{} {}",
        "Mock beacon contract instantiated at address:",
        theme.highlight.apply_to(deployed_address)
    ));

    network.deployed_beacon_address = Some(deployed_address.to_string());
}
