use discord_ferris::prelude::*;

#[event_handler]
pub async fn on_message_reaction_add(ctx: Ctx, mra: MessageReactionAdd) {
    discord_ferris::log!(
        "LOG",
        "{} reactioned with {:.?} on {}",
        mra.member.unwrap().user.user.username,
        mra.emoji.base.name.unwrap(),
        mra.channel_id
    );
    let _ = ctx.reply("If you see this message, worry about it.").await; // lol
}
