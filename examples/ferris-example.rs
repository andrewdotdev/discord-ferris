use std::error::Error;

mod events; // <-- top-level, NOT inside main()

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

    // Type is inferred by the method (Ready -> GatewayReadyDispatchData).
    client
        .once_ready(events::ready::on_ready)
        .on_message_create(events::message_create::on_message_create)
        .on_message_reaction_add(events::message_reaction_add::on_message_reaction_add);

    client.login().await?;
    Ok(())
}

#[cfg(not(feature = "examples"))]
fn main() {
    eprintln!("This example requires the `examples` feature");
    eprintln!("Usage: cargo run --features examples");
}
