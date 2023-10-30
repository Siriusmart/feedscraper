use std::path::PathBuf;

use scrapyard::Saveable;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct FScraperConfig {
    pub http_port: Option<u16>,
    pub https: Option<HttpsConfig>,
    #[serde_inline_default(true)]
    pub show_feed_configs: bool,
}

impl Default for FScraperConfig {
    fn default() -> Self {
        Self {
            http_port: Some(8080),
            https: Some(HttpsConfig::default()),
            show_feed_configs: true,
        }
    }
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, DefaultFromSerde, Debug)]
pub struct HttpsConfig {
    #[serde_inline_default(8081)]
    pub port: u16,
    #[serde_inline_default(PathBuf::from("/etc/letsencrypt/live/yourdomain.com/fullchain.pem"))]
    pub chain: PathBuf,
    #[serde_inline_default(PathBuf::from("/etc/letsencrypt/live/yourdomain.com/privkey.pem"))]
    pub key: PathBuf,
}

impl Saveable for FScraperConfig {}
