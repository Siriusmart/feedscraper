use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf, sync::OnceLock};

use rustls::*;
use rustls_pemfile::*;
use scrapyard::{FeedOption, Feeds, Saveable};
use tokio::fs;

use crate::FScraperConfig;

pub static FS_CONFIG: OnceLock<FScraperConfig> = OnceLock::new();
pub static FEEDS_MAP: OnceLock<HashMap<String, FeedOption>> = OnceLock::new();
// (label, title)
pub static FEEDS_LIST: OnceLock<Vec<(String, String)>> = OnceLock::new();

impl FScraperConfig {
    pub async fn init() {
        let config_path = dirs::config_dir().unwrap().join(env!("CARGO_PKG_NAME"));
        let main_config_path = config_path.join("feedscraper.json");
        let main_config = if fs::try_exists(&main_config_path).await.unwrap() {
            FScraperConfig::load_json(&main_config_path).await.unwrap()
        } else {
            let default = FScraperConfig::default();
            default.save_json_pretty(&main_config_path).await.unwrap();
            default
        };

        FS_CONFIG.set(main_config).unwrap();
    }
}

pub fn init(feeds: Feeds) {
    FEEDS_LIST
        .set(
            feeds
                .0
                .iter()
                .map(|feed| (feed.label.clone(), feed.channel.title.clone()))
                .collect(),
        )
        .unwrap();
    FEEDS_MAP.set(feeds.to_map()).unwrap();
}

pub fn load_rustls_config(chain: &PathBuf, key: &PathBuf) -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(chain).unwrap());
    let key_file = &mut BufReader::new(File::open(key).unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
