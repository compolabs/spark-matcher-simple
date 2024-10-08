mod config;
mod matcher_client;
mod error;
mod order_processor;

use config::AppConfig;
use matcher_client::MatcherClient;
use url::Url;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let def_conf_path = "config.toml".to_string();
    let config_path = args.get(1).unwrap_or(&def_conf_path);

    let app_config = AppConfig::new(config_path);
    let settings = Arc::new(app_config.settings);

    let ws_url = Url::parse(&settings.websocket_url).expect("Invalid WebSocket URL");
    let matcher_client = MatcherClient::new(ws_url, settings);

    match matcher_client.connect().await {
        Ok(_) => println!("Matcher connected successfully"),
        Err(e) => eprintln!("Failed to connect matcher: {:?}", e),
    }
    Ok(())
}
