use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterParams {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub bing_urls: Vec<String>,
    pub twitter_params: TwitterParams,
}

impl AppConfig {
    #[allow(dead_code)]
    pub fn save(&self, path: &str) {
        let serialized = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, serialized).expect("Unable to write config");
    }

    pub fn new(path: &str) -> Self {
        let mut content = String::with_capacity(100_000);

        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut content).unwrap();

        let config: AppConfig = serde_json::from_str(&content).unwrap();
        return config;
    }
}
