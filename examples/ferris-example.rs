use std::error::Error;

#[cfg(feature = "examples")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use discord_ferris::client::Client;
    use discord_ferris::structs::gateway::GatewayIntents;
    use dotenvy::dotenv;

    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("Missing token");

    let mut client = Client::new(&token, GatewayIntents::non_privileged());
    client.login().await?;
    Ok(())
}

#[cfg(not(feature = "examples"))]
fn main() {
    eprintln!("This example requires the `examples` feature");
    eprintln!("Usage: cargo run --example ferris-example --features examples");
}
