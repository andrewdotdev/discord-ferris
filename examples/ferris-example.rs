#[cfg(feature = "examples")]
use discord_ferris::client::Client;
#[cfg(feature = "examples")]
use dotenvy::dotenv;

#[cfg(feature = "examples")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use discord_ferris::structs::gateway::GatewayIntents;

    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("Missing token");
    let mut client = Client::new(token, GatewayIntents::non_privileged());
    client.login().await?;
    println!("ESTO NO SE DEBERIA EJECUTAR");
    Ok(())
}

#[cfg(not(feature = "examples"))]
fn main() {
    eprintln!("⚠️ This example requires the `examples` feature");
    eprintln!("Run it with: cargo run --example ferris-example --features examples");
}
