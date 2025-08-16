use discord_ferris::df_prelude::GatewayMessageReactionAddDispatchData;
use discord_ferris::framework::context::Ctx;

// Runs on every MESSAGE_REACTION_ADD.
pub async fn on_message_reaction_add(ctx: Ctx, mra: GatewayMessageReactionAddDispatchData) {
    discord_ferris::log!(
        "LOG",
        "{} reactioned with {:.?} on {}",
        mra.member.unwrap().user.user.username,
        mra.emoji.base.name.unwrap(),
        mra.channel_id
    );
    let _ = ctx.reply("If you see this message, worry about it.").await; // lol
}
