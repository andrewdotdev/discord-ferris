use serde::Deserialize;
use std::fs;
use std::error::Error;

#[derive(Deserialize)]
struct Config {
    token: String,
}

fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

#[cfg(feature = "examples")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use discord_ferris::structs::gateway::GatewayIntents;
    use discord_ferris::client::Client;

    let config = load_config("env.example.json")?;
    let token = config.token;

    let mut client = Client::new(&token, GatewayIntents::non_privileged());
    client.login().await?;

    Ok(())
}

#[cfg(not(feature = "examples"))]
fn main() {
    eprintln!("warning: This example requires the `examples` feature");
    eprintln!("usage: cargo run --example ferris-example --features examples");
}
