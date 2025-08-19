use discord_ferris::prelude::*;

#[event_handler]
pub async fn on_ready(_ctx: Ctx, ready: Ready) {
    discord_ferris::log!(
        "OK",
        "Logged as {} session={}",
        ready.user.username,
        ready.session_id
    );
}
