use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GCP {
    pub project_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub database: Database,
    pub gcp: GCP,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "local".into());
        let s = Config::builder()
            .add_source(File::with_name("src/config/local.toml"))
            .add_source(File::with_name(&format!("src/config/{}.toml", run_mode)).required(false))
            .build()?;

        s.try_deserialize()
    }
}
