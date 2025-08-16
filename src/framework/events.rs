//! Central event trait + typed dispatch.
//! Users implement `Events for Bot` in their own `src/events/*.rs`.

use async_trait::async_trait;
use serde_json as json;

use crate::framework::context::Ctx;
use crate::models::gateway::GatewayDispatchEvents as E;
use crate::models::gateway::*;

#[async_trait]
pub trait Events: Send + Sync {
    /// Fallback when decode fails; still tells which event happened.
    async fn on_any(&self, _ctx: Ctx, _which: E) {}

    // --- One method per Gateway event (all are no-op by default) ---
    async fn on_application_command_permissions_update(
        &self,
        _ctx: Ctx,
        _d: GatewayApplicationCommandPermissionsUpdateDispatchData,
    ) {
    }
    async fn on_auto_moderation_action_execution(
        &self,
        _ctx: Ctx,
        _d: GatewayAutoModerationActionExecutionDispatchData,
    ) {
    }
    async fn on_auto_moderation_rule_create(
        &self,
        _ctx: Ctx,
        _d: GatewayAutoModerationRuleCreateDispatchData,
    ) {
    }
    async fn on_auto_moderation_rule_delete(
        &self,
        _ctx: Ctx,
        _d: GatewayAutoModerationRuleDeleteDispatchData,
    ) {
    }
    async fn on_auto_moderation_rule_update(
        &self,
        _ctx: Ctx,
        _d: GatewayAutoModerationRuleUpdateDispatchData,
    ) {
    }
    async fn on_channel_create(&self, _ctx: Ctx, _d: GatewayChannelCreateDispatchData) {}
    async fn on_channel_delete(&self, _ctx: Ctx, _d: GatewayChannelDeleteDispatchData) {}
    async fn on_channel_pins_update(&self, _ctx: Ctx, _d: GatewayChannelPinsUpdateDispatchData) {}
    async fn on_channel_update(&self, _ctx: Ctx, _d: GatewayChannelUpdateDispatchData) {}
    async fn on_entitlement_create(&self, _ctx: Ctx, _d: GatewayEntitlementCreateDispatchData) {}
    async fn on_entitlement_delete(&self, _ctx: Ctx, _d: GatewayEntitlementDeleteDispatchData) {}
    async fn on_entitlement_update(&self, _ctx: Ctx, _d: GatewayEntitlementUpdateDispatchData) {}
    async fn on_guild_audit_log_entry_create(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildAuditLogEntryCreateDispatchData,
    ) {
    }
    async fn on_guild_ban_add(&self, _ctx: Ctx, _d: GatewayGuildBanAddDispatchData) {}
    async fn on_guild_ban_remove(&self, _ctx: Ctx, _d: GatewayGuildBanRemoveDispatchData) {}
    async fn on_guild_create(&self, _ctx: Ctx, _d: GatewayGuildCreateDispatchData) {}
    async fn on_guild_delete(&self, _ctx: Ctx, _d: GatewayGuildDeleteDispatchData) {}
    async fn on_guild_emojis_update(&self, _ctx: Ctx, _d: GatewayGuildEmojisUpdateDispatchData) {}
    async fn on_guild_integrations_update(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildIntegrationsUpdateDispatchData,
    ) {
    }
    async fn on_guild_member_add(&self, _ctx: Ctx, _d: GatewayGuildMemberAddDispatchData) {}
    async fn on_guild_member_remove(&self, _ctx: Ctx, _d: GatewayGuildMemberRemoveDispatchData) {}
    async fn on_guild_members_chunk(&self, _ctx: Ctx, _d: GatewayGuildMembersChunkDispatchData) {}
    async fn on_guild_member_update(&self, _ctx: Ctx, _d: GatewayGuildMemberUpdateDispatchData) {}
    async fn on_guild_role_create(&self, _ctx: Ctx, _d: GatewayGuildRoleCreateDispatchData) {}
    async fn on_guild_role_delete(&self, _ctx: Ctx, _d: GatewayGuildRoleDeleteDispatchData) {}
    async fn on_guild_role_update(&self, _ctx: Ctx, _d: GatewayGuildRoleUpdateDispatchData) {}
    async fn on_guild_scheduled_event_create(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildScheduledEventCreateDispatchData,
    ) {
    }
    async fn on_guild_scheduled_event_delete(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildScheduledEventDeleteDispatchData,
    ) {
    }
    async fn on_guild_scheduled_event_update(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildScheduledEventUpdateDispatchData,
    ) {
    }
    async fn on_guild_scheduled_event_user_add(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildScheduledEventUserAddDispatchData,
    ) {
    }
    async fn on_guild_scheduled_event_user_remove(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildScheduledEventUserRemoveDispatchData,
    ) {
    }
    async fn on_guild_soundboard_sound_create(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildSoundboardSoundCreateDispatchData,
    ) {
    }
    async fn on_guild_soundboard_sound_delete(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildSoundboardSoundDeleteDispatchData,
    ) {
    }
    async fn on_guild_soundboard_sounds_update(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildSoundboardSoundsUpdateDispatchData,
    ) {
    }
    async fn on_guild_soundboard_sound_update(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildSoundboardSoundUpdateDispatchData,
    ) {
    }
    async fn on_soundboard_sounds(&self, _ctx: Ctx, _d: GatewaySoundboardSoundsDispatchData) {}
    async fn on_guild_stickers_update(
        &self,
        _ctx: Ctx,
        _d: GatewayGuildStickersUpdateDispatchData,
    ) {
    }
    async fn on_guild_update(&self, _ctx: Ctx, _d: GatewayGuildUpdateDispatchData) {}
    async fn on_integration_create(&self, _ctx: Ctx, _d: GatewayIntegrationCreateDispatchData) {}
    async fn on_integration_delete(&self, _ctx: Ctx, _d: GatewayIntegrationDeleteDispatchData) {}
    async fn on_integration_update(&self, _ctx: Ctx, _d: GatewayIntegrationUpdateDispatchData) {}
    async fn on_interaction_create(&self, _ctx: Ctx, _d: GatewayInteractionCreateDispatchData) {}
    async fn on_invite_create(&self, _ctx: Ctx, _d: GatewayInviteCreateDispatchData) {}
    async fn on_invite_delete(&self, _ctx: Ctx, _d: GatewayInviteDeleteDispatchData) {}
    async fn on_message_create(&self, _ctx: Ctx, _d: GatewayMessageCreateDispatchData) {}
    async fn on_message_delete(&self, _ctx: Ctx, _d: GatewayMessageDeleteDispatchData) {}
    async fn on_message_delete_bulk(&self, _ctx: Ctx, _d: GatewayMessageDeleteBulkDispatchData) {}
    async fn on_message_poll_vote_add(&self, _ctx: Ctx, _d: GatewayMessagePollVoteDispatchData) {}
    async fn on_message_poll_vote_remove(&self, _ctx: Ctx, _d: GatewayMessagePollVoteDispatchData) {
    }
    async fn on_message_reaction_add(&self, _ctx: Ctx, _d: GatewayMessageReactionAddDispatchData) {}
    async fn on_message_reaction_remove(
        &self,
        _ctx: Ctx,
        _d: GatewayMessageReactionRemoveDispatchData,
    ) {
    }
    async fn on_message_reaction_remove_all(
        &self,
        _ctx: Ctx,
        _d: GatewayMessageReactionRemoveAllDispatchData,
    ) {
    }
    async fn on_message_reaction_remove_emoji(
        &self,
        _ctx: Ctx,
        _d: GatewayMessageReactionRemoveEmojiDispatchData,
    ) {
    }
    async fn on_message_update(&self, _ctx: Ctx, _d: GatewayMessageUpdateDispatchData) {}
    async fn on_presence_update(&self, _ctx: Ctx, _d: GatewayPresenceUpdateDispatchData) {}
    async fn on_ready(&self, _ctx: Ctx, _d: GatewayReadyDispatchData) {}
    async fn on_resumed(&self, _ctx: Ctx, _d: ()) {}
    async fn on_stage_instance_create(
        &self,
        _ctx: Ctx,
        _d: GatewayStageInstanceCreateDispatchData,
    ) {
    }
    async fn on_stage_instance_delete(
        &self,
        _ctx: Ctx,
        _d: GatewayStageInstanceDeleteDispatchData,
    ) {
    }
    async fn on_stage_instance_update(
        &self,
        _ctx: Ctx,
        _d: GatewayStageInstanceUpdateDispatchData,
    ) {
    }
    async fn on_subscription_create(&self, _ctx: Ctx, _d: GatewaySubscriptionCreateDispatchData) {}
    async fn on_subscription_delete(&self, _ctx: Ctx, _d: GatewaySubscriptionDeleteDispatchData) {}
    async fn on_subscription_update(&self, _ctx: Ctx, _d: GatewaySubscriptionUpdateDispatchData) {}
    async fn on_thread_create(&self, _ctx: Ctx, _d: GatewayThreadCreateDispatchData) {}
    async fn on_thread_delete(&self, _ctx: Ctx, _d: GatewayThreadDeleteDispatchData) {}
    async fn on_thread_list_sync(&self, _ctx: Ctx, _d: GatewayThreadListSyncDispatchData) {}
    async fn on_thread_members_update(
        &self,
        _ctx: Ctx,
        _d: GatewayThreadMembersUpdateDispatchData,
    ) {
    }
    async fn on_thread_member_update(&self, _ctx: Ctx, _d: GatewayThreadMemberUpdateDispatchData) {}
    async fn on_thread_update(&self, _ctx: Ctx, _d: GatewayThreadUpdateDispatchData) {}
    async fn on_typing_start(&self, _ctx: Ctx, _d: GatewayTypingStartDispatchData) {}
    async fn on_user_update(&self, _ctx: Ctx, _d: GatewayUserUpdateDispatchData) {}
    async fn on_voice_channel_effect_send(
        &self,
        _ctx: Ctx,
        _d: GatewayVoiceChannelEffectSendDispatchData,
    ) {
    }
    async fn on_voice_server_update(&self, _ctx: Ctx, _d: GatewayVoiceServerUpdateDispatchData) {}
    async fn on_voice_state_update(&self, _ctx: Ctx, _d: GatewayVoiceStateUpdateDispatchData) {}
    async fn on_webhooks_update(&self, _ctx: Ctx, _d: GatewayWebhooksUpdateDispatchData) {}

    /// Route a raw dispatch (with JSON `d`) to the typed method automatically.
    async fn dispatch(&self, ctx: Ctx, ev: GatewayDispatch<json::Value>) {
        // Keep event kind and payload separate so we can move the payload once.
        let which = ev.t;
        let d = ev.d;

        // Body-only macro. It does NOT emit a `match` arm.
        macro_rules! handle {
            ($ty:ty, $meth:ident, $variant:ident) => {{
                match json::from_value::<$ty>(d) {
                    Ok(parsed) => self.$meth(ctx, parsed).await,
                    Err(_) => self.on_any(ctx, E::$variant).await,
                }
            }};
        }

        match which {
            E::ApplicationCommandPermissionsUpdate => handle!(
                GatewayApplicationCommandPermissionsUpdateDispatchData,
                on_application_command_permissions_update,
                ApplicationCommandPermissionsUpdate
            ),
            E::AutoModerationActionExecution => handle!(
                GatewayAutoModerationActionExecutionDispatchData,
                on_auto_moderation_action_execution,
                AutoModerationActionExecution
            ),
            E::AutoModerationRuleCreate => handle!(
                GatewayAutoModerationRuleCreateDispatchData,
                on_auto_moderation_rule_create,
                AutoModerationRuleCreate
            ),
            E::AutoModerationRuleDelete => handle!(
                GatewayAutoModerationRuleDeleteDispatchData,
                on_auto_moderation_rule_delete,
                AutoModerationRuleDelete
            ),
            E::AutoModerationRuleUpdate => handle!(
                GatewayAutoModerationRuleUpdateDispatchData,
                on_auto_moderation_rule_update,
                AutoModerationRuleUpdate
            ),
            E::ChannelCreate => handle!(
                GatewayChannelCreateDispatchData,
                on_channel_create,
                ChannelCreate
            ),
            E::ChannelDelete => handle!(
                GatewayChannelDeleteDispatchData,
                on_channel_delete,
                ChannelDelete
            ),
            E::ChannelPinsUpdate => handle!(
                GatewayChannelPinsUpdateDispatchData,
                on_channel_pins_update,
                ChannelPinsUpdate
            ),
            E::ChannelUpdate => handle!(
                GatewayChannelUpdateDispatchData,
                on_channel_update,
                ChannelUpdate
            ),
            E::EntitlementCreate => handle!(
                GatewayEntitlementCreateDispatchData,
                on_entitlement_create,
                EntitlementCreate
            ),
            E::EntitlementDelete => handle!(
                GatewayEntitlementDeleteDispatchData,
                on_entitlement_delete,
                EntitlementDelete
            ),
            E::EntitlementUpdate => handle!(
                GatewayEntitlementUpdateDispatchData,
                on_entitlement_update,
                EntitlementUpdate
            ),
            E::GuildAuditLogEntryCreate => handle!(
                GatewayGuildAuditLogEntryCreateDispatchData,
                on_guild_audit_log_entry_create,
                GuildAuditLogEntryCreate
            ),
            E::GuildBanAdd => handle!(
                GatewayGuildBanAddDispatchData,
                on_guild_ban_add,
                GuildBanAdd
            ),
            E::GuildBanRemove => handle!(
                GatewayGuildBanRemoveDispatchData,
                on_guild_ban_remove,
                GuildBanRemove
            ),
            E::GuildCreate => handle!(GatewayGuildCreateDispatchData, on_guild_create, GuildCreate),
            E::GuildDelete => handle!(GatewayGuildDeleteDispatchData, on_guild_delete, GuildDelete),
            E::GuildEmojisUpdate => handle!(
                GatewayGuildEmojisUpdateDispatchData,
                on_guild_emojis_update,
                GuildEmojisUpdate
            ),
            E::GuildIntegrationsUpdate => handle!(
                GatewayGuildIntegrationsUpdateDispatchData,
                on_guild_integrations_update,
                GuildIntegrationsUpdate
            ),
            E::GuildMemberAdd => handle!(
                GatewayGuildMemberAddDispatchData,
                on_guild_member_add,
                GuildMemberAdd
            ),
            E::GuildMemberRemove => handle!(
                GatewayGuildMemberRemoveDispatchData,
                on_guild_member_remove,
                GuildMemberRemove
            ),
            E::GuildMembersChunk => handle!(
                GatewayGuildMembersChunkDispatchData,
                on_guild_members_chunk,
                GuildMembersChunk
            ),
            E::GuildMemberUpdate => handle!(
                GatewayGuildMemberUpdateDispatchData,
                on_guild_member_update,
                GuildMemberUpdate
            ),
            E::GuildRoleCreate => handle!(
                GatewayGuildRoleCreateDispatchData,
                on_guild_role_create,
                GuildRoleCreate
            ),
            E::GuildRoleDelete => handle!(
                GatewayGuildRoleDeleteDispatchData,
                on_guild_role_delete,
                GuildRoleDelete
            ),
            E::GuildRoleUpdate => handle!(
                GatewayGuildRoleUpdateDispatchData,
                on_guild_role_update,
                GuildRoleUpdate
            ),
            E::GuildScheduledEventCreate => handle!(
                GatewayGuildScheduledEventCreateDispatchData,
                on_guild_scheduled_event_create,
                GuildScheduledEventCreate
            ),
            E::GuildScheduledEventDelete => handle!(
                GatewayGuildScheduledEventDeleteDispatchData,
                on_guild_scheduled_event_delete,
                GuildScheduledEventDelete
            ),
            E::GuildScheduledEventUpdate => handle!(
                GatewayGuildScheduledEventUpdateDispatchData,
                on_guild_scheduled_event_update,
                GuildScheduledEventUpdate
            ),
            E::GuildScheduledEventUserAdd => handle!(
                GatewayGuildScheduledEventUserAddDispatchData,
                on_guild_scheduled_event_user_add,
                GuildScheduledEventUserAdd
            ),
            E::GuildScheduledEventUserRemove => handle!(
                GatewayGuildScheduledEventUserRemoveDispatchData,
                on_guild_scheduled_event_user_remove,
                GuildScheduledEventUserRemove
            ),
            E::GuildSoundboardSoundCreate => handle!(
                GatewayGuildSoundboardSoundCreateDispatchData,
                on_guild_soundboard_sound_create,
                GuildSoundboardSoundCreate
            ),
            E::GuildSoundboardSoundDelete => handle!(
                GatewayGuildSoundboardSoundDeleteDispatchData,
                on_guild_soundboard_sound_delete,
                GuildSoundboardSoundDelete
            ),
            E::GuildSoundboardSoundsUpdate => handle!(
                GatewayGuildSoundboardSoundsUpdateDispatchData,
                on_guild_soundboard_sounds_update,
                GuildSoundboardSoundsUpdate
            ),
            E::GuildSoundboardSoundUpdate => handle!(
                GatewayGuildSoundboardSoundUpdateDispatchData,
                on_guild_soundboard_sound_update,
                GuildSoundboardSoundUpdate
            ),
            E::SoundboardSounds => handle!(
                GatewaySoundboardSoundsDispatchData,
                on_soundboard_sounds,
                SoundboardSounds
            ),
            E::GuildStickersUpdate => handle!(
                GatewayGuildStickersUpdateDispatchData,
                on_guild_stickers_update,
                GuildStickersUpdate
            ),
            E::GuildUpdate => handle!(GatewayGuildUpdateDispatchData, on_guild_update, GuildUpdate),
            E::IntegrationCreate => handle!(
                GatewayIntegrationCreateDispatchData,
                on_integration_create,
                IntegrationCreate
            ),
            E::IntegrationDelete => handle!(
                GatewayIntegrationDeleteDispatchData,
                on_integration_delete,
                IntegrationDelete
            ),
            E::IntegrationUpdate => handle!(
                GatewayIntegrationUpdateDispatchData,
                on_integration_update,
                IntegrationUpdate
            ),
            E::InteractionCreate => handle!(
                GatewayInteractionCreateDispatchData,
                on_interaction_create,
                InteractionCreate
            ),
            E::InviteCreate => handle!(
                GatewayInviteCreateDispatchData,
                on_invite_create,
                InviteCreate
            ),
            E::InviteDelete => handle!(
                GatewayInviteDeleteDispatchData,
                on_invite_delete,
                InviteDelete
            ),
            E::MessageCreate => handle!(
                GatewayMessageCreateDispatchData,
                on_message_create,
                MessageCreate
            ),
            E::MessageDelete => handle!(
                GatewayMessageDeleteDispatchData,
                on_message_delete,
                MessageDelete
            ),
            E::MessageDeleteBulk => handle!(
                GatewayMessageDeleteBulkDispatchData,
                on_message_delete_bulk,
                MessageDeleteBulk
            ),
            E::MessagePollVoteAdd => handle!(
                GatewayMessagePollVoteDispatchData,
                on_message_poll_vote_add,
                MessagePollVoteAdd
            ),
            E::MessagePollVoteRemove => handle!(
                GatewayMessagePollVoteDispatchData,
                on_message_poll_vote_remove,
                MessagePollVoteRemove
            ),
            E::MessageReactionAdd => handle!(
                GatewayMessageReactionAddDispatchData,
                on_message_reaction_add,
                MessageReactionAdd
            ),
            E::MessageReactionRemove => handle!(
                GatewayMessageReactionRemoveDispatchData,
                on_message_reaction_remove,
                MessageReactionRemove
            ),
            E::MessageReactionRemoveAll => handle!(
                GatewayMessageReactionRemoveAllDispatchData,
                on_message_reaction_remove_all,
                MessageReactionRemoveAll
            ),
            E::MessageReactionRemoveEmoji => handle!(
                GatewayMessageReactionRemoveEmojiDispatchData,
                on_message_reaction_remove_emoji,
                MessageReactionRemoveEmoji
            ),
            E::MessageUpdate => handle!(
                GatewayMessageUpdateDispatchData,
                on_message_update,
                MessageUpdate
            ),
            E::PresenceUpdate => handle!(
                GatewayPresenceUpdateDispatchData,
                on_presence_update,
                PresenceUpdate
            ),
            E::Ready => handle!(GatewayReadyDispatchData, on_ready, Ready),
            // Special-case: `d` for Resumed may be `{}` or `null`; avoid serde to `()`.
            E::Resumed => self.on_resumed(ctx, ()).await,
            E::StageInstanceCreate => handle!(
                GatewayStageInstanceCreateDispatchData,
                on_stage_instance_create,
                StageInstanceCreate
            ),
            E::StageInstanceDelete => handle!(
                GatewayStageInstanceDeleteDispatchData,
                on_stage_instance_delete,
                StageInstanceDelete
            ),
            E::StageInstanceUpdate => handle!(
                GatewayStageInstanceUpdateDispatchData,
                on_stage_instance_update,
                StageInstanceUpdate
            ),
            E::SubscriptionCreate => handle!(
                GatewaySubscriptionCreateDispatchData,
                on_subscription_create,
                SubscriptionCreate
            ),
            E::SubscriptionDelete => handle!(
                GatewaySubscriptionDeleteDispatchData,
                on_subscription_delete,
                SubscriptionDelete
            ),
            E::SubscriptionUpdate => handle!(
                GatewaySubscriptionUpdateDispatchData,
                on_subscription_update,
                SubscriptionUpdate
            ),
            E::ThreadCreate => handle!(
                GatewayThreadCreateDispatchData,
                on_thread_create,
                ThreadCreate
            ),
            E::ThreadDelete => handle!(
                GatewayThreadDeleteDispatchData,
                on_thread_delete,
                ThreadDelete
            ),
            E::ThreadListSync => handle!(
                GatewayThreadListSyncDispatchData,
                on_thread_list_sync,
                ThreadListSync
            ),
            E::ThreadMembersUpdate => handle!(
                GatewayThreadMembersUpdateDispatchData,
                on_thread_members_update,
                ThreadMembersUpdate
            ),
            E::ThreadMemberUpdate => handle!(
                GatewayThreadMemberUpdateDispatchData,
                on_thread_member_update,
                ThreadMemberUpdate
            ),
            E::ThreadUpdate => handle!(
                GatewayThreadUpdateDispatchData,
                on_thread_update,
                ThreadUpdate
            ),
            E::TypingStart => handle!(GatewayTypingStartDispatchData, on_typing_start, TypingStart),
            E::UserUpdate => handle!(GatewayUserUpdateDispatchData, on_user_update, UserUpdate),
            E::VoiceChannelEffectSend => handle!(
                GatewayVoiceChannelEffectSendDispatchData,
                on_voice_channel_effect_send,
                VoiceChannelEffectSend
            ),
            E::VoiceServerUpdate => handle!(
                GatewayVoiceServerUpdateDispatchData,
                on_voice_server_update,
                VoiceServerUpdate
            ),
            E::VoiceStateUpdate => handle!(
                GatewayVoiceStateUpdateDispatchData,
                on_voice_state_update,
                VoiceStateUpdate
            ),
            E::WebhooksUpdate => handle!(
                GatewayWebhooksUpdateDispatchData,
                on_webhooks_update,
                WebhooksUpdate
            ),
        }
    }
}

/// Small prelude users can `use` in their event files.
pub mod prelude {
    pub use super::Events;
    pub use crate::framework::context::Ctx;
    pub use crate::models::gateway::*;
}
