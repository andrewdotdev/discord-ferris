use std::sync::Arc;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::http::Http;
use crate::models::gateway::{
    GatewayDispatch, GatewayDispatchEvents as GwEvt, GatewayOpcodes, GatewayReadyDispatchData,
};

#[derive(Clone)]
pub struct Context {
    pub http: Arc<Http>,
}
impl Context {
    pub fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
}

#[derive(Clone)]
pub struct Ctx {
    pub app: Arc<Context>,
    /// Raw dispatch (deserialize `d` on-demand).
    pub ev: Arc<GatewayDispatch<Value>>,
}

impl Ctx {
    /// Event name (`t`) if this is a dispatch.
    #[inline]
    pub fn event_name(&self) -> Option<GwEvt> {
        debug_assert_eq!(self.ev.op.clone() as u8, GatewayOpcodes::Dispatch as u8);
        Some(self.ev.t.clone())
    }

    /// Is the current dispatch of type `evt`?
    #[inline]
    pub fn is(&self, evt: GwEvt) -> bool {
        matches!(self.event_name(), Some(t) if t == evt)
    }

    /// Deserialize `d` to `T` without checking `t`.
    #[inline]
    pub fn data_as<T: DeserializeOwned>(&self) -> Option<T> {
        let val = self.ev.d.clone();
        match serde_json::from_value::<T>(val.clone()) {
            Ok(v) => Some(v),
            Err(e) => {
                // Basic error (line/col) and event name
                crate::log!(
                    "WARN",
                    "Failed to decode payload for {:?}: {}",
                    self.ev.t,
                    e
                );
                // Extra: scan for non-numeric flags so you know *where* to look
                debug_scan_flags("d", &val);
                None
            }
        }
    }

    /// Ensure `t == evt` and deserialize `d` to `T`.
    #[inline]
    pub fn event_as<T: DeserializeOwned>(&self, evt: GwEvt) -> Option<T> {
        if self.is(evt) {
            self.data_as::<T>()
        } else {
            None
        }
    }
}

/* -------------------------------------------------------------------------------------------------
Named accessors for all events
------------------------------------------------------------------------------------------------- */
macro_rules! event_accessors_named {
    ($($method:ident => $variant:ident),+ $(,)?) => {
        impl Ctx {
            $(
            #[inline]
            pub fn $method<T: DeserializeOwned>(&self) -> Option<T> {
                self.event_as::<T>(GwEvt::$variant)
            }
            )+
        }
    };
}

event_accessors_named! {
    as_application_command_permissions_update => ApplicationCommandPermissionsUpdate,
    as_auto_moderation_action_execution => AutoModerationActionExecution,
    as_auto_moderation_rule_create => AutoModerationRuleCreate,
    as_auto_moderation_rule_delete => AutoModerationRuleDelete,
    as_auto_moderation_rule_update => AutoModerationRuleUpdate,
    as_channel_create => ChannelCreate,
    as_channel_delete => ChannelDelete,
    as_channel_pins_update => ChannelPinsUpdate,
    as_channel_update => ChannelUpdate,
    as_entitlement_create => EntitlementCreate,
    as_entitlement_delete => EntitlementDelete,
    as_entitlement_update => EntitlementUpdate,
    as_guild_audit_log_entry_create => GuildAuditLogEntryCreate,
    as_guild_ban_add => GuildBanAdd,
    as_guild_ban_remove => GuildBanRemove,
    as_guild_create => GuildCreate,
    as_guild_delete => GuildDelete,
    as_guild_emojis_update => GuildEmojisUpdate,
    as_guild_integrations_update => GuildIntegrationsUpdate,
    as_guild_member_add => GuildMemberAdd,
    as_guild_member_remove => GuildMemberRemove,
    as_guild_members_chunk => GuildMembersChunk,
    as_guild_member_update => GuildMemberUpdate,
    as_guild_role_create => GuildRoleCreate,
    as_guild_role_delete => GuildRoleDelete,
    as_guild_role_update => GuildRoleUpdate,
    as_guild_scheduled_event_create => GuildScheduledEventCreate,
    as_guild_scheduled_event_delete => GuildScheduledEventDelete,
    as_guild_scheduled_event_update => GuildScheduledEventUpdate,
    as_guild_scheduled_event_user_add => GuildScheduledEventUserAdd,
    as_guild_scheduled_event_user_remove => GuildScheduledEventUserRemove,
    as_guild_soundboard_sound_create => GuildSoundboardSoundCreate,
    as_guild_soundboard_sound_delete => GuildSoundboardSoundDelete,
    as_guild_soundboard_sounds_update => GuildSoundboardSoundsUpdate,
    as_guild_soundboard_sound_update => GuildSoundboardSoundUpdate,
    as_soundboard_sounds => SoundboardSounds,
    as_guild_stickers_update => GuildStickersUpdate,
    as_guild_update => GuildUpdate,
    as_integration_create => IntegrationCreate,
    as_integration_delete => IntegrationDelete,
    as_integration_update => IntegrationUpdate,
    as_interaction_create => InteractionCreate,
    as_invite_create => InviteCreate,
    as_invite_delete => InviteDelete,
    as_message_create => MessageCreate,
    as_message_delete => MessageDelete,
    as_message_delete_bulk => MessageDeleteBulk,
    as_message_poll_vote_add => MessagePollVoteAdd,
    as_message_poll_vote_remove => MessagePollVoteRemove,
    as_message_reaction_add => MessageReactionAdd,
    as_message_reaction_remove => MessageReactionRemove,
    as_message_reaction_remove_all => MessageReactionRemoveAll,
    as_message_reaction_remove_emoji => MessageReactionRemoveEmoji,
    as_message_update => MessageUpdate,
    as_presence_update => PresenceUpdate,
    as_ready => Ready,
    as_resumed => Resumed,
    as_stage_instance_create => StageInstanceCreate,
    as_stage_instance_delete => StageInstanceDelete,
    as_stage_instance_update => StageInstanceUpdate,
    as_subscription_create => SubscriptionCreate,
    as_subscription_delete => SubscriptionDelete,
    as_subscription_update => SubscriptionUpdate,
    as_thread_create => ThreadCreate,
    as_thread_delete => ThreadDelete,
    as_thread_list_sync => ThreadListSync,
    as_thread_members_update => ThreadMembersUpdate,
    as_thread_member_update => ThreadMemberUpdate,
    as_thread_update => ThreadUpdate,
    as_typing_start => TypingStart,
    as_user_update => UserUpdate,
    as_voice_channel_effect_send => VoiceChannelEffectSend,
    as_voice_server_update => VoiceServerUpdate,
    as_voice_state_update => VoiceStateUpdate,
    as_webhooks_update => WebhooksUpdate,
}

/* -------------------------------------------------------------------------------------------------
Message sugar (works without your full message types)
------------------------------------------------------------------------------------------------- */
#[derive(Debug, Clone, Deserialize)]
struct MinimalMessage {
    pub id: String,
    pub channel_id: String,
    #[serde(default)]
    pub content: Option<String>,
}

impl Ctx {
    /// Owned string to avoid borrowing a temporary.
    #[inline]
    pub fn text(&self) -> Option<String> {
        self.as_message_create::<MinimalMessage>()
            .and_then(|m| m.content)
    }

    #[inline]
    pub fn channel_id(&self) -> Option<String> {
        self.as_message_create::<MinimalMessage>()
            .map(|m| m.channel_id)
    }

    pub async fn reply(&self, content: &str) -> anyhow::Result<()> {
        let m = self
            .as_message_create::<MinimalMessage>()
            .ok_or_else(|| anyhow::anyhow!("not a MessageCreate dispatch"))?;
        self.app
            .http
            .send_message(&m.channel_id, content, Some(&m.id), false)
            .await
    }

    pub async fn say(&self, channel_id: &str, content: &str) -> anyhow::Result<()> {
        self.app
            .http
            .send_message(channel_id, content, None, false)
            .await
    }

    #[inline]
    pub fn ready(&self) -> Option<GatewayReadyDispatchData> {
        self.as_ready::<GatewayReadyDispatchData>()
    }
}

fn debug_scan_flags(prefix: &str, v: &Value) {
    match v {
        Value::Object(map) => {
            for (k, val) in map {
                let path = if prefix.is_empty() {
                    k.to_string()
                } else {
                    format!("{}/{}", prefix, k)
                };

                // Heuristics: common flag field names we care about
                let looks_like_flags = k == "flags" || k.ends_with("_flags") || k == "public_flags";

                if looks_like_flags {
                    let is_num = val.is_u64() || val.is_i64();
                    if !is_num && !val.is_null() {
                        crate::log!("WARN", "Field '{}' is not numeric: {:?}", path, val);
                    }
                }
                debug_scan_flags(&path, val);
            }
        }
        Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                debug_scan_flags(&format!("{}/{}", prefix, i), val);
            }
        }
        _ => {}
    }
}
