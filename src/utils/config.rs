use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::commands::{beacon::project_config::ProjectConfig, worker::worker_config::WorkerConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConfigType {
    Project(ProjectConfig),
    Worker(WorkerConfig),
}

pub struct ConfigUtils;

impl ConfigUtils {
    pub fn save<T, P>(config: &T, path: &P) -> Result<(), std::io::Error>
    where
        T: Sized + Serialize,
        P: AsRef<Path>,
    {
        let config = serde_json::to_string_pretty(config)?;
        std::fs::write(path, config)?;
        Ok(())
    }

    pub fn load<P>(path: &P) -> Result<ConfigType, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let config = std::fs::read_to_string(path)?;
        let config = serde_json::from_str::<ConfigType>(&config)?;
        Ok(config)
    }
}
