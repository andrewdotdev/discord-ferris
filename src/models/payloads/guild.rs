// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::payloads::emoji::{APIEmoji, APIPartialEmoji};
use crate::models::payloads::gateway::PresenceUpdateReceiveStatus;
use crate::models::payloads::oauth2::OAuth2Scopes;
use crate::models::payloads::permissions::APIRole;
use crate::models::payloads::sticker::APISticker;
use crate::models::payloads::user::{APIAvatarDecorationData, APIUser};
use crate::models::rest::common::Locale;

/**
 * Types extracted from https://discord.com/developers/docs/resources/guild
 */

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIBaseGuild {
    /**
     * Guild id
     */
    pub id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#unavailable-guild-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIUnavailableGuild {
    #[serde(flatten)]
    pub base: APIBaseGuild,
    /**
     * `true` if this guild is unavailable due to an outage
     */
    pub unavailable: bool,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPartialGuild {
    #[serde(flatten)]
    pub base: APIBaseGuild,
    /**
     * Guild name (2-100 characters, excluding trailing and leading whitespace)
     */
    pub name: String,
    /**
     * Icon hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub icon: Option<String>,
    /**
     * Splash hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub splash: Option<String>,
    /**
     * Banner hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub banner: Option<Option<String>>,
    /**
     * The description for the guild
     */
    pub description: Option<Option<String>>,
    /**
     * Enabled guild features
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-features}
     */
    pub features: Option<Vec<GuildFeature>>,
    /**
     * Verification level required for the guild
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-verification-level}
     */
    pub verification_level: Option<GuildVerificationLevel>,
    /**
     * The vanity url code for the guild
     */
    pub vanity_url_code: Option<Option<String>>,
    /**
     * The welcome screen of a Community guild, shown to new members
     *
     * Returned in the invite object
     */
    pub welcome_screen: Option<APIGuildWelcomeScreen>,
}

/**
 * Source guild of channel follower webhooks.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIWebhookSourceGuild {
    pub icon: Option<String>,
    pub id: String,
    pub name: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuild {
    /**
     * Icon hash, returned when in the template object
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub icon_hash: Option<Option<String>>,
    /**
     * Discovery splash hash; only present for guilds with the "DISCOVERABLE" feature
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub discovery_splash: Option<String>,
    /**
     * `true` if the user is the owner of the guild
     *
     * **This field is only received from https://discord.com/developers/docs/resources/user#get-current-user-guilds**
     */
    pub owner: Option<bool>,
    /**
     * ID of owner
     */
    pub owner_id: String,
    /**
     * Total permissions for the user in the guild (excludes overrides)
     *
     * **This field is only received from https://discord.com/developers/docs/resources/user#get-current-user-guilds**
     *
     * @see {@link https://en.wikipedia.org/wiki/Bit_field}
     */
    pub permissions: Option<String>,
    /**
     * Voice region id for the guild
     *
     * @see {@link https://discord.com/developers/docs/resources/voice#voice-region-object}
     * @deprecated This field has been deprecated in favor of `rtc_region` on the channel.
     */
    pub region: String,
    /**
     * ID of afk channel
     */
    pub afk_channel_id: Option<String>,
    /**
     * afk timeout in seconds, can be set to: `60`, `300`, `900`, `1800`, `3600`
     */
    pub afk_timeout: i64,
    /**
     * `true` if the guild widget is enabled
     */
    pub widget_enabled: Option<bool>,
    /**
     * The channel id that the widget will generate an invite to, or `null` if set to no invite
     */
    pub widget_channel_id: Option<Option<String>>,
    /**
     * Verification level required for the guild
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-verification-level}
     */
    pub verification_level: GuildVerificationLevel,
    /**
     * Default message notifications level
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-default-message-notification-level}
     */
    pub default_message_notifications: GuildDefaultMessageNotifications,
    /**
     * Explicit content filter level
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-explicit-content-filter-level}
     */
    pub explicit_content_filter: GuildExplicitContentFilter,
    /**
     * Roles in the guild
     *
     * @see {@link https://discord.com/developers/docs/topics/permissions#role-object}
     */
    pub roles: Vec<APIRole>,
    /**
     * Custom guild emojis
     *
     * @see {@link https://discord.com/developers/docs/resources/emoji#emoji-object}
     */
    pub emojis: Vec<APIEmoji>,
    /**
     * Enabled guild features
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-features}
     */
    pub features: Vec<GuildFeature>,
    /**
     * Required MFA level for the guild
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-mfa-level}
     */
    pub mfa_level: GuildMFALevel,
    /**
     * Application id of the guild creator if it is bot-created
     */
    pub application_id: Option<String>,
    /**
     * The id of the channel where guild notices such as welcome messages and boost events are posted
     */
    pub system_channel_id: Option<String>,
    /**
     * System channel flags
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-system-channel-flags}
     */
    #[serde(with = "crate::utils::serde::flags_numeric")]
    pub system_channel_flags: GuildSystemChannelFlags,
    /**
     * The id of the channel where Community guilds can display rules and/or guidelines
     */
    pub rules_channel_id: Option<String>,
    /**
     * The maximum number of presences for the guild (`null` is always returned, apart from the largest of guilds)
     */
    pub max_presences: Option<Option<i64>>,
    /**
     * The maximum number of members for the guild
     */
    pub max_members: Option<i64>,
    /**
     * The vanity url code for the guild
     */
    pub vanity_url_code: Option<String>,
    /**
     * The description for the guild
     */
    pub description: Option<String>,
    /**
     * Banner hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub banner: Option<String>,
    /**
     * Premium tier (Server Boost level)
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-premium-tier}
     */
    pub premium_tier: GuildPremiumTier,
    /**
     * The number of boosts this guild currently has
     */
    pub premium_subscription_count: Option<i64>,
    /**
     * The preferred locale of a Community guild; used in guild discovery and notices from Discord
     *
     * @defaultValue `"en-US"`
     */
    pub preferred_locale: Locale,
    /**
     * The id of the channel where admins and moderators of Community guilds receive notices from Discord
     */
    pub public_updates_channel_id: Option<String>,
    /**
     * The maximum amount of users in a video channel
     */
    pub max_video_channel_users: Option<i64>,
    /**
     * The maximum amount of users in a stage video channel
     */
    pub max_stage_video_channel_users: Option<i64>,
    /**
     * Approximate number of members in this guild,
     * returned from the `GET /guilds/<id>` and `/users/@me/guilds` (OAuth2) endpoints when `with_counts` is `true`
     */
    pub approximate_member_count: Option<i64>,
    /**
     * Approximate number of non-offline members in this guild,
     * returned from the `GET /guilds/<id>` and `/users/@me/guilds` (OAuth2) endpoints when `with_counts` is `true`
     */
    pub approximate_presence_count: Option<i64>,
    /**
     * The nsfw level of the guild
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-nsfw-level}
     */
    pub nsfw_level: GuildNSFWLevel,
    /**
     * Custom guild stickers
     *
     * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-object}
     */
    pub stickers: Vec<APISticker>,
    /**
     * Whether the guild has the boost progress bar enabled.
     */
    pub premium_progress_bar_enabled: bool,
    /**
     * The type of Student Hub the guild is
     */
    pub hub_type: Option<GuildHubType>,
    /**
     * The id of the channel where admins and moderators of Community guilds receive safety alerts from Discord
     */
    pub safety_alerts_channel_id: Option<String>,
    /**
     * The incidents data for this guild
     */
    pub incidents_data: Option<APIIncidentsData>,

    // fields inherited from APIPartialGuild
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub id: String,
    pub banner_partial: Option<Option<String>>,
    pub description_partial: Option<Option<String>>,
    pub features_partial: Option<Vec<GuildFeature>>,
    pub verification_level_partial: Option<GuildVerificationLevel>,
    pub vanity_url_code_partial: Option<Option<String>>,
    pub welcome_screen: Option<APIGuildWelcomeScreen>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPartialInteractionGuild {
    #[serde(flatten)]
    pub base: APIBaseGuild,
    /**
     * Enabled guild features
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-features}
     */
    pub features: Vec<GuildFeature>,
    /**
     * The preferred locale of a Community guild; used in guild discovery and notices from Discord
     *
     * @unstable https://github.com/discord/discord-api-docs/issues/6938
     * @defaultValue `"en-US"`
     */
    pub locale: Locale,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-default-message-notification-level}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildDefaultMessageNotifications {
    AllMessages = 0,
    OnlyMentions = 1,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-explicit-content-filter-level}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildExplicitContentFilter {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-mfa-level}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildMFALevel {
    None = 0,
    Elevated = 1,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-nsfw-level}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildNSFWLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-verification-level}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildVerificationLevel {
    /**
     * Unrestricted
     */
    None = 0,
    /**
     * Must have verified email on account
     */
    Low = 1,
    /**
     * Must be registered on Discord for longer than 5 minutes
     */
    Medium = 2,
    /**
     * Must be a member of the guild for longer than 10 minutes
     */
    High = 3,
    /**
     * Must have a verified phone number
     */
    VeryHigh = 4,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-premium-tier}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildPremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildHubType {
    Default = 0,
    HighSchool = 1,
    College = 2,
}

bitflags! {
    /**
    * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-system-channel-flags}
    */
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
    pub struct GuildSystemChannelFlags: u32 {
        /**
         * Suppress member join notifications
         */
        const SuppressJoinNotifications = 1 << 0;
        /**
         * Suppress server boost notifications
         */
        const SuppressPremiumSubscriptions = 1 << 1;
        /**
         * Suppress server setup tips
         */
        const SuppressGuildReminderNotifications = 1 << 2;
        /**
         * Hide member join sticker reply buttons
         */
        const SuppressJoinNotificationReplies = 1 << 3;
        /**
         * Suppress role subscription purchase and renewal notifications
         */
        const SuppressRoleSubscriptionPurchaseNotifications = 1 << 4;
        /**
         * Hide role subscription sticker reply buttons
         */
        const SuppressRoleSubscriptionPurchaseNotificationReplies = 1 << 5;
    }
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-features}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuildFeature {
    /**
     * Guild has access to set an animated guild banner image
     */
    #[serde(rename = "ANIMATED_BANNER")]
    AnimatedBanner,
    /**
     * Guild has access to set an animated guild icon
     */
    #[serde(rename = "ANIMATED_ICON")]
    AnimatedIcon,
    /**
     * Guild is using the old permissions configuration behavior
     *
     * @see {@link https://discord.com/developers/docs/change-log#upcoming-application-command-permission-changes}
     */
    #[serde(rename = "APPLICATION_COMMAND_PERMISSIONS_V2")]
    ApplicationCommandPermissionsV2,
    /**
     * Guild has set up auto moderation rules
     */
    #[serde(rename = "AUTO_MODERATION")]
    AutoModeration,
    /**
     * Guild has access to set a guild banner image
     */
    #[serde(rename = "BANNER")]
    Banner,
    /**
     * Guild can enable welcome screen, Membership Screening and discovery, and receives community updates
     */
    #[serde(rename = "COMMUNITY")]
    Community,
    /**
     * Guild has enabled monetization
     */
    #[serde(rename = "CREATOR_MONETIZABLE_PROVISIONAL")]
    CreatorMonetizableProvisional,
    /**
     * Guild has enabled the role subscription promo page
     */
    #[serde(rename = "CREATOR_STORE_PAGE")]
    CreatorStorePage,
    /**
     * Guild has been set as a support server on the App Directory
     */
    #[serde(rename = "DEVELOPER_SUPPORT_SERVER")]
    DeveloperSupportServer,
    /**
     * Guild is able to be discovered in the directory
     */
    #[serde(rename = "DISCOVERABLE")]
    Discoverable,
    /**
     * Guild is able to be featured in the directory
     */
    #[serde(rename = "FEATURABLE")]
    Featurable,
    /**
     * Guild is listed in a directory channel
     */
    #[serde(rename = "HAS_DIRECTORY_ENTRY")]
    HasDirectoryEntry,
    /**
     * Guild is a Student Hub
     *
     * @see {@link https://support.discord.com/hc/articles/4406046651927}
     * @unstable This feature is currently not documented by Discord, but has known value
     */
    #[serde(rename = "HUB")]
    Hub,
    /**
     * Guild has disabled invite usage, preventing users from joining
     */
    #[serde(rename = "INVITES_DISABLED")]
    InvitesDisabled,
    /**
     * Guild has access to set an invite splash background
     */
    #[serde(rename = "INVITE_SPLASH")]
    InviteSplash,
    /**
     * Guild is in a Student Hub
     *
     * @see {@link https://support.discord.com/hc/articles/4406046651927}
     * @unstable This feature is currently not documented by Discord, but has known value
     */
    #[serde(rename = "LINKED_TO_HUB")]
    LinkedToHub,
    /**
     * Guild has enabled Membership Screening
     */
    #[serde(rename = "MEMBER_VERIFICATION_GATE_ENABLED")]
    MemberVerificationGateEnabled,
    /**
     * Guild has increased custom soundboard sound slots
     */
    #[serde(rename = "MORE_SOUNDBOARD")]
    MoreSoundboard,
    /**
     * Guild has enabled monetization
     *
     * @unstable This feature is no longer documented by Discord
     */
    #[serde(rename = "MONETIZATION_ENABLED")]
    MonetizationEnabled,
    /**
     * Guild has increased custom sticker slots
     */
    #[serde(rename = "MORE_STICKERS")]
    MoreStickers,
    /**
     * Guild has access to create news channels
     */
    #[serde(rename = "NEWS")]
    News,
    /**
     * Guild is partnered
     */
    #[serde(rename = "PARTNERED")]
    Partnered,
    /**
     * Guild can be previewed before joining via Membership Screening or the directory
     */
    #[serde(rename = "PREVIEW_ENABLED")]
    PreviewEnabled,
    /**
     * Guild has access to create private threads
     */
    #[serde(rename = "PRIVATE_THREADS")]
    PrivateThreads,
    /**
     * Guild has disabled alerts for join raids in the configured safety alerts channel
     */
    #[serde(rename = "RAID_ALERTS_DISABLED")]
    RaidAlertsDisabled,
    #[serde(rename = "RELAY_ENABLED")]
    RelayEnabled,
    /**
     * Guild is able to set role icons
     */
    #[serde(rename = "ROLE_ICONS")]
    RoleIcons,
    /**
     * Guild has role subscriptions that can be purchased
     */
    #[serde(rename = "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE")]
    RoleSubscriptionsAvailableForPurchase,
    /**
     * Guild has enabled role subscriptions
     */
    #[serde(rename = "ROLE_SUBSCRIPTIONS_ENABLED")]
    RoleSubscriptionsEnabled,
    /**
     * Guild has created soundboard sounds
     */
    #[serde(rename = "SOUNDBOARD")]
    Soundboard,
    /**
     * Guild has enabled ticketed events
     */
    #[serde(rename = "TICKETED_EVENTS_ENABLED")]
    TicketedEventsEnabled,
    /**
     * Guild has access to set a vanity URL
     */
    #[serde(rename = "VANITY_URL")]
    VanityURL,
    /**
     * Guild is verified
     */
    #[serde(rename = "VERIFIED")]
    Verified,
    /**
     * Guild has access to set 384kbps bitrate in voice (previously VIP voice servers)
     */
    #[serde(rename = "VIP_REGIONS")]
    VIPRegions,
    /**
     * Guild has enabled the welcome screen
     */
    #[serde(rename = "WELCOME_SCREEN_ENABLED")]
    WelcomeScreenEnabled,
    /**
     * Guild has access to set guild tags
     */
    #[serde(rename = "GUILD_TAGS")]
    GuildTags,
    /**
     * Guild is able to set gradient colors to roles
     */
    #[serde(rename = "ENHANCED_ROLE_COLORS")]
    EnhancedRoleColors,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-preview-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildPreview {
    /**
     * Guild id
     */
    pub id: String,
    /**
     * Guild name (2-100 characters)
     */
    pub name: String,
    /**
     * Icon hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub icon: Option<String>,
    /**
     * Splash hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub splash: Option<String>,
    /**
     * Discovery splash hash; only present for guilds with the "DISCOVERABLE" feature
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub discovery_splash: Option<String>,
    /**
     * Custom guild emojis
     *
     * @see {@link https://discord.com/developers/docs/resources/emoji#emoji-object}
     */
    pub emojis: Vec<APIEmoji>,
    /**
     * Enabled guild features
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-features}
     */
    pub features: Vec<GuildFeature>,
    /**
     * Approximate number of members in this guild
     */
    pub approximate_member_count: i64,
    /**
     * Approximate number of online members in this guild
     */
    pub approximate_presence_count: i64,
    /**
     * The description for the guild
     */
    pub description: String,
    /**
     * Custom guild stickers
     */
    pub stickers: Vec<APISticker>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-widget-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildWidgetSettings {
    /**
     * Whether the widget is enabled
     */
    pub enabled: bool,
    /**
     * The widget channel id
     */
    pub channel_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBaseGuildMember {
    /**
     * This users guild nickname
     */
    pub nick: Option<Option<String>>,
    /**
     * Array of role object ids
     *
     * @see {@link https://discord.com/developers/docs/topics/permissions#role-object}
     */
    pub roles: Vec<String>,
    /**
     * When the user started boosting the guild
     *
     * @see {@link https://support.discord.com/hc/articles/360028038352}
     */
    pub premium_since: Option<Option<String>>,
    /**
     * Whether the user has not yet passed the guild's Membership Screening requirements
     *
     * @remarks If this field is not present, it can be assumed as `false`.
     */
    pub pending: Option<bool>,
    /**
     * Timestamp of when the time out will be removed; until then, they cannot interact with the guild
     */
    pub communication_disabled_until: Option<Option<String>>,
    /**
     * The data for the member's guild avatar decoration
     *
     * @see {@link https://discord.com/developers/docs/resources/user#avatar-decoration-data-object}
     */
    pub avatar_decoration_data: Option<Option<APIAvatarDecorationData>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIFlaggedGuildMember {
    /**
     * Guild member flags represented as a bit set
     *
     * @defaultValue `0`
     */
    #[serde(with = "crate::utils::serde::flags_numeric_opt")]
    pub flags: Option<GuildMemberFlags>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildMemberJoined {
    /**
     * When the user joined the guild
     */
    pub joined_at: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildMemberAvatar {
    /**
     * The member's guild avatar hash
     */
    pub avatar: Option<Option<String>>,
    /**
     * The member's guild banner hash
     */
    pub banner: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIBaseVoiceGuildMember {
    /**
     * Whether the user is deafened in voice channels
     */
    pub deaf: bool,
    /**
     * Whether the user is muted in voice channels
     */
    pub mute: bool,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildMemberUser {
    /**
     * The user this guild member represents
     *
     * **This field won't be included in the member object attached to `MESSAGE_CREATE` and `MESSAGE_UPDATE` gateway events.**
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub user: APIUser,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildMember {
    #[serde(flatten)]
    pub base: APIBaseGuildMember,
    #[serde(flatten)]
    pub voice: APIBaseVoiceGuildMember,
    #[serde(flatten)]
    pub flagged: APIFlaggedGuildMember,
    #[serde(flatten)]
    pub avatar: APIGuildMemberAvatar,
    #[serde(flatten)]
    pub joined: APIGuildMemberJoined,
    #[serde(flatten)]
    pub user: APIGuildMemberUser,
}

bitflags! {
    /**
    * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object-guild-member-flags}
    */
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct GuildMemberFlags: u32 {
        /**
         * Member has left and rejoined the guild
         */
        const DidRejoin = 1 << 0;
        /**
         * Member has completed onboarding
         */
        const CompletedOnboarding = 1 << 1;
        /**
         * Member is exempt from guild verification requirements
         */
        const BypassesVerification = 1 << 2;
        /**
         * Member has started onboarding
         */
        const StartedOnboarding = 1 << 3;
        /**
         * Member is a guest and can only access the voice channel they were invited to
         */
        const IsGuest = 1 << 4;
        /**
         * Member has started Server Guide new member actions
         */
        const StartedHomeActions = 1 << 5;
        /**
         * Member has completed Server Guide new member actions
         */
        const CompletedHomeActions = 1 << 6;
        /**
         * Member's username, display name, or nickname is blocked by AutoMod
         */
        const AutomodQuarantinedUsernameOrGuildNickname = 1 << 7;
        /**
         * @deprecated
         * {@link https://github.com/discord/discord-api-docs/pull/7113 | discord-api-docs#7113}
         */
        const AutomodQuarantinedBio = 1 << 8;
        /**
         * Member has dismissed the DM settings upsell
         */
        const DmSettingsUpsellAcknowledged = 1 << 9;
        /**
         * Member's guild tag is blocked by AutoMod
         */
        const AutoModQuarantinedGuildTag = 1 << 10;
    }
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#integration-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildIntegration {
    /**
     * Integration id
     */
    pub id: String,
    /**
     * Integration name
     */
    pub name: String,
    /**
     * Integration type
     */
    pub r#type: APIGuildIntegrationType,
    /**
     * Is this integration enabled
     */
    pub enabled: bool,
    /**
     * Is this integration syncing
     *
     * **This field is not provided for `discord` bot integrations.**
     */
    pub syncing: Option<bool>,
    /**
     * ID that this integration uses for "subscribers"
     *
     * **This field is not provided for `discord` bot integrations.**
     */
    pub role_id: Option<String>,
    /**
     * Whether emoticons should be synced for this integration (`twitch` only currently)
     *
     * **This field is not provided for `discord` bot integrations.**
     */
    pub enable_emoticons: Option<bool>,
    /**
     * The behavior of expiring subscribers
     *
     * **This field is not provided for `discord` bot integrations.**
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#integration-object-integration-expire-behaviors}
     */
    pub expire_behavior: Option<IntegrationExpireBehavior>,
    /**
     * The grace period (in days) before expiring subscribers
     *
     * **This field is not provided for `discord` bot integrations.**
     */
    pub expire_grace_period: Option<i64>,
    /**
     * User for this integration
     *
     * **Some older integrations may not have an attached user.**
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub user: Option<APIUser>,
    /**
     * Integration account information
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#integration-account-object}
     */
    pub account: APIIntegrationAccount,
    /**
     * When this integration was last synced
     *
     * **This field is not provided for `discord` bot integrations.**
     */
    pub synced_at: Option<String>,
    /**
     * How many subscribers this integration has
     *
     * **This field is not provided for `discord` bot integrations.**
     */
    pub subscriber_count: Option<i64>,
    /**
     * Has this integration been revoked
     *
     * **This field is not provided for `discord` bot integrations.**
     */
    pub revoked: Option<bool>,
    /**
     * The bot/OAuth2 application for discord integrations
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#integration-application-object}
     *
     * **This field is not provided for `discord` bot integrations.**
     */
    pub application: Option<APIGuildIntegrationApplication>,
    /**
     * The scopes the application has been authorized for
     */
    pub scopes: Option<Vec<OAuth2Scopes>>,
}

pub type APIGuildIntegrationType = APIGuildIntegrationTypeEnum;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum APIGuildIntegrationTypeEnum {
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "guild_subscription")]
    GuildSubscription,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "youtube")]
    Youtube,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#integration-object-integration-expire-behaviors}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IntegrationExpireBehavior {
    RemoveRole = 0,
    Kick = 1,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#integration-account-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIIntegrationAccount {
    /**
     * ID of the account
     */
    pub id: String,
    /**
     * Name of the account
     */
    pub name: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#integration-application-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildIntegrationApplication {
    /**
     * The id of the app
     */
    pub id: String,
    /**
     * The name of the app
     */
    pub name: String,
    /**
     * The icon hash of the app
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub icon: Option<String>,
    /**
     * The description of the app
     */
    pub description: String,
    /**
     * The bot associated with this application
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub bot: Option<APIUser>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#ban-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBan {
    /**
     * The reason for the ban
     */
    pub reason: Option<String>,
    /**
     * The banned user
     */
    pub user: APIUser,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-widget-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildWidget {
    pub id: String,
    pub name: String,
    pub instant_invite: Option<String>,
    pub channels: Vec<APIGuildWidgetChannel>,
    pub members: Vec<APIGuildWidgetMember>,
    pub presence_count: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-widget-object-example-guild-widget}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildWidgetChannel {
    pub id: String,
    pub name: String,
    pub position: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-widget-object-example-guild-widget}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildWidgetMember {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub status: PresenceUpdateReceiveStatus,
    pub activity: Option<APIGuildWidgetMemberActivity>,
    pub avatar_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildWidgetMemberActivity {
    pub name: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-widget-image-widget-style-options}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuildWidgetStyle {
    /**
     * Shield style widget with Discord icon and guild members online count
     */
    #[serde(rename = "shield")]
    Shield,
    /**
     * Large image with guild icon, name and online count. "POWERED BY DISCORD" as the footer of the widget
     */
    #[serde(rename = "banner1")]
    Banner1,
    /**
     * Smaller widget style with guild icon, name and online count. Split on the right with Discord logo
     */
    #[serde(rename = "banner2")]
    Banner2,
    /**
     * Large image with guild icon, name and online count. In the footer, Discord logo on the left and "Chat Now" on the right
     */
    #[serde(rename = "banner3")]
    Banner3,
    /**
     * Large Discord logo at the top of the widget. Guild icon, name and online count in the middle portion of the widget
     * and a "JOIN MY SERVER" button at the bottom
     */
    #[serde(rename = "banner4")]
    Banner4,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildWelcomeScreen {
    /**
     * The welcome screen short message
     */
    pub description: Option<String>,
    /**
     * Array of suggested channels
     */
    pub welcome_channels: Vec<APIGuildWelcomeScreenChannel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildWelcomeScreenChannel {
    /**
     * The channel id that is suggested
     */
    pub channel_id: String,
    /**
     * The description shown for the channel
     */
    pub description: String,
    /**
     * The emoji id of the emoji that is shown on the left of the channel
     */
    pub emoji_id: Option<String>,
    /**
     * The emoji name of the emoji that is shown on the left of the channel
     */
    pub emoji_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildMembershipScreening {
    /**
     * When the fields were last updated
     */
    pub version: String,
    /**
     * The steps in the screening form
     */
    pub form_fields: Vec<APIGuildMembershipScreeningField>,
    /**
     * The server description shown in the screening form
     */
    pub description: Option<String>,
}

// TODO: make this a union based on the type in the future, when new types are added

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildMembershipScreeningField {
    /**
     * The type of field
     */
    pub field_type: MembershipScreeningFieldType,
    /**
     * The title of the field
     */
    pub label: String,
    /**
     * The list of rules
     */
    pub values: Option<Vec<String>>,
    /**
     * Whether the user has to fill out this field
     */
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MembershipScreeningFieldType {
    /**
     * Server Rules
     */
    #[serde(rename = "TERMS")]
    Terms,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-onboarding-object-guild-onboarding-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildOnboarding {
    /**
     * Id of the guild this onboarding is part of
     */
    pub guild_id: String,
    /**
     * Prompts shown during onboarding and in customize community
     */
    pub prompts: Vec<APIGuildOnboardingPrompt>,
    /**
     * Channel ids that members get opted into automatically
     */
    pub default_channel_ids: Vec<String>,
    /**
     * Whether onboarding is enabled in the guild
     */
    pub enabled: bool,
    /**
     * Current mode of onboarding
     */
    pub mode: GuildOnboardingMode,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-onboarding-object-onboarding-prompt-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildOnboardingPrompt {
    /**
     * Id of the prompt
     */
    pub id: String,
    /**
     * Options available within the prompt
     */
    pub options: Vec<APIGuildOnboardingPromptOption>,
    /**
     * Title of the prompt
     */
    pub title: String,
    /**
     * Indicates whether users are limited to selecting one option for the prompt
     */
    pub single_select: bool,
    /**
     * Indicates whether the prompt is required before a user completes the onboarding flow
     */
    pub required: bool,
    /**
     * Indicates whether the prompt is present in the onboarding flow.
     * If `false`, the prompt will only appear in the Channels & Roles tab
     */
    pub in_onboarding: bool,
    /**
     * Type of prompt
     */
    pub r#type: GuildOnboardingPromptType,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-onboarding-object-prompt-option-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildOnboardingPromptOption {
    /**
     * Id of the prompt option
     */
    pub id: String,
    /**
     * Ids for channels a member is added to when the option is selected
     */
    pub channel_ids: Vec<String>,
    /**
     * Ids for roles assigned to a member when the option is selected
     */
    pub role_ids: Vec<String>,
    /**
     * Emoji of the option
     */
    pub emoji: APIPartialEmoji,
    /**
     * Title of the option
     */
    pub title: String,
    /**
     * Description of the option
     */
    pub description: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-onboarding-object-onboarding-mode}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildOnboardingMode {
    /**
     * Counts only Default Channels towards constraints
     */
    OnboardingDefault = 0,
    /**
     * Counts Default Channels and Questions towards constraints
     */
    OnboardingAdvanced = 1,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-onboarding-object-prompt-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildOnboardingPromptType {
    MultipleChoice = 0,
    Dropdown = 1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIIncidentsData {
    /**
     * When invites get enabled again
     */
    pub invites_disabled_until: Option<String>,
    /**
     * When direct messages get enabled again
     */
    pub dms_disabled_until: Option<String>,
    /**
     * When the dm spam was detected
     */
    pub dm_spam_detected_at: Option<Option<String>>,
    /**
     * When the raid was detected
     */
    pub raid_detected_at: Option<Option<String>>,
}
