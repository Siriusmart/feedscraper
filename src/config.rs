use std::path::PathBuf;

use scrapyard::Saveable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FScraperConfig {
    pub http_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https: Option<HttpsConfig>,
    pub show_feed_configs: bool,
}

impl Default for FScraperConfig {
    fn default() -> Self {
        Self {
            http_port: Some(8080),
            https: None,
            show_feed_configs: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpsConfig {
    pub port: u16,
    pub chain: PathBuf,
    pub key: PathBuf,
}

impl Saveable for FScraperConfig {}
