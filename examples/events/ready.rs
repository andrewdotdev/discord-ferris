use discord_ferris::framework::context::Ctx;
use discord_ferris::models::gateway::GatewayReadyDispatchData;

// Called once on READY.
pub async fn on_ready(_ctx: Ctx, ready: GatewayReadyDispatchData) {
    discord_ferris::log!(
        "OK",
        "Logged as {} session={}",
        ready.user.username,
        ready.session_id
    );
}
