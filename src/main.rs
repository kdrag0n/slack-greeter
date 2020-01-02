extern crate stderrlog;
#[macro_use]
extern crate log;

use std::fs;
use toml::Value;
use slack::RtmClient;

mod handler;

fn main() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(2)
        .color(stderrlog::ColorChoice::Auto)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();

    let config_data = fs::read_to_string("config.toml")
        .expect("Unable to read config file");
    let config = config_data.parse::<Value>()
        .expect("Unable to parse config as TOML");

    let api_key = config["slack"]["api_key"].as_str()
        .expect("API key is not a string");
    let greetings = config["bot"]["greetings"].as_array()
        .expect("Greetings is not a list")
        .iter()
        .map(|val| val.as_str().expect("Greeting is not a string").to_string())
        .collect();
    let mut handler = handler::Handler { greetings };

    info!("Starting bot...");
    RtmClient::login_and_run(api_key, &mut handler)
        .expect("Failed to start client");
}
