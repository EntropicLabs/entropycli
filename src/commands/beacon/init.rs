use std::path::PathBuf;

use crate::{
    commands::beacon::project_config::ProjectConfig,
    utils::{config::ConfigType, deploy::deploy_beacon},
    utils::{config::ConfigUtils, CLITheme},
};

use clap::Parser;
use dialoguer::Confirm;

pub const TEMPLATE_CONTRACT_REPO: &str =
    "https://github.com/EntropicLabs/entropy_example_contract.git";

#[derive(Debug, Parser, Clone)]
pub struct InitCommandOptions {
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
pub async fn init_cmd(options: InitCommandOptions) {
    let theme = CLITheme::default();

    println!(
        "{}",
        dialoguer::console::style(format!(
            "entropy beacon init v{}",
            env!("CARGO_PKG_VERSION")
        ))
        .bold()
    );

    let mut prompt = "Create new config file?";

    if PathBuf::from(&options.config).exists() {
        prompt = "Create new config file? (OVERWRITES EXISTING CONFIG)";
    }

    if Confirm::with_theme(&theme)
        .with_prompt(prompt)
        .default(false)
        .interact()
        .unwrap()
    {
        ProjectConfig::prompt_config_creation(&options.config);
    }

    let config = ConfigUtils::load(&options.config).unwrap_or_else(|e| {
        println!(
            "{} {}",
            theme.error.apply_to("Error loading config file: "),
            theme.error.apply_to(e.to_string())
        );
        std::process::exit(1);
    });
    let mut config = if let ConfigType::Project(config) = config {
        config
    } else {
        println!(
            "{}",
            theme.error.apply_to("Config file is not a project config")
        );
        std::process::exit(1);
    };

    if Confirm::with_theme(&theme)
        .with_prompt("Deploy mock beacon?")
        .default(true)
        .interact()
        .unwrap()
    {
        deploy_beacon(
            options.network,
            options.wallet,
            Option::<String>::None,
            &mut config,
        )
        .await;

        ConfigUtils::save(&config, &options.config).unwrap_or_else(|e| {
            println!(
                "{} {}",
                theme.error.apply_to("Error updating config file: "),
                theme.error.apply_to(e.to_string())
            );
            std::process::exit(1);
        });

        println!(
            "{}",
            theme
                .dimmed
                .apply_to("Updated config file with deployed beacon address")
        );
    }

    if Confirm::with_theme(&theme)
        .with_prompt("Download template contract to current directory?")
        .default(true)
        .interact()
        .unwrap()
    {
        // Clone the template repo with git
        let download_path = std::env::temp_dir().join("entropy_template_contract");
        if download_path.exists() {
            std::fs::remove_dir_all(&download_path).unwrap_or_else(|e| {
                println!(
                    "{} {}",
                    theme.error.apply_to("Error removing existing template contract directory:"),
                    theme.error.apply_to(e.to_string())
                );
                std::process::exit(1);
            });
        }

        let status = std::process::Command::new("git")
            .arg("clone")
            .arg(TEMPLATE_CONTRACT_REPO)
            .arg(download_path.to_str().unwrap())
            .status()
            .unwrap_or_else(|e| {
                println!(
                    "{} {}",
                    theme
                        .error
                        .apply_to("Error downloading template contract: "),
                    theme.error.apply_to(e.to_string())
                );
                std::process::exit(1);
            });
        if !status.success() {
            println!(
                "{}",
                theme.error.apply_to(
                    "Error downloading template contract: git exited with non-zero status"
                )
            );
            std::process::exit(1);
        }
        let cwd = std::env::current_dir().unwrap();

        for entry in std::fs::read_dir(download_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path().clone();
            let target = cwd.join(entry.path().file_name().unwrap());
            if path.is_dir() && path.file_name().unwrap() == ".git" {
                continue;
            }
            std::process::Command::new("cp")
                .arg("-r")
                .arg(path)
                .arg(target)
                .status()
                .unwrap_or_else(|e| {
                    println!(
                        "{} {}",
                        theme.error.apply_to("Error copying template contract: "),
                        theme.error.apply_to(e.to_string())
                    );
                    std::process::exit(1);
                });
            if !status.success() {
                println!(
                    "{}",
                    theme.error.apply_to(
                        "Error copying template contract: cp exited with non-zero status"
                    )
                );
                std::process::exit(1);
            }
        }

        println!(
            "{}",
            theme
                .dimmed
                .apply_to("Template contract downloaded to current directory")
        );
    }
}
