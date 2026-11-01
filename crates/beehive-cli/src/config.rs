//! Configuration file loading.
//!
//! The file is optional: with no file, the CLI falls back to the mock backend
//! and default EZSP settings. `config/beehive.toml` is gitignored; the checked
//! in template is `config/beehive.example.toml`.

use std::path::Path;

use beehive_ezsp::EzspConfig;
use serde::Deserialize;

use crate::cli::Backend;

/// Root of the TOML configuration file.
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AppConfig {
    /// Backend selected by the file; the `--backend` flag wins.
    pub backend: Option<Backend>,
    /// Settings for the EZSP backend.
    pub ezsp: Option<EzspConfig>,
}

/// Read and parse the configuration file, if it exists.
pub fn load(path: &Path) -> anyhow::Result<AppConfig> {
    match std::fs::read_to_string(path) {
        Ok(contents) => Ok(toml::from_str(&contents)?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(AppConfig::default()),
        Err(error) => Err(error.into()),
    }
}
