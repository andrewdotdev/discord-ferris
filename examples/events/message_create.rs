use discord_ferris::framework::context::Ctx;
use discord_ferris::models::gateway::GatewayMessageCreateDispatchData;

// Runs on every MESSAGE_CREATE.
pub async fn on_message_create(ctx: Ctx, mc: GatewayMessageCreateDispatchData) {
    if mc.is_bot() {
        return;
    }
    let content = mc.message.content;
    if content == "!ping" {
        let _ = ctx.reply("PONG!").await;
    }
}
