use std::error::Error;

#[cfg(feature = "examples")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use discord_ferris::client::Client;
    use discord_ferris::models::gateway::{
        GatewayDispatchEvents as GwEvt, GatewayIntents, GatewayMessageCreateDispatchData,
        GatewayReadyDispatchData,
    };
    use dotenvy::dotenv;
    // use serde_json::Value;

    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Missing token");

    let intents = GatewayIntents::all();
    let mut client = Client::new(&token, intents);

    // READY (once) — payload typed
    client.once::<GatewayReadyDispatchData, _, _>(GwEvt::Ready, |_ctx, ready| async move {
        discord_ferris::log!(
            "OK",
            "Logged as {} session={}",
            ready.user.username,
            ready.session_id
        );
    });

    // MESSAGE_CREATE — payload typed
    client.on::<GatewayMessageCreateDispatchData, _, _>(
        GwEvt::MessageCreate,
        |ctx, mc| async move {
            let is_bot = mc.message.base.author.bot.unwrap_or(false);
            if is_bot {
                return;
            }

            println!("{:.?}", mc); // only for testing
            let content = mc.message.base.content;
            if content == "!ping" {
                let _ = ctx.reply("PONG!").await;
            }
        },
    );

    client.login().await?;
    Ok(())
}

#[cfg(not(feature = "examples"))]
fn main() {
    eprintln!("This example requires the `examples` feature");
    eprintln!("Usage: cargo run --features examples");
}
