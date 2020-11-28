use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub telegram: Telegram
}

#[derive(Debug, Clone, Deserialize)]
pub struct Telegram {
    pub token: String
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.merge(File::new("settings.yaml", FileFormat::Yaml))?;

        let settings = config.try_into()?;
        Ok(settings)
    }
}