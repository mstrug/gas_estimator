use reqwest::Url;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Deserialize)]
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
                let file = File::open(config_file)
                    .expect(&format!("Unable to read config file: {}", file_location));
                let reader = BufReader::new(file);
                let val: Self = serde_json::from_reader(reader)
                    .expect(&format!("Configuration file bad format. {}", file_location));
                val
            }
            Err(e) => panic!("Failed to get current exe path: {e}"),
        }
    }
}
