use discord_ferris::prelude::*;

#[event_handler]
pub async fn on_channel_create(ctx: Ctx, cc: ChannelCreate) {
    discord_ferris::log!("LOG", "{:.?}", cc);
    let _ = ctx.reply("If you see this message, worry about it.").await; // lol
}
