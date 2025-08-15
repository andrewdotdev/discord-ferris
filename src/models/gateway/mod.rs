//! Types extracted from https://discord.com/developers/docs/topics/gateway

pub use crate::models::payloads::common::*;
use serde::{Deserialize, Serialize};

use crate::models::payloads::{
    APIApplication, APIApplicationCommandPermission, APIAuditLogEntry, APIAutoModerationAction,
    APIAutoModerationRule, APIBaseGuild, APIBaseGuildMember, APIBaseMessage,
    APIBaseVoiceGuildMember, APIBaseVoiceState, APIChannel, APIEmoji, APIEntitlement,
    APIFlaggedGuildMember, APIGuild, APIGuildIntegration, APIGuildMember, APIGuildMemberAvatar,
    APIGuildMemberJoined, APIGuildMemberUser, APIGuildScheduledEvent, APIInteraction, APIRole,
    APISoundboardSound, APIStageInstance, APISticker, APISubscription, APIThreadChannel,
    APIThreadMember, APIUnavailableGuild, APIUser, APIVoiceState, AutoModerationRuleTriggerType,
    ChannelType, GatewayGuildMembersChunkPresence, GatewayPresenceUpdate, GatewayThreadListSync,
    GatewayThreadMembersUpdate as RawGatewayThreadMembersUpdate, InviteTargetType,
    PresenceUpdateStatus,
};
use crate::models::rest::ReactionType;

/// Simple replacement for the TS utility `_Nullable<T>`
pub type _Nullable<T> = Option<T>;

pub const GATEWAY_VERSION: &str = "10";

/**
 * @see {@link https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum GatewayOpcodes {
    /**
     * An event was dispatched
     */
    Dispatch = 0,
    /**
     * A bidirectional opcode to maintain an active gateway connection.
     * Fired periodically by the client, or fired by the gateway to request an immediate heartbeat from the client.
     */
    Heartbeat = 1,
    /**
     * Starts a new session during the initial handshake
     */
    Identify = 2,
    /**
     * Update the client's presence
     */
    PresenceUpdate = 3,
    /**
     * Used to join/leave or move between voice channels
     */
    VoiceStateUpdate = 4,
    /**
     * Resume a previous session that was disconnected
     */
    Resume = 6,
    /**
     * You should attempt to reconnect and resume immediately
     */
    Reconnect = 7,
    /**
     * Request information about offline guild members in a large guild
     */
    RequestGuildMembers = 8,
    /**
     * The session has been invalidated. You should reconnect and identify/resume accordingly
     */
    InvalidSession = 9,
    /**
     * Sent immediately after connecting, contains the `heartbeat_interval` to use
     */
    Hello = 10,
    /**
     * Sent in response to receiving a heartbeat to acknowledge that it has been received
     */
    HeartbeatAck = 11,
    /**
     * Request information about soundboard sounds in a set of guilds
     */
    RequestSoundboardSounds = 31,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum GatewayCloseCodes {
    /**
     * We're not sure what went wrong. Try reconnecting?
     */
    UnknownError = 4000,
    /**
     * You sent an invalid Gateway opcode or an invalid payload for an opcode. Don't do that!
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#payload-structure}
     */
    UnknownOpcode = 4001,
    /**
     * You sent an invalid payload to us. Don't do that!
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway#sending-events}
     */
    DecodeError = 4002,
    /**
     * You sent us a payload prior to identifying
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#identify}
     */
    NotAuthenticated = 4003,
    /**
     * The account token sent with your identify payload is incorrect
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#identify}
     */
    AuthenticationFailed = 4004,
    /**
     * You sent more than one identify payload. Don't do that!
     */
    AlreadyAuthenticated = 4005,
    /**
     * The sequence sent when resuming the session was invalid. Reconnect and start a new session
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#resume}
     */
    InvalidSeq = 4007,
    /**
     * Woah nelly! You're sending payloads to us too quickly. Slow it down! You will be disconnected on receiving this
     */
    RateLimited = 4008,
    /**
     * Your session timed out. Reconnect and start a new one
     */
    SessionTimedOut = 4009,
    /**
     * You sent us an invalid shard when identifying
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway#sharding}
     */
    InvalidShard = 4010,
    /**
     * The session would have handled too many guilds - you are required to shard your connection in order to connect
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway#sharding}
     */
    ShardingRequired = 4011,
    /**
     * You sent an invalid version for the gateway
     */
    InvalidAPIVersion = 4012,
    /**
     * You sent an invalid intent for a Gateway Intent. You may have incorrectly calculated the bitwise value
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway#gateway-intents}
     */
    InvalidIntents = 4013,
    /**
     * You sent a disallowed intent for a Gateway Intent. You may have tried to specify an intent that you have not
     * enabled or are not whitelisted for
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway#gateway-intents}
     * @see {@link https://discord.com/developers/docs/topics/gateway#privileged-intents}
     */
    DisallowedIntents = 4014,
}

use bitflags::bitflags;

bitflags! {
    /**
    * @see {@link https://discord.com/developers/docs/topics/gateway#list-of-intents}
    */
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct GatewayIntents: u64 {
        const GUILDS                        = 1 << 0;
        const GUILD_MEMBERS                 = 1 << 1;
        const GUILD_MODERATION              = 1 << 2;
        const GUILD_EXPRESSIONS             = 1 << 3;
        const GUILD_INTEGRATIONS            = 1 << 4;
        const GUILD_WEBHOOKS                = 1 << 5;
        const GUILD_INVITES                 = 1 << 6;
        const GUILD_VOICE_STATES            = 1 << 7;
        const GUILD_PRESENCES               = 1 << 8;
        const GUILD_MESSAGES                = 1 << 9;
        const GUILD_MESSAGE_REACTIONS       = 1 << 10;
        const GUILD_MESSAGE_TYPING          = 1 << 11;
        const DIRECT_MESSAGES               = 1 << 12;
        const DIRECT_MESSAGE_REACTIONS      = 1 << 13;
        const DIRECT_MESSAGE_TYPING         = 1 << 14;
        const MESSAGE_CONTENT               = 1 << 15;
        const GUILD_SCHEDULED_EVENTS        = 1 << 16;
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;
        const AUTO_MODERATION_EXECUTION     = 1 << 21;
        const GUILD_MESSAGE_POLLS           = 1 << 24;
        const DIRECT_MESSAGE_POLLS          = 1 << 25;
    }
}

impl GatewayIntents {
    pub const fn minimal() -> Self {
        Self::GUILDS.union(Self::GUILD_MESSAGES)
    }

    pub const fn recommended() -> Self {
        Self::minimal().union(Self::GUILD_MESSAGE_REACTIONS)
    }

    pub const fn privileged() -> Self {
        Self::GUILD_MEMBERS
            .union(Self::GUILD_PRESENCES)
            .union(Self::MESSAGE_CONTENT)
    }

    pub fn non_privileged() -> Self {
        Self::all() - Self::privileged()
    }
}

/// A creation of mine lol
pub type GatewayDispatch<D = serde_json::Value> = _DataPayload<GatewayDispatchEvents, D>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#receive-events}
 */
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GatewayDispatchEvents {
    ApplicationCommandPermissionsUpdate,
    AutoModerationActionExecution,
    AutoModerationRuleCreate,
    AutoModerationRuleDelete,
    AutoModerationRuleUpdate,
    ChannelCreate,
    ChannelDelete,
    ChannelPinsUpdate,
    ChannelUpdate,
    EntitlementCreate,
    EntitlementDelete,
    EntitlementUpdate,
    GuildAuditLogEntryCreate,
    GuildBanAdd,
    GuildBanRemove,
    GuildCreate,
    GuildDelete,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMembersChunk,
    GuildMemberUpdate,
    GuildRoleCreate,
    GuildRoleDelete,
    GuildRoleUpdate,
    GuildScheduledEventCreate,
    GuildScheduledEventDelete,
    GuildScheduledEventUpdate,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    GuildSoundboardSoundCreate,
    GuildSoundboardSoundDelete,
    GuildSoundboardSoundsUpdate,
    GuildSoundboardSoundUpdate,
    SoundboardSounds,
    GuildStickersUpdate,
    GuildUpdate,
    IntegrationCreate,
    IntegrationDelete,
    IntegrationUpdate,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MessageCreate,
    MessageDelete,
    MessageDeleteBulk,
    MessagePollVoteAdd,
    MessagePollVoteRemove,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    MessageUpdate,
    PresenceUpdate,
    Ready,
    Resumed,
    StageInstanceCreate,
    StageInstanceDelete,
    StageInstanceUpdate,
    SubscriptionCreate,
    SubscriptionDelete,
    SubscriptionUpdate,
    ThreadCreate,
    ThreadDelete,
    ThreadListSync,
    ThreadMembersUpdate,
    ThreadMemberUpdate,
    ThreadUpdate,
    TypingStart,
    UserUpdate,
    VoiceChannelEffectSend,
    VoiceServerUpdate,
    VoiceStateUpdate,
    WebhooksUpdate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GatewaySendPayload {
    GatewayHeartbeat(GatewayHeartbeat),
    GatewayIdentify(GatewayIdentify),
    GatewayRequestGuildMembers(GatewayRequestGuildMembers),
    GatewayRequestSoundboardSounds(GatewayRequestSoundboardSounds),
    GatewayResume(GatewayResume),
    GatewayUpdatePresence(GatewayUpdatePresence),
    GatewayVoiceStateUpdate(GatewayVoiceStateUpdate),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GatewayReceivePayload {
    GatewayDispatchPayload(GatewayDispatchPayload),
    GatewayHeartbeatAck(GatewayHeartbeatAck),
    GatewayHeartbeatRequest(GatewayHeartbeatRequest),
    GatewayHello(GatewayHello),
    GatewayInvalidSession(GatewayInvalidSession),
    GatewayReconnect(GatewayReconnect),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GatewayDispatchPayload {
    GatewayApplicationCommandPermissionsUpdateDispatch(
        GatewayApplicationCommandPermissionsUpdateDispatch,
    ),
    GatewayAutoModerationActionExecutionDispatch(GatewayAutoModerationActionExecutionDispatch),
    GatewayAutoModerationRuleCreateDispatch(GatewayAutoModerationRuleCreateDispatch),
    GatewayAutoModerationRuleDeleteDispatch(GatewayAutoModerationRuleDeleteDispatch),
    GatewayAutoModerationRuleModifyDispatch(GatewayAutoModerationRuleModifyDispatch),
    GatewayChannelModifyDispatch(GatewayChannelModifyDispatch),
    GatewayChannelPinsUpdateDispatch(GatewayChannelPinsUpdateDispatch),
    GatewayEntitlementModifyDispatch(GatewayEntitlementModifyDispatch),
    GatewayGuildAuditLogEntryCreateDispatch(GatewayGuildAuditLogEntryCreateDispatch),
    GatewayGuildBanModifyDispatch(GatewayGuildBanModifyDispatch),
    GatewayGuildCreateDispatch(GatewayGuildCreateDispatch),
    GatewayGuildDeleteDispatch(GatewayGuildDeleteDispatch),
    GatewayGuildEmojisUpdateDispatch(GatewayGuildEmojisUpdateDispatch),
    GatewayGuildIntegrationsUpdateDispatch(GatewayGuildIntegrationsUpdateDispatch),
    GatewayGuildMemberAddDispatch(GatewayGuildMemberAddDispatch),
    GatewayGuildMemberRemoveDispatch(GatewayGuildMemberRemoveDispatch),
    GatewayGuildMembersChunkDispatch(GatewayGuildMembersChunkDispatch),
    GatewayGuildMemberUpdateDispatch(GatewayGuildMemberUpdateDispatch),
    GatewayGuildModifyDispatch(GatewayGuildModifyDispatch),
    GatewayGuildRoleDeleteDispatch(GatewayGuildRoleDeleteDispatch),
    GatewayGuildRoleModifyDispatch(GatewayGuildRoleModifyDispatch),
    GatewayGuildScheduledEventCreateDispatch(GatewayGuildScheduledEventCreateDispatch),
    GatewayGuildScheduledEventDeleteDispatch(GatewayGuildScheduledEventDeleteDispatch),
    GatewayGuildScheduledEventUpdateDispatch(GatewayGuildScheduledEventUpdateDispatch),
    GatewayGuildScheduledEventUserAddDispatch(GatewayGuildScheduledEventUserAddDispatch),
    GatewayGuildScheduledEventUserRemoveDispatch(GatewayGuildScheduledEventUserRemoveDispatch),
    GatewayGuildSoundboardSoundCreateDispatch(GatewayGuildSoundboardSoundCreateDispatch),
    GatewayGuildSoundboardSoundDeleteDispatch(GatewayGuildSoundboardSoundDeleteDispatch),
    GatewayGuildSoundboardSoundsUpdateDispatch(GatewayGuildSoundboardSoundsUpdateDispatch),
    GatewayGuildSoundboardSoundUpdateDispatch(GatewayGuildSoundboardSoundUpdateDispatch),
    GatewayGuildStickersUpdateDispatch(GatewayGuildStickersUpdateDispatch),
    GatewayIntegrationCreateDispatch(GatewayIntegrationCreateDispatch),
    GatewayIntegrationDeleteDispatch(GatewayIntegrationDeleteDispatch),
    GatewayIntegrationUpdateDispatch(GatewayIntegrationUpdateDispatch),
    GatewayInteractionCreateDispatch(GatewayInteractionCreateDispatch),
    GatewayInviteCreateDispatch(GatewayInviteCreateDispatch),
    GatewayInviteDeleteDispatch(GatewayInviteDeleteDispatch),
    GatewayMessageCreateDispatch(GatewayMessageCreateDispatch),
    GatewayMessageDeleteBulkDispatch(GatewayMessageDeleteBulkDispatch),
    GatewayMessageDeleteDispatch(GatewayMessageDeleteDispatch),
    GatewayMessagePollVoteAddDispatch(GatewayMessagePollVoteAddDispatch),
    GatewayMessagePollVoteRemoveDispatch(GatewayMessagePollVoteRemoveDispatch),
    GatewayMessageReactionAddDispatch(GatewayMessageReactionAddDispatch),
    GatewayMessageReactionRemoveAllDispatch(GatewayMessageReactionRemoveAllDispatch),
    GatewayMessageReactionRemoveDispatch(GatewayMessageReactionRemoveDispatch),
    GatewayMessageReactionRemoveEmojiDispatch(GatewayMessageReactionRemoveEmojiDispatch),
    GatewayMessageUpdateDispatch(GatewayMessageUpdateDispatch),
    GatewayPresenceUpdateDispatch(GatewayPresenceUpdateDispatch),
    GatewayReadyDispatch(GatewayReadyDispatch),
    GatewayResumedDispatch(GatewayResumedDispatch),
    GatewaySoundboardSoundsDispatch(GatewaySoundboardSoundsDispatch),
    GatewayStageInstanceCreateDispatch(GatewayStageInstanceCreateDispatch),
    GatewayStageInstanceDeleteDispatch(GatewayStageInstanceDeleteDispatch),
    GatewayStageInstanceUpdateDispatch(GatewayStageInstanceUpdateDispatch),
    GatewaySubscriptionModifyDispatch(GatewaySubscriptionModifyDispatch),
    GatewayThreadCreateDispatch(GatewayThreadCreateDispatch),
    GatewayThreadDeleteDispatch(GatewayThreadDeleteDispatch),
    GatewayThreadListSyncDispatch(GatewayThreadListSyncDispatch),
    GatewayThreadMembersUpdateDispatch(GatewayThreadMembersUpdateDispatch),
    GatewayThreadMemberUpdateDispatch(GatewayThreadMemberUpdateDispatch),
    GatewayThreadUpdateDispatch(GatewayThreadUpdateDispatch),
    GatewayTypingStartDispatch(GatewayTypingStartDispatch),
    GatewayUserUpdateDispatch(GatewayUserUpdateDispatch),
    GatewayVoiceChannelEffectSendDispatch(GatewayVoiceChannelEffectSendDispatch),
    GatewayVoiceServerUpdateDispatch(GatewayVoiceServerUpdateDispatch),
    GatewayVoiceStateUpdateDispatch(GatewayVoiceStateUpdateDispatch),
    GatewayWebhooksUpdateDispatch(GatewayWebhooksUpdateDispatch),
}

// #region Dispatch Payloads

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#hello}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayHello {
    pub op: GatewayOpcodes,
    pub d: GatewayHelloData,
    pub t: Option<serde_json::Value>,
    pub s: Option<serde_json::Value>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#hello}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayHelloData {
    /**
     * The interval (in milliseconds) the client should heartbeat with
     */
    pub heartbeat_interval: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway#sending-heartbeats}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayHeartbeatRequest {
    pub op: GatewayOpcodes,
    pub d: (),
    pub t: Option<serde_json::Value>,
    pub s: Option<serde_json::Value>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#heartbeat}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayHeartbeatAck {
    pub op: GatewayOpcodes,
    pub d: (),
    pub t: Option<serde_json::Value>,
    pub s: Option<serde_json::Value>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#invalid-session}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayInvalidSession {
    pub op: GatewayOpcodes,
    pub d: GatewayInvalidSessionData,
    pub t: Option<serde_json::Value>,
    pub s: Option<serde_json::Value>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#invalid-session}
 */
pub type GatewayInvalidSessionData = bool;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#reconnect}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayReconnect {
    pub op: GatewayOpcodes,
    pub d: (),
    pub t: Option<serde_json::Value>,
    pub s: Option<serde_json::Value>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#ready}
 */
pub type GatewayReadyDispatch = _DataPayload<GatewayDispatchEvents, GatewayReadyDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#ready}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayReadyDispatchData {
    /**
     * Gateway version
     *
     * @see {@link https://discord.com/developers/docs/reference#api-versioning}
     */
    pub v: i64,
    /**
     * Information about the user including email
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub user: APIUser,
    /**
     * The guilds the user is in
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#unavailable-guild-object}
     */
    pub guilds: Vec<APIUnavailableGuild>,
    /**
     * Used for resuming connections
     */
    pub session_id: String,
    /**
     * Gateway url for resuming connections
     */
    pub resume_gateway_url: String,
    /**
     * The shard information associated with this session, if sent when identifying
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway#sharding}
     */
    pub shard: Option<(i64, i64)>,
    /**
     * Contains `id` and `flags`
     *
     * @see {@link https://discord.com/developers/docs/resources/application#application-object}
     */
    pub application: GatewayReadyApplication,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayReadyApplication {
    pub id: String,
    pub flags: u64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#resumed}
 */
pub type GatewayResumedDispatch = _DataPayload<GatewayDispatchEvents, ()>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-update}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-delete}
 */
pub type GatewayAutoModerationRuleModifyDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayAutoModerationRuleModifyDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-update}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-delete}
 */
pub type GatewayAutoModerationRuleModifyDispatchData = APIAutoModerationRule;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-create}
 */
pub type GatewayAutoModerationRuleCreateDispatch = GatewayAutoModerationRuleModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-create}
 */
pub type GatewayAutoModerationRuleCreateDispatchData = GatewayAutoModerationRuleModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-update}
 */
pub type GatewayAutoModerationRuleUpdateDispatch = GatewayAutoModerationRuleModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-update}
 */
pub type GatewayAutoModerationRuleUpdateDispatchData = GatewayAutoModerationRuleModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-delete}
 */
pub type GatewayAutoModerationRuleDeleteDispatch = GatewayAutoModerationRuleModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-delete}
 */
pub type GatewayAutoModerationRuleDeleteDispatchData = GatewayAutoModerationRuleModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-action-execution}
 */
pub type GatewayAutoModerationActionExecutionDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayAutoModerationActionExecutionDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#auto-moderation-action-execution}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayAutoModerationActionExecutionDispatchData {
    /**
     * The id of the guild in which action was executed
     */
    pub guild_id: String,
    /**
     * The action which was executed
     */
    pub action: APIAutoModerationAction,
    /**
     * The id of the rule which action belongs to
     */
    pub rule_id: String,
    /**
     * The trigger type of rule which was triggered
     */
    pub rule_trigger_type: AutoModerationRuleTriggerType,
    /**
     * The id of the user which generated the content which triggered the rule
     */
    pub user_id: String,
    /**
     * The id of the channel in which user content was posted
     */
    pub channel_id: Option<String>,
    /**
     * The id of any user message which content belongs to
     *
     * This field will not be present if message was blocked by AutoMod or content was not part of any message
     */
    pub message_id: Option<String>,
    /**
     * The id of any system auto moderation messages posted as a result of this action
     *
     * This field will not be present if this event does not correspond to an action with type {@link AutoModerationActionType.SendAlertMessage}
     */
    pub alert_system_message_id: Option<String>,
    /**
     * The user generated text content
     *
     * `MESSAGE_CONTENT` (`1 << 15`) gateway intent is required to receive non-empty values from this field
     */
    pub content: String,
    /**
     * The word or phrase configured in the rule that triggered the rule
     */
    pub matched_keyword: Option<String>,
    /**
     * The substring in content that triggered the rule
     *
     * `MESSAGE_CONTENT` (`1 << 15`) gateway intent is required to receive non-empty values from this field
     */
    pub matched_content: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#application-command-permissions-update}
 */
pub type GatewayApplicationCommandPermissionsUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayApplicationCommandPermissionsUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#application-command-permissions-update}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayApplicationCommandPermissionsUpdateDispatchData {
    /**
     * ID of the command or the application ID
     */
    pub id: String,
    /**
     * ID of the application the command belongs to
     */
    pub application_id: String,
    /**
     * ID of the guild
     */
    pub guild_id: String,
    /**
     * Permissions for the command in the guild, max of 100
     */
    pub permissions: Vec<APIApplicationCommandPermission>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-update}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-delete}
 */
pub type GatewaySubscriptionModifyDispatch =
    _DataPayload<GatewayDispatchEvents, GatewaySubscriptionModifyDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-update}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-delete}
 */
pub type GatewaySubscriptionModifyDispatchData = APISubscription;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-create}
 */
pub type GatewaySubscriptionCreateDispatch = GatewaySubscriptionModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-create}
 */
pub type GatewaySubscriptionCreateDispatchData = GatewaySubscriptionModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-update}
 */
pub type GatewaySubscriptionUpdateDispatch = GatewaySubscriptionModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-update}
 */
pub type GatewaySubscriptionUpdateDispatchData = GatewaySubscriptionModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-delete}
 */
pub type GatewaySubscriptionDeleteDispatch = GatewaySubscriptionModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#subscription-delete}
 */
pub type GatewaySubscriptionDeleteDispatchData = GatewaySubscriptionModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-update}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-delete}
 */
pub type GatewayChannelModifyDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayChannelModifyDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-update}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-delete}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayChannelModifyDispatchData {
    pub guild_id: String,
    #[serde(flatten)]
    pub channel: APIChannel,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-create}
 */
pub type GatewayChannelCreateDispatch = GatewayChannelModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-create}
 */
pub type GatewayChannelCreateDispatchData = GatewayChannelModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-update}
 */
pub type GatewayChannelUpdateDispatch = GatewayChannelModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-update}
 */
pub type GatewayChannelUpdateDispatchData = GatewayChannelModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-delete}
 */
pub type GatewayChannelDeleteDispatch = GatewayChannelModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-delete}
 */
pub type GatewayChannelDeleteDispatchData = GatewayChannelModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-pins-update}
 */
pub type GatewayChannelPinsUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayChannelPinsUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#channel-pins-update}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayChannelPinsUpdateDispatchData {
    /**
     * The id of the guild
     */
    pub guild_id: Option<String>,
    /**
     * The id of the channel
     */
    pub channel_id: String,
    /**
     * The time at which the most recent pinned message was pinned
     */
    pub last_pin_timestamp: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-update}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-delete}
 */
pub type GatewayEntitlementModifyDispatchData = APIEntitlement;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-update}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-delete}
 */
pub type GatewayEntitlementModifyDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayEntitlementModifyDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-create}
 */
pub type GatewayEntitlementCreateDispatchData = GatewayEntitlementModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-create}
 */
pub type GatewayEntitlementCreateDispatch = GatewayEntitlementModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-update}
 */
pub type GatewayEntitlementUpdateDispatchData = GatewayEntitlementModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-update}
 */
pub type GatewayEntitlementUpdateDispatch = GatewayEntitlementModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-delete}
 */
pub type GatewayEntitlementDeleteDispatchData = GatewayEntitlementModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#entitlement-delete}
 */
pub type GatewayEntitlementDeleteDispatch = GatewayEntitlementModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-update}
 */
pub type GatewayGuildModifyDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildModifyDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-update}
 */
pub type GatewayGuildModifyDispatchData = APIGuild;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-create}
 */
pub type GatewayGuildCreateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildCreateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-create-guild-create-extra-fields}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayGuildCreateDispatchData {
    /**
     * When this guild was joined at
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     */
    pub joined_at: String,
    /**
     * `true` if this is considered a large guild
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     */
    pub large: bool,
    /**
     * `true` if this guild is unavailable due to an outage
     */
    pub unavailable: Option<bool>,
    /**
     * Total number of members in this guild
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     */
    pub member_count: i64,
    /**
     * States of members currently in voice channels; lacks the `guild_id` key
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     *
     * @see {@link https://discord.com/developers/docs/resources/voice#voice-state-object}
     */
    pub voice_states: Vec<APIBaseVoiceState>,
    /**
     * Users in the guild
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
     */
    pub members: Vec<APIGuildMember>,
    /**
     * Channels in the guild
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object}
     */
    pub channels: Vec<APIChannel>,
    /**
     * Threads in the guild
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object}
     */
    pub threads: Vec<APIChannel>,
    /**
     * Presences of the members in the guild, will only include non-offline members if the size is greater than `large_threshold`
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#presence-update}
     */
    pub presences: Vec<GatewayPresenceUpdate>,
    /**
     * The stage instances in the guild
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     *
     * @see {@link https://discord.com/developers/docs/resources/stage-instance#stage-instance-object-stage-instance-structure}
     */
    pub stage_instances: Vec<APIStageInstance>,
    /**
     * The scheduled events in the guild
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     *
     * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object}
     */
    pub guild_scheduled_events: Vec<APIGuildScheduledEvent>,
    /**
     * The soundboard sounds in the guild
     *
     * **This field is only sent within the {@link https://discord.com/developers/docs/topics/gateway-events#guild-create | GUILD_CREATE} event**
     *
     * @see {@link https://discord.com/developers/docs/resources/soundboard#soundboard-sound-object}
     */
    pub soundboard_sounds: Vec<APISoundboardSound>,
    #[serde(flatten)]
    pub guild: APIGuild,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-update}
 */
pub type GatewayGuildUpdateDispatch = GatewayGuildModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-update}
 */
pub type GatewayGuildUpdateDispatchData = GatewayGuildModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-delete}
 */
pub type GatewayGuildDeleteDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildDeleteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-delete}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayGuildDeleteDispatchData {
    /**
     * `true` if this guild is unavailable due to an outage
     *
     * If the field is not set, the user was removed from the guild.
     */
    pub unavailable: Option<bool>,
    #[serde(flatten)]
    pub guild: APIBaseGuild,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-ban-add}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-ban-remove}
 */
pub type GatewayGuildBanModifyDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildBanModifyDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-ban-add}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-ban-remove}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayGuildBanModifyDispatchData {
    /**
     * ID of the guild
     */
    pub guild_id: String,
    /**
     * The banned user
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub user: APIUser,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-ban-add}
 */
pub type GatewayGuildBanAddDispatch = GatewayGuildBanModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-ban-add}
 */
pub type GatewayGuildBanAddDispatchData = GatewayGuildBanModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-ban-remove}
 */
pub type GatewayGuildBanRemoveDispatch = GatewayGuildBanModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-ban-remove}
 */
pub type GatewayGuildBanRemoveDispatchData = GatewayGuildBanModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-emojis-update}
 */
pub type GatewayGuildEmojisUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildEmojisUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-emojis-update}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayGuildEmojisUpdateDispatchData {
    /**
     * ID of the guild
     */
    pub guild_id: String,
    /**
     * Array of emojis
     *
     * @see {@link https://discord.com/developers/docs/resources/emoji#emoji-object}
     */
    pub emojis: Vec<APIEmoji>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-stickers-update}
 */
pub type GatewayGuildStickersUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildStickersUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-stickers-update}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayGuildStickersUpdateDispatchData {
    /**
     * ID of the guild
     */
    pub guild_id: String,
    /**
     * Array of stickers
     *
     * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-object}
     */
    pub stickers: Vec<APISticker>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-integrations-update}
 */
pub type GatewayGuildIntegrationsUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildIntegrationsUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-integrations-update}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayGuildIntegrationsUpdateDispatchData {
    /**
     * ID of the guild whose integrations were updated
     */
    pub guild_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-member-add}
 */
pub type GatewayGuildMemberAddDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildMemberAddDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-member-add}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayGuildMemberAddDispatchData {
    /**
     * The id of the guild
     */
    pub guild_id: String,
    #[serde(flatten)]
    pub member: APIGuildMember,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-member-remove}
 */
pub type GatewayGuildMemberRemoveDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildMemberRemoveDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-member-remove}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayGuildMemberRemoveDispatchData {
    /**
     * The id of the guild
     */
    pub guild_id: String,
    /**
     * The user who was removed
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub user: APIUser,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-member-update}
 */
pub type GatewayGuildMemberUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildMemberUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-member-update}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayGuildMemberUpdateDispatchData {
    /**
     * The id of the guild
     */
    pub guild_id: String,
    #[serde(flatten)]
    pub joined: Option<APIGuildMemberJoined>,
    #[serde(flatten)]
    pub base: APIBaseGuildMember,
    #[serde(flatten)]
    pub voice: Option<APIBaseVoiceGuildMember>,
    #[serde(flatten)]
    pub flagged: Option<APIFlaggedGuildMember>,
    #[serde(flatten)]
    pub avatar: APIGuildMemberAvatar,
    #[serde(flatten)]
    pub user: APIGuildMemberUser,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-members-chunk}
 */
pub type GatewayGuildMembersChunkDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildMembersChunkDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-members-chunk}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayGuildMembersChunkDispatchData {
    /**
     * The id of the guild
     */
    pub guild_id: String,
    /**
     * Set of guild members
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
     */
    pub members: Vec<APIGuildMember>,
    /**
     * The chunk index in the expected chunks for this response (`0 <= chunk_index < chunk_count`)
     */
    pub chunk_index: i64,
    /**
     * The total number of expected chunks for this response
     */
    pub chunk_count: i64,
    /**
     * If passing an invalid id to `REQUEST_GUILD_MEMBERS`, it will be returned here
     */
    pub not_found: Option<Vec<serde_json::Value>>,
    /**
     * If passing true to `REQUEST_GUILD_MEMBERS`, presences of the returned members will be here
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#update-presence}
     */
    pub presences: Option<Vec<GatewayGuildMembersChunkPresence>>,
    /**
     * The nonce used in the Guild Members Request
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#request-guild-members}
     */
    pub nonce: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-role-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-role-update}
 */
pub type GatewayGuildRoleModifyDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildRoleModifyDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-role-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-role-update}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayGuildRoleModifyDispatchData {
    /**
     * The id of the guild
     */
    pub guild_id: String,
    /**
     * The role created or updated
     *
     * @see {@link https://discord.com/developers/docs/topics/permissions#role-object}
     */
    pub role: APIRole,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-role-create}
 */
pub type GatewayGuildRoleCreateDispatch = GatewayGuildRoleModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-role-create}
 */
pub type GatewayGuildRoleCreateDispatchData = GatewayGuildRoleModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-role-update}
 */
pub type GatewayGuildRoleUpdateDispatch = GatewayGuildRoleModifyDispatch;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-role-update}
 */
pub type GatewayGuildRoleUpdateDispatchData = GatewayGuildRoleModifyDispatchData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-role-delete}
 */
pub type GatewayGuildRoleDeleteDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildRoleDeleteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-role-delete}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayGuildRoleDeleteDispatchData {
    /**
     * The id of the guild
     */
    pub guild_id: String,
    /**
     * The id of the role
     */
    pub role_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-create}
 */
pub type GatewayGuildScheduledEventCreateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildScheduledEventCreateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-create}
 */
pub type GatewayGuildScheduledEventCreateDispatchData = APIGuildScheduledEvent;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-update}
 */
pub type GatewayGuildScheduledEventUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildScheduledEventUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-update}
 */
pub type GatewayGuildScheduledEventUpdateDispatchData = APIGuildScheduledEvent;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-delete}
 */
pub type GatewayGuildScheduledEventDeleteDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildScheduledEventDeleteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-delete}
 */
pub type GatewayGuildScheduledEventDeleteDispatchData = APIGuildScheduledEvent;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-user-add}
 */
pub type GatewayGuildScheduledEventUserAddDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildScheduledEventUserAddDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-user-add}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayGuildScheduledEventUserAddDispatchData {
    pub guild_scheduled_event_id: String,
    pub user_id: String,
    pub guild_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-user-remove}
 */
pub type GatewayGuildScheduledEventUserRemoveDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildScheduledEventUserAddDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-user-remove}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayGuildScheduledEventUserRemoveDispatchData {
    pub guild_scheduled_event_id: String,
    pub user_id: String,
    pub guild_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-soundboard-sound-create}
 */
pub type GatewayGuildSoundboardSoundCreateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildSoundboardSoundCreateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-soundboard-sound-create}
 */
pub type GatewayGuildSoundboardSoundCreateDispatchData = APISoundboardSound;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-soundboard-sound-update}
 */
pub type GatewayGuildSoundboardSoundUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildSoundboardSoundUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-soundboard-sound-update}
 */
pub type GatewayGuildSoundboardSoundUpdateDispatchData = APISoundboardSound;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-soundboard-sound-delete}
 */
pub type GatewayGuildSoundboardSoundDeleteDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildSoundboardSoundDeleteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-soundboard-sound-delete}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayGuildSoundboardSoundDeleteDispatchData {
    /**
     * The id of the sound that was deleted
     */
    pub sound_id: String,
    /**
     * The id of the guild the sound was in
     */
    pub guild_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-soundboard-sounds-update}
 */
pub type GatewayGuildSoundboardSoundsUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildSoundboardSoundsUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-soundboard-sounds-update}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayGuildSoundboardSoundsUpdateDispatchData {
    /**
     * The guild's soundboard sounds
     */
    pub soundboard_sounds: Vec<APISoundboardSound>,
    /**
     * The id of the guild
     */
    pub guild_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/events/gateway-events#soundboard-sounds}
 */
pub type GatewaySoundboardSoundsDispatch =
    _DataPayload<GatewayDispatchEvents, GatewaySoundboardSoundsDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/events/gateway-events#soundboard-sounds}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewaySoundboardSoundsDispatchData {
    /**
     * The guild's soundboard sounds
     */
    pub soundboard_sounds: Vec<APISoundboardSound>,
    /**
     * The id of the guild
     */
    pub guild_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#integration-create}
 */
pub type GatewayIntegrationCreateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayIntegrationCreateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#integration-create}
 */
pub type GatewayIntegrationCreateDispatchData = APIGuildIntegrationWithGuild;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIGuildIntegrationWithGuild {
    pub guild_id: String,
    #[serde(flatten)]
    pub integration: APIGuildIntegration,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#integration-update}
 */
pub type GatewayIntegrationUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayIntegrationUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#integration-update}
 */
pub type GatewayIntegrationUpdateDispatchData = APIGuildIntegrationWithGuild;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#integration-update}
 */
pub type GatewayIntegrationDeleteDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayIntegrationDeleteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#integration-delete}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayIntegrationDeleteDispatchData {
    /**
     * Integration id
     */
    pub id: String,
    /**
     * ID of the guild
     */
    pub guild_id: String,
    /**
     * ID of the bot/OAuth2 application for this Discord integration
     */
    pub application_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#interaction-create}
 */
pub type GatewayInteractionCreateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayInteractionCreateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#interaction-create}
 */
pub type GatewayInteractionCreateDispatchData = APIInteraction;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#invite-create}
 */
pub type GatewayInviteCreateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayInviteCreateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#invite-create}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayInviteCreateDispatchData {
    /**
     * The channel the invite is for
     */
    pub channel_id: String,
    /**
     * The unique invite code
     *
     * @see {@link https://discord.com/developers/docs/resources/invite#invite-object}
     */
    pub code: String,
    /**
     * The time at which the invite was created
     */
    pub created_at: i64,
    /**
     * The guild of the invite
     */
    pub guild_id: Option<String>,
    /**
     * The user that created the invite
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub inviter: Option<APIUser>,
    /**
     * How long the invite is valid for (in seconds)
     */
    pub max_age: i64,
    /**
     * The maximum number of times the invite can be used
     */
    pub max_uses: i64,
    /**
     * The type of target for this voice channel invite
     *
     * @see {@link https://discord.com/developers/docs/resources/invite#invite-object-invite-target-types}
     */
    pub target_type: Option<InviteTargetType>,
    /**
     * The user whose stream to display for this voice channel stream invite
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub target_user: Option<APIUser>,
    /**
     * The embedded application to open for this voice channel embedded application invite
     */
    pub target_application: Option<APIApplication>,
    /**
     * Whether or not the invite is temporary (invited users will be kicked on disconnect unless they're assigned a role)
     */
    pub temporary: bool,
    /**
     * How many times the invite has been used (always will be `0`)
     */
    pub uses: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#invite-delete}
 */
pub type GatewayInviteDeleteDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayInviteDeleteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#invite-delete}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayInviteDeleteDispatchData {
    /**
     * The channel of the invite
     */
    pub channel_id: String,
    /**
     * The guild of the invite
     */
    pub guild_id: Option<String>,
    /**
     * The unique invite code
     *
     * @see {@link https://discord.com/developers/docs/resources/invite#invite-object}
     */
    pub code: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-create}
 */
pub type GatewayMessageCreateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayMessageCreateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-create}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayMessageCreateDispatchData {
    #[serde(flatten)]
    pub extra: GatewayMessageEventExtraFields,
    #[serde(flatten)]
    pub message: APIBaseMessage,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-update}
 */
pub type GatewayMessageUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayMessageUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-update}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayMessageUpdateDispatchData {
    #[serde(flatten)]
    pub extra: GatewayMessageEventExtraFields,
    #[serde(flatten)]
    pub message: APIBaseMessage,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIGuildMemberNoUser {
    #[serde(flatten)]
    pub base: APIBaseGuildMember,
    #[serde(flatten)]
    pub flagged: APIFlaggedGuildMember,
    #[serde(flatten)]
    pub avatar: APIGuildMemberAvatar,
    #[serde(flatten)]
    pub joined: APIGuildMemberJoined,
    #[serde(flatten)]
    pub voice: APIBaseVoiceGuildMember,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIUserWithMember {
    /**
     * The `member` field is only present in `MESSAGE_CREATE` and `MESSAGE_UPDATE` events
     * from text-based guild channels
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
     */
    pub member: Option<APIGuildMemberNoUser>,
    #[serde(flatten)]
    pub user: APIUser,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-create-message-create-extra-fields}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayMessageEventExtraFields {
    /**
     * ID of the guild the message was sent in
     */
    pub guild_id: Option<String>,
    /**
     * Member properties for this message's author
     *
     * The member object exists in `MESSAGE_CREATE` and `MESSAGE_UPDATE` events
     * from text-based guild channels
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
     */
    pub member: Option<APIGuildMemberNoUser>,
    /**
     * Users specifically mentioned in the message
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub mentions: Vec<APIUserWithMember>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-delete}
 */
pub type GatewayMessageDeleteDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayMessageDeleteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-delete}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayMessageDeleteDispatchData {
    /**
     * The id of the message
     */
    pub id: String,
    /**
     * The id of the channel
     */
    pub channel_id: String,
    /**
     * The id of the guild
     */
    pub guild_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-delete-bulk}
 */
pub type GatewayMessageDeleteBulkDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayMessageDeleteBulkDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-delete-bulk}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayMessageDeleteBulkDispatchData {
    /**
     * The ids of the messages
     */
    pub ids: Vec<String>,
    /**
     * The id of the channel
     */
    pub channel_id: String,
    /**
     * The id of the guild
     */
    pub guild_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-reaction-add}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayMessageReactionAddDispatchData {
    /**
     * The member who reacted if this happened in a guild
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
     */
    pub member: Option<APIGuildMember>,
    /**
     * The id of the user that posted the message that was reacted to
     */
    pub message_author_id: Option<String>,
    /**
     * Colors used for super-reaction animation in "#rrggbb" format
     */
    pub burst_colors: Option<Vec<String>>,
    #[serde(flatten)]
    pub rest: GatewayMessageReactionRemoveDispatchData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-reaction-add}
 */
pub type GatewayMessageReactionAddDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayMessageReactionAddDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayMessageReactionRemoveDispatchData {
    /**
     * The id of the user
     */
    pub user_id: String,
    /**
     * The id of the channel
     */
    pub channel_id: String,
    /**
     * The id of the message
     */
    pub message_id: String,
    /**
     * The id of the guild
     */
    pub guild_id: Option<String>,
    /**
     * The emoji used to react
     *
     * @see {@link https://discord.com/developers/docs/resources/emoji#emoji-object}
     */
    pub emoji: APIEmoji,
    /**
     * True if this is a super-reaction
     */
    pub burst: bool,
    /**
     * The type of reaction
     */
    pub r#type: ReactionType,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove}
 */
pub type GatewayMessageReactionRemoveDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayMessageReactionRemoveDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove-all}
 */
pub type GatewayMessageReactionRemoveAllDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayMessageReactionRemoveAllDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove-all}
 */
pub type GatewayMessageReactionRemoveAllDispatchData = GatewayMessageReactionRemoveData;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove-emoji}
 */
pub type GatewayMessageReactionRemoveEmojiDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayMessageReactionRemoveEmojiDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove-emoji}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayMessageReactionRemoveEmojiDispatchData {
    /**
     * The emoji that was removed
     */
    pub emoji: APIEmoji,
    #[serde(flatten)]
    pub rest: GatewayMessageReactionRemoveData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#presence-update}
 */
pub type GatewayPresenceUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayPresenceUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#presence-update}
 */
pub type GatewayPresenceUpdateDispatchData = GatewayPresenceUpdate;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#stage-instance-create}
 */
pub type GatewayStageInstanceCreateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayStageInstanceCreateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#stage-instance-create}
 */
pub type GatewayStageInstanceCreateDispatchData = APIStageInstance;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#stage-instance-delete}
 */
pub type GatewayStageInstanceDeleteDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayStageInstanceDeleteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#stage-instance-delete}
 */
pub type GatewayStageInstanceDeleteDispatchData = APIStageInstance;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#stage-instance-update}
 */
pub type GatewayStageInstanceUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayStageInstanceUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#stage-instance-update}
 */
pub type GatewayStageInstanceUpdateDispatchData = APIStageInstance;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-list-sync}
 */
pub type GatewayThreadListSyncDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayThreadListSyncDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-list-sync}
 */
pub type GatewayThreadListSyncDispatchData = GatewayThreadListSync;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-members-update}
 */
pub type GatewayThreadMembersUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayThreadMembersUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-members-update}
 */
pub type GatewayThreadMembersUpdateDispatchData = RawGatewayThreadMembersUpdate;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-member-update}
 */
pub type GatewayThreadMemberUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayThreadMemberUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-member-update}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayThreadMemberUpdateDispatchData {
    pub guild_id: String,
    #[serde(flatten)]
    pub member: APIThreadMember,
}

/**
 * @deprecated This type doesn't accurately reflect the Discord API.
 * Use {@link GatewayThreadCreateDispatch},
 * {@link GatewayThreadUpdateDispatch}, or
 * {@link GatewayThreadDeleteDispatch} instead.
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-create}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-update}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-delete}
 */
pub type GatewayThreadModifyDispatch = _DataPayload<GatewayDispatchEvents, APIThreadChannel>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-create}
 */
pub type GatewayThreadCreateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayThreadCreateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-create}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayThreadCreateDispatchData {
    /**
     * Whether the thread is newly created or not.
     */
    pub newly_created: Option<bool>,
    #[serde(flatten)]
    pub thread: APIThreadChannel,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-update}
 */
pub type GatewayThreadUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayThreadUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-update}
 */
pub type GatewayThreadUpdateDispatchData = APIThreadChannel;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-delete}
 */
pub type GatewayThreadDeleteDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayThreadDeleteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-delete}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayThreadDeleteDispatchData {
    /**
     * The id of the channel
     */
    pub id: String,
    /**
     * The id of the guild
     */
    pub guild_id: String,
    /**
     * The id of the parent channel of the thread
     */
    pub parent_id: String,
    /**
     * The type of the channel
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object-channel-types}
     */
    pub r#type: ChannelType,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#typing-start}
 */
pub type GatewayTypingStartDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayTypingStartDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#typing-start}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayTypingStartDispatchData {
    /**
     * The id of the channel
     */
    pub channel_id: String,
    /**
     * The id of the guild
     */
    pub guild_id: Option<String>,
    /**
     * The id of the user
     */
    pub user_id: String,
    /**
     * Unix time (in seconds) of when the user started typing
     */
    pub timestamp: i64,
    /**
     * The member who started typing if this happened in a guild
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
     */
    pub member: Option<APIGuildMember>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#user-update}
 */
pub type GatewayUserUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayUserUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#user-update}
 */
pub type GatewayUserUpdateDispatchData = APIUser;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#voice-channel-effect-send}
 */
pub type GatewayVoiceChannelEffectSendDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayVoiceChannelEffectSendDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#voice-channel-effect-send}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayVoiceChannelEffectSendDispatchData {
    /**
     * ID of the channel the effect was sent in
     */
    pub channel_id: String,
    /**
     * ID of the guild the effect was sent in
     */
    pub guild_id: String,
    /**
     * ID of the user who sent the effect
     */
    pub user_id: String,
    /**
     * The emoji sent, for emoji reaction and soundboard effects
     */
    pub emoji: Option<APIEmoji>,
    /**
     * The type of emoji animation, for emoji reaction and soundboard effects
     */
    pub animation_type: Option<VoiceChannelEffectSendAnimationType>,
    /**
     * The ID of the emoji animation, for emoji reaction and soundboard effects
     */
    pub animation_id: Option<i64>,
    /**
     * The ID of the soundboard sound, for soundboard effects
     */
    pub sound_id: Option<StringOrNumber>,
    /**
     * The volume of the soundboard sound, from 0 to 1, for soundboard effects
     */
    pub sound_volume: Option<f64>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#voice-channel-effect-send-animation-types}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceChannelEffectSendAnimationType {
    /**
     * A fun animation, sent by a Nitro subscriber
     */
    Premium,
    /**
     * The standard animation
     */
    Basic,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(i64),
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#voice-state-update}
 */
pub type GatewayVoiceStateUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayVoiceStateUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#voice-state-update}
 */
pub type GatewayVoiceStateUpdateDispatchData = APIVoiceState;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#voice-server-update}
 */
pub type GatewayVoiceServerUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayVoiceServerUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#voice-server-update}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayVoiceServerUpdateDispatchData {
    /**
     * Voice connection token
     */
    pub token: String,
    /**
     * The guild this voice server update is for
     */
    pub guild_id: String,
    /**
     * The voice server host
     *
     * A `null` endpoint means that the voice server allocated has gone away and is trying to be reallocated.
     * You should attempt to disconnect from the currently connected voice server, and not attempt to reconnect
     * until a new voice server is allocated
     */
    pub endpoint: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#webhooks-update}
 */
pub type GatewayWebhooksUpdateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayWebhooksUpdateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#webhooks-update}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayWebhooksUpdateDispatchData {
    /**
     * The id of the guild
     */
    pub guild_id: String,
    /**
     * The id of the channel
     */
    pub channel_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-audit-log-entry-create}
 */
pub type GatewayGuildAuditLogEntryCreateDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayGuildAuditLogEntryCreateDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-audit-log-entry-create}
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayGuildAuditLogEntryCreateDispatchData {
    /**
     * ID of the guild
     */
    pub guild_id: String,
    #[serde(flatten)]
    pub entry: APIAuditLogEntry,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-poll-vote-add}
 */
pub type GatewayMessagePollVoteAddDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayMessagePollVoteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-poll-vote-remove}
 */
pub type GatewayMessagePollVoteRemoveDispatch =
    _DataPayload<GatewayDispatchEvents, GatewayMessagePollVoteDispatchData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-poll-vote-add}
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#message-poll-vote-remove}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayMessagePollVoteDispatchData {
    /**
     * ID of the user
     */
    pub user_id: String,
    /**
     * ID of the channel
     */
    pub channel_id: String,
    /**
     * ID of the message
     */
    pub message_id: String,
    /**
     * ID of the guild
     */
    pub guild_id: Option<String>,
    /**
     * ID of the answer
     */
    pub answer_id: i64,
}

// #endregion Dispatch Payloads

// #region Sendable Payloads

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway#sending-heartbeats}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayHeartbeat {
    pub op: GatewayOpcodes,
    pub d: GatewayHeartbeatData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway#sending-heartbeats}
 */
pub type GatewayHeartbeatData = Option<i64>;

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#identify}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayIdentify {
    pub op: GatewayOpcodes,
    pub d: GatewayIdentifyData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#identify}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayIdentifyData {
    /**
     * Authentication token
     */
    pub token: String,
    /**
     * Connection properties
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#identify-identify-connection-properties}
     */
    pub properties: GatewayIdentifyProperties,
    /**
     * Whether this connection supports compression of packets
     *
     * @defaultValue `false`
     */
    pub compress: Option<bool>,
    /**
     * Value between 50 and 250, total number of members where the gateway will stop sending
     * offline members in the guild member list
     *
     * @defaultValue `50`
     */
    pub large_threshold: Option<i64>,
    /**
     * Used for Guild Sharding
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway#sharding}
     */
    pub shard: Option<(i64, i64)>,
    /**
     * Presence structure for initial presence information
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#update-presence}
     */
    pub presence: Option<GatewayPresenceUpdateData>,
    /**
     * The Gateway Intents you wish to receive
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway#gateway-intents}
     */
    pub intents: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#identify-identify-connection-properties}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayIdentifyProperties {
    /**
     * Your operating system
     */
    pub os: String,
    /**
     * Your library name
     */
    pub browser: String,
    /**
     * Your library name
     */
    pub device: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#resume}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayResume {
    pub op: GatewayOpcodes,
    pub d: GatewayResumeData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#resume}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayResumeData {
    /**
     * Session token
     */
    pub token: String,
    /**
     * Session id
     */
    pub session_id: String,
    /**
     * Last sequence number received
     */
    pub seq: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#request-guild-members}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayRequestGuildMembers {
    pub op: GatewayOpcodes,
    pub d: GatewayRequestGuildMembersData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#request-guild-members}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayRequestGuildMembersDataBase {
    /**
     * ID of the guild to get members for
     */
    pub guild_id: String,
    /**
     * Used to specify if we want the presences of the matched members
     */
    pub presences: Option<bool>,
    /**
     * Nonce to identify the Guild Members Chunk response
     *
     * Nonce can only be up to 32 bytes. If you send an invalid nonce it will be ignored and the reply member_chunk(s) will not have a `nonce` set.
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#guild-members-chunk}
     */
    pub nonce: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#request-guild-members}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayRequestGuildMembersDataWithUserIds {
    #[serde(flatten)]
    pub base: GatewayRequestGuildMembersDataBase,
    /**
     * Used to specify which users you wish to fetch
     */
    pub user_ids: StringOrStrings,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrStrings {
    One(String),
    Many(Vec<String>),
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#request-guild-members}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayRequestGuildMembersDataWithQuery {
    #[serde(flatten)]
    pub base: GatewayRequestGuildMembersDataBase,
    /**
     * String that username starts with, or an empty string to return all members
     */
    pub query: String,
    /**
     * Maximum number of members to send matching the `query`;
     * a limit of `0` can be used with an empty string `query` to return all members
     */
    pub limit: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#request-guild-members}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GatewayRequestGuildMembersData {
    WithQuery(GatewayRequestGuildMembersDataWithQuery),
    WithUserIds(GatewayRequestGuildMembersDataWithUserIds),
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#request-soundboard-sounds}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayRequestSoundboardSounds {
    pub op: GatewayOpcodes,
    pub d: GatewayRequestSoundboardSoundsData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#request-soundboard-sounds}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayRequestSoundboardSoundsData {
    /**
     * The ids of the guilds to get soundboard sounds for
     */
    pub guild_ids: Vec<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#update-voice-state}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayVoiceStateUpdate {
    pub op: GatewayOpcodes,
    pub d: GatewayVoiceStateUpdateData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#update-voice-state}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayVoiceStateUpdateData {
    /**
     * ID of the guild
     */
    pub guild_id: String,
    /**
     * ID of the voice channel client wants to join (`null` if disconnecting)
     */
    pub channel_id: Option<String>,
    /**
     * Is the client muted
     */
    pub self_mute: bool,
    /**
     * Is the client deafened
     */
    pub self_deaf: bool,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#update-presence}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayUpdatePresence {
    pub op: GatewayOpcodes,
    pub d: GatewayPresenceUpdateData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#update-presence-gateway-presence-update-structure}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayPresenceUpdateData {
    /**
     * Unix time (in milliseconds) of when the client went idle, or `null` if the client is not idle
     */
    pub since: Option<i64>,
    /**
     * The user's activities
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object}
     */
    pub activities: Vec<GatewayActivityUpdateData>,
    /**
     * The user's new status
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#update-presence-status-types}
     */
    pub status: PresenceUpdateStatus,
    /**
     * Whether or not the client is afk
     */
    pub afk: bool,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-structure}
 */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayActivityUpdateData {
    pub name: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    pub url: Option<String>,
}

// #endregion Sendable Payloads

// #region Shared
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct _BaseBasePayload {
    /**
     * Opcode for the payload
     */
    pub op: GatewayOpcodes,
    /**
     * Event data
     */
    pub d: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct _BasePayload {
    /**
     * Sequence number, used for resuming sessions and heartbeats
     */
    pub s: i64,
    /**
     * The event name for this payload
     */
    pub t: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct _NonDispatchPayload {
    pub t: Option<serde_json::Value>,
    pub s: Option<serde_json::Value>,
    #[serde(flatten)]
    pub base: _BaseBasePayload,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct _DataPayload<Event, D> {
    pub op: GatewayOpcodes,
    pub t: Event,
    pub d: D,
    pub s: i64,
}

/**
 * This is not used internally anymore, just remains to be non-breaking
 */
pub type GatewayMessageReactionData<E> = _DataPayload<E, GatewayMessageReactionAddDispatchData>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayMessageReactionRemoveData {
    /**
     * The id of the channel
     */
    pub channel_id: String,
    /**
     * The id of the message
     */
    pub message_id: String,
    /**
     * The id of the guild
     */
    pub guild_id: Option<String>,
}
// #endregion Shared
