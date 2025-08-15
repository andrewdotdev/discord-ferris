//! Types extracted from https://discord.com/developers/docs/resources/audit-log

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// -------------------------------------------------------------------------------------------------
// Shared aliases
// -------------------------------------------------------------------------------------------------

/// Stringified 64-bit identifier used across Discord.
pub type Snowflake = String;

use crate::models::payloads::APIWebhook;
use crate::models::payloads::user::APIUser;

use super::auto_moderation::{APIAutoModerationRule, AutoModerationRuleTriggerType};
use super::channel::APIChannel;
use super::guild::{APIGuildIntegration, APIGuildIntegrationType};
use super::guild_scheduled_event::APIGuildScheduledEvent;
use crate::models::payloads::application_commands::APIApplicationCommand;

// -------------------------------------------------------------------------------------------------
// Audit Log root
// -------------------------------------------------------------------------------------------------

/// Container returned by the Audit Log endpoint.
/// @see {@link https://discord.com/developers/docs/resources/audit-log#audit-log-object-audit-log-structure}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAuditLog {
    /// List of application commands found in the audit log
    /// @see {@link https://discord.com/developers/docs/interactions/application-commands#application-command-object}
    pub application_commands: Vec<APIApplicationCommand>,
    /// Webhooks found in the audit log
    /// @see {@link https://discord.com/developers/docs/resources/webhook#webhook-object}
    pub webhooks: Vec<APIWebhook>,
    /// Users found in the audit log
    /// @see {@link https://discord.com/developers/docs/resources/user#user-object}
    pub users: Vec<APIUser>,
    /// Audit log entries
    /// @see {@link https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object}
    pub audit_log_entries: Vec<APIAuditLogEntry>,
    /// List of auto moderation rules referenced in the audit log
    /// @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object}
    pub auto_moderation_rules: Vec<APIAutoModerationRule>,
    /// Partial integration objects
    /// @see {@link https://discord.com/developers/docs/resources/guild#integration-object}
    pub integrations: Vec<APIGuildIntegration>,
    /// Threads found in the audit log
    ///
    /// Threads referenced in THREAD_CREATE and THREAD_UPDATE events are included in the threads list,
    /// since archived threads might not be kept in memory by clients.
    /// @see {@link https://discord.com/developers/docs/resources/channel#channel-object}
    pub threads: Vec<APIChannel>,
    /// The guild scheduled events in the audit log
    /// @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object}
    pub guild_scheduled_events: Vec<APIGuildScheduledEvent>,
}

// -------------------------------------------------------------------------------------------------
// Audit Log Entry
// -------------------------------------------------------------------------------------------------

/// One entry describing a moderation/configuration action.
/// @see {@link https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object-audit-log-entry-structure}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAuditLogEntry {
    /// ID of the affected entity (webhook, user, role, etc.)
    pub target_id: Option<String>,
    /// Changes made to the `target_id`
    /// @see {@link https://discord.com/developers/docs/resources/audit-log#audit-log-change-object}
    pub changes: Option<Vec<APIAuditLogChange>>,
    /// The user who made the changes. Can be `null` (e.g. webhooks acting on themselves).
    pub user_id: Option<Snowflake>,
    /// ID of the entry
    pub id: Snowflake,
    /// Type of action that occurred
    /// @see {@link https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object-audit-log-events}
    pub action_type: AuditLogEvent,
    /// Additional info for certain action types
    /// @see {@link https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object-optional-audit-entry-info}
    pub options: Option<APIAuditLogOptions>,
    /// The reason for the change (0-512 characters)
    pub reason: Option<String>,
}

/// Enumeration of all audit log actions (stable numeric codes).
/// @see {@link https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object-audit-log-events}
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum AuditLogEvent {
    GuildUpdate = 1,

    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelOverwriteCreate = 13,
    ChannelOverwriteUpdate = 14,
    ChannelOverwriteDelete = 15,

    MemberKick = 20,
    MemberPrune = 21,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    MemberRoleUpdate = 25,
    MemberMove = 26,
    MemberDisconnect = 27,
    BotAdd = 28,

    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,

    InviteCreate = 40,
    InviteUpdate = 41,
    InviteDelete = 42,

    WebhookCreate = 50,
    WebhookUpdate = 51,
    WebhookDelete = 52,

    EmojiCreate = 60,
    EmojiUpdate = 61,
    EmojiDelete = 62,

    MessageDelete = 72,
    MessageBulkDelete = 73,
    MessagePin = 74,
    MessageUnpin = 75,

    IntegrationCreate = 80,
    IntegrationUpdate = 81,
    IntegrationDelete = 82,
    StageInstanceCreate = 83,
    StageInstanceUpdate = 84,
    StageInstanceDelete = 85,

    StickerCreate = 90,
    StickerUpdate = 91,
    StickerDelete = 92,

    GuildScheduledEventCreate = 100,
    GuildScheduledEventUpdate = 101,
    GuildScheduledEventDelete = 102,

    ThreadCreate = 110,
    ThreadUpdate = 111,
    ThreadDelete = 112,

    ApplicationCommandPermissionUpdate = 121,

    SoundboardSoundCreate = 130,
    SoundboardSoundUpdate = 131,
    SoundboardSoundDelete = 132,

    AutoModerationRuleCreate = 140,
    AutoModerationRuleUpdate = 141,
    AutoModerationRuleDelete = 142,
    AutoModerationBlockMessage = 143,
    AutoModerationFlagToChannel = 144,
    AutoModerationUserCommunicationDisabled = 145,
    AutoModerationQuarantineUser = 146,

    CreatorMonetizationRequestCreated = 150,
    CreatorMonetizationTermsAccepted = 151,

    OnboardingPromptCreate = 163,
    OnboardingPromptUpdate = 164,
    OnboardingPromptDelete = 165,
    OnboardingCreate = 166,
    OnboardingUpdate = 167,

    HomeSettingsCreate = 190,
    HomeSettingsUpdate = 191,
}

// -------------------------------------------------------------------------------------------------
// Optional Audit Entry Info
// -------------------------------------------------------------------------------------------------

/// Extra metadata present on specific actions.
/// @see {@link https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object-optional-audit-entry-info}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAuditLogOptions {
    /// Name of the Auto Moderation rule that was triggered
    /// Present for: BLOCK_MESSAGE, FLAG_TO_CHANNEL, USER_COMMUNICATION_DISABLED, QUARANTINE_USER
    pub auto_moderation_rule_name: Option<String>,

    /// Trigger type of the Auto Moderation rule that was triggered
    /// Present for: BLOCK_MESSAGE, FLAG_TO_CHANNEL, USER_COMMUNICATION_DISABLED, QUARANTINE_USER
    pub auto_moderation_rule_trigger_type: Option<AuditLogRuleTriggerType>,

    /// Number of days after which inactive members were kicked (MEMBER_PRUNE)
    pub delete_member_days: Option<String>,
    /// Number of members removed by the prune (MEMBER_PRUNE)
    pub members_removed: Option<String>,

    /// Channel in which the entities were targeted (various message/stage/auto-mod actions)
    pub channel_id: Option<Snowflake>,
    /// ID of the message that was targeted (MESSAGE_PIN / UNPIN)
    pub message_id: Option<Snowflake>,
    /// Number of entities that were targeted (bulk deletes/moves/disconnects)
    pub count: Option<String>,

    /// ID of the overwritten entity (permission overwrite actions)
    pub id: Option<Snowflake>,
    /// Type of overwritten entity - "0" for role or "1" for member
    pub r#type: Option<AuditLogOptionsType>,
    /// Name of the role (present if `type` is role)
    pub role_name: Option<String>,

    /// Type of integration which performed the action (kick/role update)
    pub integration_type: Option<APIGuildIntegrationType>,
}

/// "0" for role, "1" for member (stringly-typed by Discord).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditLogOptionsType {
    #[serde(rename = "0")]
    Role,
    #[serde(rename = "1")]
    Member,
}

/// Alias for the Auto Moderation trigger type used inside audit log options.
pub type AuditLogRuleTriggerType = AutoModerationRuleTriggerType;

// -------------------------------------------------------------------------------------------------
// Changes (generic representation)
// -------------------------------------------------------------------------------------------------

/// Generic change record emitted inside `APIAuditLogEntry::changes`.
///
/// TypeScript models this as a **large union** keyed by `key` (e.g., "name", "icon_hash", "$add").
/// To keep this ergonomic and future-proof, we use a generic payload with `serde_json::Value`.
/// You can pattern-match on `key` to downcast `new_value`/`old_value` as needed.
/// @see {@link https://discord.com/developers/docs/resources/audit-log#audit-log-change-object-audit-log-change-structure}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAuditLogChange {
    /// Discriminant describing which property changed.
    pub key: AuditLogChangeKey,
    /// New value for the property (if present). When missing but `old_value` exists, the
    /// property was reset or set to `null`.
    pub new_value: Option<serde_json::Value>,
    /// Previous value for the property (if present).
    pub old_value: Option<serde_json::Value>,
}

/// Known change keys as of the documentation snapshot.
/// Keeping keys as an enum improves safety while allowing forward compatibility
/// via the `Unknown(String)` variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditLogChangeKey {
    /// Known literal keys with stable string forms
    #[serde(with = "audit_log_change_key_known")]
    Known(AuditLogChangeKeyKnown),
    /// Any unknown/forward-compatible key string
    Other(String),
}

/// Helper module to (de)serialize the known key enum as a bare string.
mod audit_log_change_key_known {
    use super::AuditLogChangeKeyKnown;
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub fn serialize<S: Serializer>(v: &AuditLogChangeKeyKnown, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(match v {
            AuditLogChangeKeyKnown::Name => "name",
            AuditLogChangeKeyKnown::Description => "description",
            AuditLogChangeKeyKnown::IconHash => "icon_hash",
            AuditLogChangeKeyKnown::ImageHash => "image_hash",
            AuditLogChangeKeyKnown::SplashHash => "splash_hash",
            AuditLogChangeKeyKnown::DiscoverySplashHash => "discovery_splash_hash",
            AuditLogChangeKeyKnown::BannerHash => "banner_hash",
            AuditLogChangeKeyKnown::OwnerId => "owner_id",
            AuditLogChangeKeyKnown::Region => "region",
            AuditLogChangeKeyKnown::PreferredLocale => "preferred_locale",
            AuditLogChangeKeyKnown::AfkChannelId => "afk_channel_id",
            AuditLogChangeKeyKnown::AfkTimeout => "afk_timeout",
            AuditLogChangeKeyKnown::RulesChannelId => "rules_channel_id",
            AuditLogChangeKeyKnown::PublicUpdatesChannelId => "public_updates_channel_id",
            AuditLogChangeKeyKnown::SafetyAlertsChannelId => "safety_alerts_channel_id",
            AuditLogChangeKeyKnown::MfaLevel => "mfa_level",
            AuditLogChangeKeyKnown::VerificationLevel => "verification_level",
            AuditLogChangeKeyKnown::ExplicitContentFilter => "explicit_content_filter",
            AuditLogChangeKeyKnown::DefaultMessageNotifications => "default_message_notifications",
            AuditLogChangeKeyKnown::VanityUrlCode => "vanity_url_code",
            AuditLogChangeKeyKnown::PremiumProgressBarEnabled => "premium_progress_bar_enabled",
            AuditLogChangeKeyKnown::Add => "$add",
            AuditLogChangeKeyKnown::Remove => "$remove",
            AuditLogChangeKeyKnown::PruneDeleteDays => "prune_delete_days",
            AuditLogChangeKeyKnown::WidgetEnabled => "widget_enabled",
            AuditLogChangeKeyKnown::WidgetChannelId => "widget_channel_id",
            AuditLogChangeKeyKnown::SystemChannelFlags => "system_channel_flags",
            AuditLogChangeKeyKnown::SystemChannelId => "system_channel_id",
            AuditLogChangeKeyKnown::Position => "position",
            AuditLogChangeKeyKnown::Topic => "topic",
            AuditLogChangeKeyKnown::Bitrate => "bitrate",
            AuditLogChangeKeyKnown::PermissionOverwrites => "permission_overwrites",
            AuditLogChangeKeyKnown::Nsfw => "nsfw",
            AuditLogChangeKeyKnown::ApplicationId => "application_id",
            AuditLogChangeKeyKnown::RateLimitPerUser => "rate_limit_per_user",
            AuditLogChangeKeyKnown::RecurrenceRule => "recurrence_rule",
            AuditLogChangeKeyKnown::Permissions => "permissions",
            AuditLogChangeKeyKnown::Color => "color",
            AuditLogChangeKeyKnown::Snowflake => "id", // special generic snowflake change id
            AuditLogChangeKeyKnown::Hoist => "hoist",
            AuditLogChangeKeyKnown::Mentionable => "mentionable",
            AuditLogChangeKeyKnown::Allow => "allow",
            AuditLogChangeKeyKnown::Deny => "deny",
            AuditLogChangeKeyKnown::Code => "code",
            AuditLogChangeKeyKnown::ChannelId => "channel_id",
            AuditLogChangeKeyKnown::InviterId => "inviter_id",
            AuditLogChangeKeyKnown::MaxUses => "max_uses",
            AuditLogChangeKeyKnown::Uses => "uses",
            AuditLogChangeKeyKnown::MaxAge => "max_age",
            AuditLogChangeKeyKnown::Temporary => "temporary",
            AuditLogChangeKeyKnown::Deaf => "deaf",
            AuditLogChangeKeyKnown::Mute => "mute",
            AuditLogChangeKeyKnown::Nick => "nick",
            AuditLogChangeKeyKnown::AvatarHash => "avatar_hash",
            AuditLogChangeKeyKnown::ChangedEntityId => "id",
            AuditLogChangeKeyKnown::Type => "type",
            AuditLogChangeKeyKnown::EnableEmoticons => "enable_emoticons",
            AuditLogChangeKeyKnown::ExpireBehavior => "expire_behavior",
            AuditLogChangeKeyKnown::ExpireGracePeriod => "expire_grace_period",
            AuditLogChangeKeyKnown::UserLimit => "user_limit",
            AuditLogChangeKeyKnown::PrivacyLevel => "privacy_level",
            AuditLogChangeKeyKnown::Tags => "tags",
            AuditLogChangeKeyKnown::FormatType => "format_type",
            AuditLogChangeKeyKnown::Asset => "asset",
            AuditLogChangeKeyKnown::Available => "available",
            AuditLogChangeKeyKnown::GuildId => "guild_id",
            AuditLogChangeKeyKnown::Archived => "archived",
            AuditLogChangeKeyKnown::Locked => "locked",
            AuditLogChangeKeyKnown::AutoArchiveDuration => "auto_archive_duration",
            AuditLogChangeKeyKnown::DefaultAutoArchiveDuration => "default_auto_archive_duration",
            AuditLogChangeKeyKnown::EntityType => "entity_type",
            AuditLogChangeKeyKnown::Status => "status",
            AuditLogChangeKeyKnown::Location => "location",
            AuditLogChangeKeyKnown::CommunicationDisabledUntil => "communication_disabled_until",
            AuditLogChangeKeyKnown::TriggerType => "trigger_type",
            AuditLogChangeKeyKnown::EventType => "event_type",
            AuditLogChangeKeyKnown::TriggerMetadata => "trigger_metadata",
            AuditLogChangeKeyKnown::Actions => "actions",
            AuditLogChangeKeyKnown::Enabled => "enabled",
            AuditLogChangeKeyKnown::ExemptRoles => "exempt_roles",
            AuditLogChangeKeyKnown::ExemptChannels => "exempt_channels",
            AuditLogChangeKeyKnown::AvailableTags => "available_tags",
            AuditLogChangeKeyKnown::DefaultReactionEmoji => "default_reaction_emoji",
            AuditLogChangeKeyKnown::Flags => "flags",
            AuditLogChangeKeyKnown::DefaultThreadRateLimitPerUser => {
                "default_thread_rate_limit_per_user"
            }
            AuditLogChangeKeyKnown::SoundId => "sound_id",
            AuditLogChangeKeyKnown::Volume => "volume",
            AuditLogChangeKeyKnown::EmojiId => "emoji_id",
            AuditLogChangeKeyKnown::EmojiName => "emoji_name",
            AuditLogChangeKeyKnown::UserId => "user_id",
            AuditLogChangeKeyKnown::PreferredLocaleKey => "preferred_locale",
        })
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<AuditLogChangeKeyKnown, D::Error> {
        let s = String::deserialize(d)?;
        Ok(match s.as_str() {
            "name" => AuditLogChangeKeyKnown::Name,
            "description" => AuditLogChangeKeyKnown::Description,
            "icon_hash" => AuditLogChangeKeyKnown::IconHash,
            "image_hash" => AuditLogChangeKeyKnown::ImageHash,
            "splash_hash" => AuditLogChangeKeyKnown::SplashHash,
            "discovery_splash_hash" => AuditLogChangeKeyKnown::DiscoverySplashHash,
            "banner_hash" => AuditLogChangeKeyKnown::BannerHash,
            "owner_id" => AuditLogChangeKeyKnown::OwnerId,
            "region" => AuditLogChangeKeyKnown::Region,
            "preferred_locale" => AuditLogChangeKeyKnown::PreferredLocale,
            "afk_channel_id" => AuditLogChangeKeyKnown::AfkChannelId,
            "afk_timeout" => AuditLogChangeKeyKnown::AfkTimeout,
            "rules_channel_id" => AuditLogChangeKeyKnown::RulesChannelId,
            "public_updates_channel_id" => AuditLogChangeKeyKnown::PublicUpdatesChannelId,
            "safety_alerts_channel_id" => AuditLogChangeKeyKnown::SafetyAlertsChannelId,
            "mfa_level" => AuditLogChangeKeyKnown::MfaLevel,
            "verification_level" => AuditLogChangeKeyKnown::VerificationLevel,
            "explicit_content_filter" => AuditLogChangeKeyKnown::ExplicitContentFilter,
            "default_message_notifications" => AuditLogChangeKeyKnown::DefaultMessageNotifications,
            "vanity_url_code" => AuditLogChangeKeyKnown::VanityUrlCode,
            "premium_progress_bar_enabled" => AuditLogChangeKeyKnown::PremiumProgressBarEnabled,
            "$add" => AuditLogChangeKeyKnown::Add,
            "$remove" => AuditLogChangeKeyKnown::Remove,
            "prune_delete_days" => AuditLogChangeKeyKnown::PruneDeleteDays,
            "widget_enabled" => AuditLogChangeKeyKnown::WidgetEnabled,
            "widget_channel_id" => AuditLogChangeKeyKnown::WidgetChannelId,
            "system_channel_flags" => AuditLogChangeKeyKnown::SystemChannelFlags,
            "system_channel_id" => AuditLogChangeKeyKnown::SystemChannelId,
            "position" => AuditLogChangeKeyKnown::Position,
            "topic" => AuditLogChangeKeyKnown::Topic,
            "bitrate" => AuditLogChangeKeyKnown::Bitrate,
            "permission_overwrites" => AuditLogChangeKeyKnown::PermissionOverwrites,
            "nsfw" => AuditLogChangeKeyKnown::Nsfw,
            "application_id" => AuditLogChangeKeyKnown::ApplicationId,
            "rate_limit_per_user" => AuditLogChangeKeyKnown::RateLimitPerUser,
            "recurrence_rule" => AuditLogChangeKeyKnown::RecurrenceRule,
            "permissions" => AuditLogChangeKeyKnown::Permissions,
            "color" => AuditLogChangeKeyKnown::Color,
            "id" => AuditLogChangeKeyKnown::ChangedEntityId,
            "hoist" => AuditLogChangeKeyKnown::Hoist,
            "mentionable" => AuditLogChangeKeyKnown::Mentionable,
            "allow" => AuditLogChangeKeyKnown::Allow,
            "deny" => AuditLogChangeKeyKnown::Deny,
            "code" => AuditLogChangeKeyKnown::Code,
            "channel_id" => AuditLogChangeKeyKnown::ChannelId,
            "inviter_id" => AuditLogChangeKeyKnown::InviterId,
            "max_uses" => AuditLogChangeKeyKnown::MaxUses,
            "uses" => AuditLogChangeKeyKnown::Uses,
            "max_age" => AuditLogChangeKeyKnown::MaxAge,
            "temporary" => AuditLogChangeKeyKnown::Temporary,
            "deaf" => AuditLogChangeKeyKnown::Deaf,
            "mute" => AuditLogChangeKeyKnown::Mute,
            "nick" => AuditLogChangeKeyKnown::Nick,
            "avatar_hash" => AuditLogChangeKeyKnown::AvatarHash,
            "type" => AuditLogChangeKeyKnown::Type,
            "enable_emoticons" => AuditLogChangeKeyKnown::EnableEmoticons,
            "expire_behavior" => AuditLogChangeKeyKnown::ExpireBehavior,
            "expire_grace_period" => AuditLogChangeKeyKnown::ExpireGracePeriod,
            "user_limit" => AuditLogChangeKeyKnown::UserLimit,
            "privacy_level" => AuditLogChangeKeyKnown::PrivacyLevel,
            "tags" => AuditLogChangeKeyKnown::Tags,
            "format_type" => AuditLogChangeKeyKnown::FormatType,
            "asset" => AuditLogChangeKeyKnown::Asset,
            "available" => AuditLogChangeKeyKnown::Available,
            "guild_id" => AuditLogChangeKeyKnown::GuildId,
            "archived" => AuditLogChangeKeyKnown::Archived,
            "locked" => AuditLogChangeKeyKnown::Locked,
            "auto_archive_duration" => AuditLogChangeKeyKnown::AutoArchiveDuration,
            "default_auto_archive_duration" => AuditLogChangeKeyKnown::DefaultAutoArchiveDuration,
            "entity_type" => AuditLogChangeKeyKnown::EntityType,
            "status" => AuditLogChangeKeyKnown::Status,
            "location" => AuditLogChangeKeyKnown::Location,
            "communication_disabled_until" => AuditLogChangeKeyKnown::CommunicationDisabledUntil,
            "trigger_type" => AuditLogChangeKeyKnown::TriggerType,
            "event_type" => AuditLogChangeKeyKnown::EventType,
            "trigger_metadata" => AuditLogChangeKeyKnown::TriggerMetadata,
            "actions" => AuditLogChangeKeyKnown::Actions,
            "enabled" => AuditLogChangeKeyKnown::Enabled,
            "exempt_roles" => AuditLogChangeKeyKnown::ExemptRoles,
            "exempt_channels" => AuditLogChangeKeyKnown::ExemptChannels,
            "available_tags" => AuditLogChangeKeyKnown::AvailableTags,
            "default_reaction_emoji" => AuditLogChangeKeyKnown::DefaultReactionEmoji,
            "flags" => AuditLogChangeKeyKnown::Flags,
            "default_thread_rate_limit_per_user" => {
                AuditLogChangeKeyKnown::DefaultThreadRateLimitPerUser
            }
            "sound_id" => AuditLogChangeKeyKnown::SoundId,
            "volume" => AuditLogChangeKeyKnown::Volume,
            "emoji_id" => AuditLogChangeKeyKnown::EmojiId,
            "emoji_name" => AuditLogChangeKeyKnown::EmojiName,
            "user_id" => AuditLogChangeKeyKnown::UserId,
            #[allow(unreachable_patterns)]
            "preferred_locale" => AuditLogChangeKeyKnown::PreferredLocaleKey,
            _ => return Err(D::Error::custom("unknown audit log change key")),
        })
    }
}

/// Exhaustive list of known keys. Keeping this separate simplifies (de)serialization.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditLogChangeKeyKnown {
    Name,
    Description,
    IconHash,
    ImageHash,
    SplashHash,
    DiscoverySplashHash,
    BannerHash,
    OwnerId,
    Region,
    PreferredLocale,
    AfkChannelId,
    AfkTimeout,
    RulesChannelId,
    PublicUpdatesChannelId,
    SafetyAlertsChannelId,
    MfaLevel,
    VerificationLevel,
    ExplicitContentFilter,
    DefaultMessageNotifications,
    VanityUrlCode,
    PremiumProgressBarEnabled,
    Add,
    Remove,
    PruneDeleteDays,
    WidgetEnabled,
    WidgetChannelId,
    SystemChannelFlags,
    SystemChannelId,
    Position,
    Topic,
    Bitrate,
    PermissionOverwrites,
    Nsfw,
    ApplicationId,
    RateLimitPerUser,
    RecurrenceRule,
    Permissions,
    Color,
    /// Snowflake-typed generic key (command permission change case); see docs.
    Snowflake,
    Hoist,
    Mentionable,
    Allow,
    Deny,
    Code,
    ChannelId,
    InviterId,
    MaxUses,
    Uses,
    MaxAge,
    Temporary,
    Deaf,
    Mute,
    Nick,
    AvatarHash,
    ChangedEntityId,
    Type,
    EnableEmoticons,
    ExpireBehavior,
    ExpireGracePeriod,
    UserLimit,
    PrivacyLevel,
    Tags,
    FormatType,
    Asset,
    Available,
    GuildId,
    Archived,
    Locked,
    AutoArchiveDuration,
    DefaultAutoArchiveDuration,
    EntityType,
    Status,
    Location,
    CommunicationDisabledUntil,
    TriggerType,
    EventType,
    TriggerMetadata,
    Actions,
    Enabled,
    ExemptRoles,
    ExemptChannels,
    AvailableTags,
    DefaultReactionEmoji,
    Flags,
    DefaultThreadRateLimitPerUser,
    SoundId,
    Volume,
    EmojiId,
    EmojiName,
    UserId,
    /// Duplicate for clarity when the changed key is the guild's preferred locale.
    PreferredLocaleKey,
}

// -------------------------------------------------------------------------------------------------
// Extra: strongly-typed helpers (optional)
// -------------------------------------------------------------------------------------------------

impl APIAuditLogChange {
    /// Convenience helper: try to view this change as a string-to-string modification.
    /// Useful for common keys like `name`, `description`, etc.
    pub fn as_string_change(&self) -> Option<(Option<String>, Option<String>)> {
        use serde_json::Value;
        let to_string = |v: &Value| match v {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            _ => None,
        };
        let new_s = self.new_value.as_ref().and_then(to_string);
        let old_s = self.old_value.as_ref().and_then(to_string);
        if new_s.is_some() || old_s.is_some() {
            Some((new_s, old_s))
        } else {
            None
        }
    }
}
