use std::future::Future;
use std::sync::Arc;

use crate::discord_ferris::log::Log;
use crate::gateway::{self, Gateway};
use crate::http::Http;
use crate::log;

use crate::framework::context::{Context, Ctx};
use crate::framework::router::Router;
use crate::models::gateway::{GatewayDispatchEvents, GatewayIntents};

#[inline]
fn jitter(min: u64, max: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as u64;
    let mut state = nanos ^ 0x5DEECE66D;
    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    min + (state % (max - min + 1))
}

/// High-level client.
pub struct Client {
    token: String,
    intents: GatewayIntents,
    gateway: Option<Gateway>,
    #[allow(dead_code)]
    log: Log,
    router: Router,
    ctx: Ctx,
}

impl Client {
    pub fn new(token: impl Into<String>, intents: GatewayIntents) -> Self {
        let token = token.into();

        let mut router = Router::new();
        router.on_all(|c| async move {
            if let Some(t) = c.event_name() {
                crate::log!("EVT", "{:?}", t);
            }
        });

        let http = Arc::new(Http::new(&token));
        let inner_ctx = Arc::new(Context::new(Arc::clone(&http)));
        let ctx = Ctx::new(inner_ctx);

        Self {
            token,
            intents,
            gateway: None,
            log: Log {},
            router,
            ctx,
        }
    }

    pub fn on<T, F, Fut>(&mut self, kind: GatewayDispatchEvents, handler: F) -> &mut Self
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        F: Send + Sync + 'static + Fn(Ctx, T) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.router.on::<T, F, Fut>(kind, handler);
        self
    }

    pub fn once<T, F, Fut>(&mut self, kind: GatewayDispatchEvents, handler: F) -> &mut Self
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        F: Send + Sync + 'static + Fn(Ctx, T) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.router.once::<T, F, Fut>(kind, handler);
        self
    }

    pub fn on_all<F, Fut>(&mut self, handler: F) -> &mut Self
    where
        F: Send + Sync + 'static + Fn(Ctx) -> Fut,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.router.on_all(handler);
        self
    }

    pub async fn login(&mut self) -> anyhow::Result<()> {
        // Initial IDENTIFY.
        let gateway = gateway::ws::connect(&self.token, self.intents.clone()).await?;
        log!(
            "OK",
            "Client authenticated (session_id={})",
            gateway.session_id
        );
        self.gateway = Some(gateway);
        log!("CLI", "Client is running. use Ctrl+C to exit.");

        loop {
            let gw = self.gateway.as_mut().expect("gateway must exist");

            tokio::select! {
                maybe = gw.events_rx.recv() => {
                    match maybe {
                        Some(ev) => {
                            let ev = Arc::new(ev);
                            self.router.dispatch(&self.ctx, ev).await;
                        }
                        None => {
                            log!("WARN", "events_rx closed — attempting reconnection…");

                            let old_gw = self.gateway.take().expect("gateway must exist");
                            let session_id = old_gw.session_id.clone();
                            let resume_gateway_url = old_gw.resume_gateway_url.clone();
                            let last_seq = old_gw.last_seq_rx.borrow().clone();

                            let mut delay = std::time::Duration::from_secs(1);
                            use crate::gateway::ws::ResumeError;

                            loop {
                                match gateway::ws::resume(&self.token, &session_id, &resume_gateway_url, last_seq).await {
                                    Ok(new_gw) => {
                                        log!("OK", "RESUMED successfully");
                                        self.gateway = Some(new_gw);
                                        break;
                                    }
                                    Err(ResumeError::InvalidSession { resumable: false }) => {
                                        log!("WARN", "Session not resumable — fresh IDENTIFY…");
                                        let ms: u64 = jitter(1000, 5000);
                                        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;

                                        match gateway::ws::connect(&self.token, self.intents.clone()).await {
                                            Ok(new_gw) => {
                                                log!("OK", "Re-IDENTIFY successful (session_id={})", new_gw.session_id);
                                                self.gateway = Some(new_gw);
                                                break;
                                            }
                                            Err(e2) => {
                                                log!("ERR", "Re-connect failed: {e2}. Retrying in {:?}…", delay);
                                                tokio::time::sleep(delay).await;
                                                delay = std::cmp::min(delay * 2, std::time::Duration::from_secs(60));
                                            }
                                        }
                                    }
                                    Err(ResumeError::InvalidSession { resumable: true }) => {
                                        log!("WARN", "Session temporarily invalid; retrying RESUME…");
                                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                                    }
                                    Err(ResumeError::Transport(err)) => {
                                        log!("WARN", "RESUME transport error: {err}. Retrying in {:?}…", delay);
                                        tokio::time::sleep(delay).await;
                                        delay = std::cmp::min(delay * 2, std::time::Duration::from_secs(60));
                                    }
                                }
                            }
                        }
                    }
                }

                _ = tokio::signal::ctrl_c() => {
                    log!("CLI", "Keyboard Interrupt: Exiting");
                    if let Some(gw) = self.gateway.as_ref() {
                        use tokio_tungstenite::tungstenite::protocol::{CloseFrame, frame::coding::CloseCode};
                        use tokio_tungstenite::tungstenite::Message;
                        let _ = gw.writer_tx.send(Message::Close(Some(CloseFrame {
                            code: CloseCode::Normal,
                            reason: "shutdown".into(),
                        })));
                    }
                    break;
                }
            }
        }

        Ok(())
    }
}

// Sugar: generate chainable methods like `on_ready`, `once_ready`, etc.
// Each method returns &mut Self to allow `.on_xxx(...).once_yyy(...)` chaining.
macro_rules! impl_event_shortcuts {
    ($( $name:ident, $once:ident : $evt:ident => $ty:ty ),* $(,)?) => {
        impl Client {
            $(
            #[inline]
            pub fn $name<F, Fut>(&mut self, handler: F) -> &mut Self
            where
                F: Send + Sync + 'static + Fn(Ctx, $ty) -> Fut,
                Fut: std::future::Future<Output = ()> + Send + 'static,
            {
                self.on::<$ty, _, _>(GatewayDispatchEvents::$evt, handler)
            }

            #[inline]
            pub fn $once<F, Fut>(&mut self, handler: F) -> &mut Self
            where
                F: Send + Sync + 'static + Fn(Ctx, $ty) -> Fut,
                Fut: std::future::Future<Output = ()> + Send + 'static,
            {
                self.once::<$ty, _, _>(GatewayDispatchEvents::$evt, handler)
            }
            )*
        }
    }
}

// Full list so all shortcuts exist.
impl_event_shortcuts! {
    on_application_command_permissions_update, once_application_command_permissions_update:
        ApplicationCommandPermissionsUpdate => crate::models::gateway::GatewayApplicationCommandPermissionsUpdateDispatchData,

    on_auto_moderation_action_execution, once_auto_moderation_action_execution:
        AutoModerationActionExecution => crate::models::gateway::GatewayAutoModerationActionExecutionDispatchData,
    on_auto_moderation_rule_create, once_auto_moderation_rule_create:
        AutoModerationRuleCreate => crate::models::gateway::GatewayAutoModerationRuleCreateDispatchData,
    on_auto_moderation_rule_delete, once_auto_moderation_rule_delete:
        AutoModerationRuleDelete => crate::models::gateway::GatewayAutoModerationRuleDeleteDispatchData,
    on_auto_moderation_rule_update, once_auto_moderation_rule_update:
        AutoModerationRuleUpdate => crate::models::gateway::GatewayAutoModerationRuleUpdateDispatchData,

    on_channel_create, once_channel_create:
        ChannelCreate => crate::models::gateway::GatewayChannelCreateDispatchData,
    on_channel_delete, once_channel_delete:
        ChannelDelete => crate::models::gateway::GatewayChannelDeleteDispatchData,
    on_channel_pins_update, once_channel_pins_update:
        ChannelPinsUpdate => crate::models::gateway::GatewayChannelPinsUpdateDispatchData,
    on_channel_update, once_channel_update:
        ChannelUpdate => crate::models::gateway::GatewayChannelUpdateDispatchData,

    on_entitlement_create, once_entitlement_create:
        EntitlementCreate => crate::models::gateway::GatewayEntitlementCreateDispatchData,
    on_entitlement_delete, once_entitlement_delete:
        EntitlementDelete => crate::models::gateway::GatewayEntitlementDeleteDispatchData,
    on_entitlement_update, once_entitlement_update:
        EntitlementUpdate => crate::models::gateway::GatewayEntitlementUpdateDispatchData,

    on_guild_audit_log_entry_create, once_guild_audit_log_entry_create:
        GuildAuditLogEntryCreate => crate::models::gateway::GatewayGuildAuditLogEntryCreateDispatchData,
    on_guild_ban_add, once_guild_ban_add:
        GuildBanAdd => crate::models::gateway::GatewayGuildBanAddDispatchData,
    on_guild_ban_remove, once_guild_ban_remove:
        GuildBanRemove => crate::models::gateway::GatewayGuildBanRemoveDispatchData,
    on_guild_create, once_guild_create:
        GuildCreate => crate::models::gateway::GatewayGuildCreateDispatchData,
    on_guild_delete, once_guild_delete:
        GuildDelete => crate::models::gateway::GatewayGuildDeleteDispatchData,
    on_guild_emojis_update, once_guild_emojis_update:
        GuildEmojisUpdate => crate::models::gateway::GatewayGuildEmojisUpdateDispatchData,
    on_guild_integrations_update, once_guild_integrations_update:
        GuildIntegrationsUpdate => crate::models::gateway::GatewayGuildIntegrationsUpdateDispatchData,
    on_guild_member_add, once_guild_member_add:
        GuildMemberAdd => crate::models::gateway::GatewayGuildMemberAddDispatchData,
    on_guild_member_remove, once_guild_member_remove:
        GuildMemberRemove => crate::models::gateway::GatewayGuildMemberRemoveDispatchData,
    on_guild_members_chunk, once_guild_members_chunk:
        GuildMembersChunk => crate::models::gateway::GatewayGuildMembersChunkDispatchData,
    on_guild_member_update, once_guild_member_update:
        GuildMemberUpdate => crate::models::gateway::GatewayGuildMemberUpdateDispatchData,
    on_guild_role_create, once_guild_role_create:
        GuildRoleCreate => crate::models::gateway::GatewayGuildRoleCreateDispatchData,
    on_guild_role_delete, once_guild_role_delete:
        GuildRoleDelete => crate::models::gateway::GatewayGuildRoleDeleteDispatchData,
    on_guild_role_update, once_guild_role_update:
        GuildRoleUpdate => crate::models::gateway::GatewayGuildRoleUpdateDispatchData,
    on_guild_scheduled_event_create, once_guild_scheduled_event_create:
        GuildScheduledEventCreate => crate::models::gateway::GatewayGuildScheduledEventCreateDispatchData,
    on_guild_scheduled_event_delete, once_guild_scheduled_event_delete:
        GuildScheduledEventDelete => crate::models::gateway::GatewayGuildScheduledEventDeleteDispatchData,
    on_guild_scheduled_event_update, once_guild_scheduled_event_update:
        GuildScheduledEventUpdate => crate::models::gateway::GatewayGuildScheduledEventUpdateDispatchData,
    on_guild_scheduled_event_user_add, once_guild_scheduled_event_user_add:
        GuildScheduledEventUserAdd => crate::models::gateway::GatewayGuildScheduledEventUserAddDispatchData,
    on_guild_scheduled_event_user_remove, once_guild_scheduled_event_user_remove:
        GuildScheduledEventUserRemove => crate::models::gateway::GatewayGuildScheduledEventUserRemoveDispatchData,

    on_guild_soundboard_sound_create, once_guild_soundboard_sound_create:
        GuildSoundboardSoundCreate => crate::models::gateway::GatewayGuildSoundboardSoundCreateDispatchData,
    on_guild_soundboard_sound_delete, once_guild_soundboard_sound_delete:
        GuildSoundboardSoundDelete => crate::models::gateway::GatewayGuildSoundboardSoundDeleteDispatchData,
    on_guild_soundboard_sounds_update, once_guild_soundboard_sounds_update:
        GuildSoundboardSoundsUpdate => crate::models::gateway::GatewayGuildSoundboardSoundsUpdateDispatchData,
    on_guild_soundboard_sound_update, once_guild_soundboard_sound_update:
        GuildSoundboardSoundUpdate => crate::models::gateway::GatewayGuildSoundboardSoundUpdateDispatchData,
    on_soundboard_sounds, once_soundboard_sounds:
        SoundboardSounds => crate::models::gateway::GatewaySoundboardSoundsDispatchData,

    on_guild_stickers_update, once_guild_stickers_update:
        GuildStickersUpdate => crate::models::gateway::GatewayGuildStickersUpdateDispatchData,
    on_guild_update, once_guild_update:
        GuildUpdate => crate::models::gateway::GatewayGuildUpdateDispatchData,

    on_integration_create, once_integration_create:
        IntegrationCreate => crate::models::gateway::GatewayIntegrationCreateDispatchData,
    on_integration_delete, once_integration_delete:
        IntegrationDelete => crate::models::gateway::GatewayIntegrationDeleteDispatchData,
    on_integration_update, once_integration_update:
        IntegrationUpdate => crate::models::gateway::GatewayIntegrationUpdateDispatchData,

    on_interaction_create, once_interaction_create:
        InteractionCreate => crate::models::gateway::GatewayInteractionCreateDispatchData,

    on_invite_create, once_invite_create:
        InviteCreate => crate::models::gateway::GatewayInviteCreateDispatchData,
    on_invite_delete, once_invite_delete:
        InviteDelete => crate::models::gateway::GatewayInviteDeleteDispatchData,

    on_message_create, once_message_create:
        MessageCreate => crate::models::gateway::GatewayMessageCreateDispatchData,
    on_message_delete, once_message_delete:
        MessageDelete => crate::models::gateway::GatewayMessageDeleteDispatchData,
    on_message_delete_bulk, once_message_delete_bulk:
        MessageDeleteBulk => crate::models::gateway::GatewayMessageDeleteBulkDispatchData,
    on_message_poll_vote_add, once_message_poll_vote_add:
        MessagePollVoteAdd => crate::models::gateway::GatewayMessagePollVoteDispatchData,
    on_message_poll_vote_remove, once_message_poll_vote_remove:
        MessagePollVoteRemove => crate::models::gateway::GatewayMessagePollVoteDispatchData,
    on_message_reaction_add, once_message_reaction_add:
        MessageReactionAdd => crate::models::gateway::GatewayMessageReactionAddDispatchData,
    on_message_reaction_remove, once_message_reaction_remove:
        MessageReactionRemove => crate::models::gateway::GatewayMessageReactionRemoveDispatchData,
    on_message_reaction_remove_all, once_message_reaction_remove_all:
        MessageReactionRemoveAll => crate::models::gateway::GatewayMessageReactionRemoveAllDispatchData,
    on_message_reaction_remove_emoji, once_message_reaction_remove_emoji:
        MessageReactionRemoveEmoji => crate::models::gateway::GatewayMessageReactionRemoveEmojiDispatchData,
    on_message_update, once_message_update:
        MessageUpdate => crate::models::gateway::GatewayMessageUpdateDispatchData,

    on_presence_update, once_presence_update:
        PresenceUpdate => crate::models::gateway::GatewayPresenceUpdateDispatchData,

    on_ready, once_ready:
        Ready => crate::models::gateway::GatewayReadyDispatchData,
    on_resumed, once_resumed:
        Resumed => (),

    on_stage_instance_create, once_stage_instance_create:
        StageInstanceCreate => crate::models::gateway::GatewayStageInstanceCreateDispatchData,
    on_stage_instance_delete, once_stage_instance_delete:
        StageInstanceDelete => crate::models::gateway::GatewayStageInstanceDeleteDispatchData,
    on_stage_instance_update, once_stage_instance_update:
        StageInstanceUpdate => crate::models::gateway::GatewayStageInstanceUpdateDispatchData,

    on_subscription_create, once_subscription_create:
        SubscriptionCreate => crate::models::gateway::GatewaySubscriptionCreateDispatchData,
    on_subscription_delete, once_subscription_delete:
        SubscriptionDelete => crate::models::gateway::GatewaySubscriptionDeleteDispatchData,
    on_subscription_update, once_subscription_update:
        SubscriptionUpdate => crate::models::gateway::GatewaySubscriptionUpdateDispatchData,

    on_thread_create, once_thread_create:
        ThreadCreate => crate::models::gateway::GatewayThreadCreateDispatchData,
    on_thread_delete, once_thread_delete:
        ThreadDelete => crate::models::gateway::GatewayThreadDeleteDispatchData,
    on_thread_list_sync, once_thread_list_sync:
        ThreadListSync => crate::models::gateway::GatewayThreadListSyncDispatchData,
    on_thread_members_update, once_thread_members_update:
        ThreadMembersUpdate => crate::models::gateway::GatewayThreadMembersUpdateDispatchData,
    on_thread_member_update, once_thread_member_update:
        ThreadMemberUpdate => crate::models::gateway::GatewayThreadMemberUpdateDispatchData,
    on_thread_update, once_thread_update:
        ThreadUpdate => crate::models::gateway::GatewayThreadUpdateDispatchData,

    on_typing_start, once_typing_start:
        TypingStart => crate::models::gateway::GatewayTypingStartDispatchData,

    on_user_update, once_user_update:
        UserUpdate => crate::models::gateway::GatewayUserUpdateDispatchData,

    on_voice_channel_effect_send, once_voice_channel_effect_send:
        VoiceChannelEffectSend => crate::models::gateway::GatewayVoiceChannelEffectSendDispatchData,
    on_voice_server_update, once_voice_server_update:
        VoiceServerUpdate => crate::models::gateway::GatewayVoiceServerUpdateDispatchData,
    on_voice_state_update, once_voice_state_update:
        VoiceStateUpdate => crate::models::gateway::GatewayVoiceStateUpdateDispatchData,

    on_webhooks_update, once_webhooks_update:
        WebhooksUpdate => crate::models::gateway::GatewayWebhooksUpdateDispatchData,
}
