use std::error::Error;

mod events; // <-- top-level

#[cfg(feature = "examples")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use discord_ferris::client::Client;
    use discord_ferris::models::gateway::GatewayIntents;
    use dotenvy::dotenv;

    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Missing token");

    let intents = GatewayIntents::all();
    let mut client = Client::new(&token, intents);

    client.login().await?;
    Ok(())
}

#[cfg(not(feature = "examples"))]
fn main() {
    eprintln!("This example requires the `examples` feature");
    eprintln!("Usage: cargo run --features examples");
}
