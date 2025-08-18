use discord_ferris::prelude::*;

#[event_handler]
pub async fn on_message_create(ctx: Ctx, mc: GatewayMessageCreateDispatchData) {
    if mc.is_bot() {
        return;
    }
    if mc.message.content == "!ping" {
        let _ = ctx.reply("PONG!").await;
    }
}
