/// This file contains definition of the application configuration.
/// 
/// Application tries to load configuration file located in the same folder as the executable file.
/// Configuration file name: gas_estimator.cfg
/// Content:
///  {
///    "rpc_url": "some url of the RPC node",
///    "bind_addr": "0.0.0.0:80"
///  }

use reqwest::Url;
use serde::Deserialize;
use std::{env, fs::File, io::BufReader};


#[derive(Clone, Deserialize)]
/// Application configuration
pub struct Config {
    pub rpc_url: Url,
    pub bind_addr: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        match env::current_exe() {
            Ok(exe_path) => {
                let config_file = exe_path.with_extension("cfg");
                let file_location = config_file.display().to_string();
                if let Ok(file) = File::open(config_file) {
                    let reader = BufReader::new(file);
                    let val: Self = serde_json::from_reader(reader)
                        .unwrap_or_else(|_| panic!("Configuration file bad format. {}", file_location));
                    log::info!("Using configuration file: {}", file_location);
                    val
                } else {
                    Self::default()
                }
            }
            Err(_) => Self::default()
        }
    }

    fn default() -> Self {
        log::info!("Using default configuration");
        Self {
            rpc_url: Url::parse("https://rpc.flashbots.net/fast").unwrap(),
            bind_addr: Some(String::from("0.0.0.0:3000")),
        }
    }
}
