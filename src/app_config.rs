use serde::{Deserialize, Serialize};

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
