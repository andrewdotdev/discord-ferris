use super::channel::RESTPutAPIChannelPermissionJSONBody;
use super::common::Locale;
use crate::models::payloads::{
    APIBan, APIChannel, APIExtendedInvite, APIGuild, APIGuildIntegration, APIGuildMember,
    APIGuildMembershipScreening, APIGuildOnboarding, APIGuildOnboardingPromptOption,
    APIGuildPreview, APIGuildWelcomeScreen, APIGuildWidget, APIGuildWidgetSettings, APIRole,
    APIRoleColors, APIThreadList, APIVoiceRegion, GuildDefaultMessageNotifications,
    GuildExplicitContentFilter, GuildFeature, GuildMFALevel, GuildSystemChannelFlags,
    GuildVerificationLevel, GuildWidgetStyle,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum NumberOrString {
    Number(u64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SnowflakeOrNumber {
    String(String),
    Number(u64),
}

pub type RESTAPIGuildChannelResolvable = APIChannel;

/**
 * @deprecated Use {@link RESTAPIGuildChannelResolvable} instead
 */
#[deprecated(note = "Use RESTAPIGuildChannelResolvable instead")]
pub type APIGuildChannelResolvable = RESTAPIGuildChannelResolvable;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIGuildCreateOverwrite {
    pub id: NumberOrString,
    #[serde(flatten)]
    pub perms: RESTPutAPIChannelPermissionJSONBody,
}

/**
 * @deprecated Use {@link RESTAPIGuildCreateOverwrite} instead
 */
#[deprecated(note = "Use RESTAPIGuildCreateOverwrite instead")]
pub type APIGuildCreateOverwrite = RESTAPIGuildCreateOverwrite;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIGuildCreatePartialChannel {
    pub name: String,
    pub id: Option<NumberOrString>,
    pub parent_id: Option<Option<NumberOrString>>,
    pub permission_overwrites: Option<Vec<RESTAPIGuildCreateOverwrite>>,

    // Pick<
    //  RESTAPIGuildChannelResolvable,
    //  'available_tags' | 'bitrate' | 'default_auto_archive_duration' | 'default_forum_layout'
    //  | 'default_reaction_emoji' | 'default_sort_order' | 'default_thread_rate_limit_per_user'
    //  | 'flags' | 'nsfw' | 'position' | 'rate_limit_per_user' | 'rtc_region' | 'topic' | 'type'
    //  | 'user_limit' | 'video_quality_mode'
    // >
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Vec<APIGuildOnboardingPromptOption>>, // placeholder: tags type lives under payloads (APIGuildForumTag). Use your actual type here if named differently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<u8>,
}

/**
 * @deprecated Use {@link RESTAPIGuildCreatePartialChannel} instead
 */
#[deprecated(note = "Use RESTAPIGuildCreatePartialChannel instead")]
pub type APIGuildCreatePartialChannel = RESTAPIGuildCreatePartialChannel;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIGuildCreateRole {
    pub id: NumberOrString,

    // Extends RESTPostAPIGuildRoleJSONBody
    /**
     * Name of the role
     *
     * @defaultValue `"new role"`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /**
     * Bitwise value of the enabled/disabled permissions
     *
     * @defaultValue
     * Default role permissions in guild
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Option<String>>,
    /**
     * RGB color value
     *
     * @defaultValue `0`
     * @remarks `color` will still be returned by the API, but using the `colors` field is recommended when doing requests.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<u32>>,
    /**
     * The role's colors
     *
     * @defaultValue `{ "primary_color": 0, "secondary_color": null, "tertiary_color": null }`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<APIRoleColors>,
    /**
     * Whether the role should be displayed separately in the sidebar
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<Option<bool>>,
    /**
     * The role's icon image (if the guild has the `ROLE_ICONS` feature)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    /**
     * The role's unicode emoji as a standard emoji (if the guild has the `ROLE_ICONS` feature)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<Option<String>>,
    /**
     * Whether the role should be mentionable
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<Option<bool>>,
}

/**
 * @deprecated Use {@link RESTAPIGuildCreateRole} instead
 */
#[deprecated(note = "Use RESTAPIGuildCreateRole instead")]
pub type APIGuildCreateRole = RESTAPIGuildCreateRole;

/**
 * @see {@link https://discord.com/developers/docs/change-log#guild-create-deprecation}
 * @deprecated
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIGuildsJSONBody {
    /**
     * Name of the guild (2-100 characters)
     */
    pub name: String,
    /**
     * Voice region id
     *
     * @see {@link https://discord.com/developers/docs/resources/voice#voice-region-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /**
     * base64 1024x1024 png/jpeg image for the guild icon
     *
     * @see {@link https://discord.com/developers/docs/reference#image-data}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /**
     * Verification level
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-verification-level}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<GuildVerificationLevel>,
    /**
     * Default message notification level
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-default-message-notification-level}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<GuildDefaultMessageNotifications>,
    /**
     * Explicit content filter level
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-explicit-content-filter-level}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<GuildExplicitContentFilter>,
    /**
     * New guild roles
     *
     * @remarks
     * When using this parameter, the first member of the array is used to change properties of the guild's `@everyone` role.
     * If you are trying to bootstrap a guild with additional roles, keep this in mind.
     *
     * Also, the required `id` field within each role object is an integer placeholder,
     * and will be replaced by the API upon consumption. Its purpose is to allow you to overwrite a role's permissions
     * in a channel when also passing in channels with the channels array.
     * @see {@link https://discord.com/developers/docs/topics/permissions#role-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RESTAPIGuildCreateRole>>,
    /**
     * New guild's channels
     *
     * @remarks
     * When using the channels parameter, the `position` field is ignored, and none of the default channels are created.
     *
     * Also, the `id` field within each channel object may be set to an integer placeholder,
     * and will be replaced by the API upon consumption. Its purpose is to allow you to create `GUILD_CATEGORY` channels
     * by setting the `parent_id` field on any children to the category's id field.
     * Category channels must be listed before any children.
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<RESTAPIGuildCreatePartialChannel>>,
    /**
     * ID for afk channel
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<Option<SnowflakeOrNumber>>,
    /**
     * afk timeout in seconds, can be set to: `60`, `300`, `900`, `1800`, `3600`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u32>,
    /**
     * The id of the channel where guild notices such as welcome messages and boost events are posted
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<Option<SnowflakeOrNumber>>,
    /**
     * System channel flags
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-system-channel-flags}
     */
    #[serde(with = "crate::utils::serde::flags_numeric")]
    // #[serde(with = "crate::utils::serde::flags_numeric")]
    pub system_channel_flags: GuildSystemChannelFlags,
    /**
     * Whether the boosts progress bar should be enabled.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_progress_bar_enabled: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/change-log#guild-create-deprecation}
 * @deprecated
 */
pub type RESTPostAPIGuildsResult = APIGuild;

/**
 * @see {@link https://discord.com/developers/docs/change-log#guild-create-deprecation}
 * @deprecated
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIGuildsMFAJSONBody {
    /**
     * MFA level
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-mfa-level}
     */
    pub level: GuildMFALevel,
}

/**
 * @see {@link https://discord.com/developers/docs/change-log#guild-create-deprecation}
 * @deprecated
 */
pub type RESTPostAPIGuildsMFAResult = RESTPostAPIGuildsMFAJSONBody;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIGuildQuery {
    /**
     * When `true`, will return approximate member and presence counts for the guild
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_counts: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild}
 */
pub type RESTGetAPIGuildResult = APIGuild;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-preview}
 */
pub type RESTGetAPIGuildPreviewResult = APIGuildPreview;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIGuildJSONBody {
    /**
     * New name for the guild (2-100 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**
     * Voice region id
     *
     * @see {@link https://discord.com/developers/docs/resources/voice#voice-region-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Option<String>>,
    /**
     * Verification level
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-verification-level}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<Option<GuildVerificationLevel>>,
    /**
     * Default message notification level
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-default-message-notification-level}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<Option<GuildDefaultMessageNotifications>>,
    /**
     * Explicit content filter level
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-explicit-content-filter-level}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<Option<GuildExplicitContentFilter>>,
    /**
     * ID for afk channel
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<Option<String>>,
    /**
     * afk timeout in seconds, can be set to: `60`, `300`, `900`, `1800`, `3600`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u32>,
    /**
     * base64 1024x1024 png/jpeg/gif image for the guild icon (can be animated gif when the guild has `ANIMATED_ICON` feature)
     *
     * @see {@link https://discord.com/developers/docs/reference#image-data}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    /**
     * User id to transfer guild ownership to (must be owner)
     *
     * @deprecated
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /**
     * base64 16:9 png/jpeg image for the guild splash (when the guild has `INVITE_SPLASH` feature)
     *
     * @see {@link https://discord.com/developers/docs/reference#image-data}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash: Option<Option<String>>,
    /**
     * base64 png/jpeg image for the guild discovery splash (when the guild has `DISCOVERABLE` feature)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_splash: Option<Option<String>>,
    /**
     * base64 16:9 png/jpeg image for the guild banner (when the server has the `BANNER` feature; can be animated gif when the server has the `ANIMATED_BANNER` feature)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Option<String>>,
    /**
     * The id of the channel where guild notices such as welcome messages and boost events are posted
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<Option<String>>,
    /**
     * System channel flags
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-system-channel-flags}
     */
    #[serde(with = "crate::utils::serde::flags_numeric")]
    pub system_channel_flags: GuildSystemChannelFlags,
    /**
     * The id of the channel where Community guilds display rules and/or guidelines
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<Option<String>>,
    /**
     * The id of the channel where admins and moderators of Community guilds receive notices from Discord
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: Option<Option<String>>,
    /**
     * The preferred locale of a Community guild used in server discovery and notices from Discord
     *
     * @defaultValue `"en-US"` (if the value is set to `null`)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<Option<Locale>>,
    /**
     * Enabled guild features
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object-guild-features}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<GuildFeature>>,
    /**
     * The description for the guild
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /**
     * Whether the boosts progress bar should be enabled.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_progress_bar_enabled: Option<bool>,
    /**
     * The id of the channel where admins and moderators of Community guilds receive safety alerts from Discord
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild}
 */
pub type RESTPatchAPIGuildResult = APIGuild;

/**
 * @see {@link https://discord.com/developers/docs/change-log#guild-create-deprecation}
 * @deprecated
 */
pub type RESTDeleteAPIGuildResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-channels}
 */
pub type RESTGetAPIGuildChannelsResult = Vec<APIChannel>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#create-guild-channel}
 */
pub type RESTPostAPIGuildChannelJSONBody = RESTAPIGuildCreatePartialChannel;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#create-guild-channel}
 */
pub type RESTPostAPIGuildChannelResult = APIChannel;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPatchAPIGuildChannelPositionsJSONBodyItem {
    /**
     * Channel id
     */
    pub id: String,
    /**
     * Sorting position of the channel
     */
    pub position: i32,
    /**
     * Sync channel overwrites with the new parent, when moving to a new `parent_id`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_permissions: Option<bool>,
    /**
     * The new parent id of this channel
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-channel-positions}
 */
pub type RESTPatchAPIGuildChannelPositionsJSONBody =
    Vec<RESTPatchAPIGuildChannelPositionsJSONBodyItem>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-channel-positions}
 */
pub type RESTPatchAPIGuildChannelPositionsResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#list-active-guild-threads}
 */
pub type RESTGetAPIGuildThreadsResult = APIThreadList;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-member}
 */
pub type RESTGetAPIGuildMemberResult = APIGuildMember;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#list-guild-members}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIGuildMembersQuery {
    /**
     * Max number of members to return (1-1000)
     *
     * @defaultValue `1`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /**
     * The highest user id in the previous page
     *
     * @defaultValue `0`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#list-guild-members}
 */
pub type RESTGetAPIGuildMembersResult = Vec<APIGuildMember>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#search-guild-members}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTGetAPIGuildMembersSearchQuery {
    /**
     * Query string to match username(s) and nickname(s) against
     */
    pub query: String,
    /**
     * Max number of members to return (1-1000)
     *
     * @defaultValue `1`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

pub type RESTGetAPIGuildMembersSearchResult = Vec<APIGuildMember>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#add-guild-member}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPutAPIGuildMemberJSONBody {
    /**
     * An oauth2 access token granted with the `guilds.join` to the bot's application for the user you want to add to the guild
     */
    pub access_token: String,
    /**
     * Value to set users nickname to
     *
     * Requires `MANAGE_NICKNAMES` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    /**
     * Array of role ids the member is assigned
     *
     * Requires `MANAGE_ROLES` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /**
     * Whether the user is muted in voice channels
     *
     * Requires `MUTE_MEMBERS` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    /**
     * Whether the user is deafened in voice channels
     *
     * Requires `DEAFEN_MEMBERS` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
}

pub type RESTPutAPIGuildMemberResult = Option<APIGuildMember>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-member}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIGuildMemberJSONBody {
    /**
     * Value to set users nickname to
     *
     * Requires `MANAGE_NICKNAMES` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<Option<String>>,
    /**
     * Array of role ids the member is assigned
     *
     * Requires `MANAGE_ROLES` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Option<Vec<String>>>,
    /**
     * Whether the user is muted in voice channels. Will throw a 400 if the user is not in a voice channel
     *
     * Requires `MUTE_MEMBERS` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<Option<bool>>,
    /**
     * Whether the user is deafened in voice channels. Will throw a 400 if the user is not in a voice channel
     *
     * Requires `DEAFEN_MEMBERS` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<Option<bool>>,
    /**
     * ID of channel to move user to (if they are connected to voice)
     *
     * Requires `MOVE_MEMBERS` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Option<String>>,
    /**
     * Timestamp of when the time out will be removed; until then, they cannot interact with the guild
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#add-guild-member}
 */
pub type RESTPatchAPIGuildMemberResult = APIGuildMember;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-current-user-nick}
 * @deprecated Use {@link https://discord.com/developers/docs/resources/guild#modify-current-member | Modify Current Member} instead.
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPICurrentGuildMemberNicknameJSONBody {
    /**
     * Value to set users nickname to
     *
     * Requires `CHANGE_NICKNAME` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-current-member}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPICurrentGuildMemberJSONBody {
    /**
     * Value to set users nickname to
     *
     * Requires `CHANGE_NICKNAME` permission
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-current-user-nick}
 * @deprecated Use {@link https://discord.com/developers/docs/resources/guild#modify-current-member | Modify Current Member} instead.
 */
pub type RESTPatchAPICurrentGuildMemberNicknameResult =
    RESTPatchAPICurrentGuildMemberNicknameJSONBody;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#add-guild-member-role}
 */
pub type RESTPutAPIGuildMemberRoleResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#remove-guild-member-role}
 */
pub type RESTDeleteAPIGuildMemberRoleResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#remove-guild-member}
 */
pub type RESTDeleteAPIGuildMemberResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-bans}
 */
pub type RESTGetAPIGuildBansResult = Vec<APIBan>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-bans}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIGuildBansQuery {
    /**
     * Consider only users before given user id
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /**
     * Consider only users after given user id
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /**
     * Number of users to return (1-1000)
     *
     * @defaultValue `1000`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-ban}
 */
pub type RESTGetAPIGuildBanResult = APIBan;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#create-guild-ban}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPutAPIGuildBanJSONBody {
    /**
     * Number of days to delete messages for (0-7)
     *
     * @deprecated Use {@link RESTPutAPIGuildBanJSONBody.delete_message_seconds} instead
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_message_days: Option<u8>,
    /**
     * Number of seconds to delete messages for, between 0 and 604800 (7 days)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_message_seconds: Option<u32>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#create-guild-ban}
 */
pub type RESTPutAPIGuildBanResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#remove-guild-ban}
 */
pub type RESTDeleteAPIGuildBanResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#bulk-guild-ban}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIGuildBulkBanJSONBody {
    /**
     * List of user ids to ban (max 200)
     */
    pub user_ids: Vec<String>,
    /**
     * Number of seconds to delete messages for, between 0 and 604800 (7 days)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_message_seconds: Option<u32>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#bulk-guild-ban}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIGuildBulkBanResult {
    /**
     * List of user ids, that were successfully banned
     */
    pub banned_users: Vec<String>,
    /**
     * List of user ids, that were not banned
     */
    pub failed_users: Vec<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-roles}
 */
pub type RESTGetAPIGuildRolesResult = Vec<APIRole>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#create-guild-role}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIGuildRoleJSONBody {
    /**
     * Name of the role
     *
     * @defaultValue `"new role"`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /**
     * Bitwise value of the enabled/disabled permissions
     *
     * @defaultValue
     * Default role permissions in guild
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Option<String>>,
    /**
     * RGB color value
     *
     * @defaultValue `0`
     * @remarks `color` will still be returned by the API, but using the `colors` field is recommended when doing requests.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<u32>>,
    /**
     * The role's colors
     *
     * @defaultValue `{ "primary_color": 0, "secondary_color": null, "tertiary_color": null }`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<APIRoleColors>,
    /**
     * Whether the role should be displayed separately in the sidebar
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<Option<bool>>,
    /**
     * The role's icon image (if the guild has the `ROLE_ICONS` feature)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    /**
     * The role's unicode emoji as a standard emoji (if the guild has the `ROLE_ICONS` feature)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<Option<String>>,
    /**
     * Whether the role should be mentionable
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<Option<bool>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#create-guild-role}
 */
pub type RESTPostAPIGuildRoleResult = APIRole;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-role-positions}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPatchAPIGuildRolePositionsJSONBodyItem {
    /**
     * Role id
     */
    pub id: String,
    /**
     * Sorting position of the role
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-role-positions}
 */
pub type RESTPatchAPIGuildRolePositionsJSONBody = Vec<RESTPatchAPIGuildRolePositionsJSONBodyItem>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-role-positions}
 */
pub type RESTPatchAPIGuildRolePositionsResult = Vec<APIRole>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-role}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIGuildRoleJSONBody {
    /**
     * Name of the role
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /**
     * Bitwise value of the enabled/disabled permissions
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Option<String>>,
    /**
     * RGB color value
     *
     * @remarks `color` will still be returned by the API, but using the `colors` field is recommended when doing requests.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<u32>>,
    /**
     * The role's colors
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<APIRoleColors>,
    /**
     * Whether the role should be displayed separately in the sidebar
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<Option<bool>>,
    /**
     * The role's icon image (if the guild has the `ROLE_ICONS` feature)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    /**
     * The role's unicode emoji as a standard emoji (if the guild has the `ROLE_ICONS` feature)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<Option<String>>,
    /**
     * Whether the role should be mentionable
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<Option<bool>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-role}
 */
pub type RESTGetAPIGuildRoleResult = APIRole;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-role}
 */
pub type RESTPatchAPIGuildRoleResult = APIRole;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#delete-guild-role}
 */
pub type RESTDeleteAPIGuildRoleResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-prune-count}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIGuildPruneCountQuery {
    /**
     * Number of days to count prune for (1 or more)
     *
     * @defaultValue `7`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u16>,
    /**
     * Role(s) to include
     *
     * While this is typed as a string, it represents an array of
     * role IDs delimited by commas
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-prune-count-query-string-params}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_roles: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-prune-count}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTGetAPIGuildPruneCountResult {
    pub pruned: i32,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#begin-guild-prune}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIGuildPruneJSONBody {
    /**
     * Number of days to count prune for (1 or more)
     *
     * @defaultValue `7`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u16>,
    /**
     * Whether `pruned` is returned, discouraged for large guilds
     *
     * @defaultValue `true`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_prune_count: Option<bool>,
    /**
     * Role(s) to include
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_roles: Option<Vec<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#begin-guild-prune}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIGuildPruneResult {
    pub pruned: Option<i32>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-voice-regions}
 */
pub type RESTGetAPIGuildVoiceRegionsResult = Vec<APIVoiceRegion>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-invites}
 */
pub type RESTGetAPIGuildInvitesResult = Vec<APIExtendedInvite>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-integrations}
 */
pub type RESTGetAPIGuildIntegrationsResult = Vec<APIGuildIntegration>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#delete-guild-integration}
 */
pub type RESTDeleteAPIGuildIntegrationResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-widget-settings}
 */
pub type RESTGetAPIGuildWidgetSettingsResult = APIGuildWidgetSettings;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-widget}
 */
pub type RESTPatchAPIGuildWidgetSettingsJSONBody = APIGuildWidgetSettings;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-widget}
 */
pub type RESTPatchAPIGuildWidgetSettingsResult = APIGuildWidgetSettings;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-widget}
 */
pub type RESTGetAPIGuildWidgetJSONResult = APIGuildWidget;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-vanity-url}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTGetAPIGuildVanityUrlResult {
    pub code: Option<String>,
    pub uses: i32,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-widget-image}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIGuildWidgetImageQuery {
    /**
     * Style of the widget image returned
     *
     * @defaultValue `"shield"`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<GuildWidgetStyle>,
}

/**
 * Note: while the return type is `ArrayBuffer`, the expected result is
 * a buffer of sorts (depends if in browser or on node.js/deno).
 */
pub type RESTGetAPIGuildWidgetImageResult = Vec<u8>;

pub type RESTGetAPIGuildMemberVerificationResult = APIGuildMembershipScreening;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIGuildMemberVerificationJSONBody {
    /**
     * Whether Membership Screening is enabled
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /**
     * Array of field objects serialized in a string
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_fields: Option<String>,
    /**
     * The server description to show in the screening form
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

pub type RESTPatchAPIGuildMemberVerificationResult = APIGuildMembershipScreening;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-welcome-screen}
 */
pub type RESTGetAPIGuildWelcomeScreenResult = APIGuildWelcomeScreen;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-welcome-screen}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIGuildWelcomeScreenJSONBody {
    /**
     * Whether the welcome screen is enabled
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Option<bool>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub other: Option<Value>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-welcome-screen}
 */
pub type RESTPatchAPIGuildWelcomeScreenResult = APIGuildWelcomeScreen;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#get-guild-onboarding}
 */
pub type RESTGetAPIGuildOnboardingResult = APIGuildOnboarding;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-onboarding}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPutAPIGuildOnboardingJSONBody {
    /**
     * Prompts shown during onboarding and in customize community
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Vec<RESTAPIGuildOnboardingPrompt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_channel_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIGuildOnboardingPrompt {
    /**
     * Options available within the prompt
     */
    pub options: Vec<RESTAPIGuildOnboardingPromptOption>,
    pub id: String,
    pub title: String,
}

/**
 * @deprecated Use {@link RESTAPIGuildOnboardingPrompt} instead.
 */
#[deprecated(note = "Use RESTAPIGuildOnboardingPrompt instead.")]
pub type RESTAPIModifyGuildOnboardingPromptData = RESTAPIGuildOnboardingPrompt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIGuildOnboardingPromptOption {
    /**
     * Emoji id
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<Option<String>>,
    /**
     * Emoji name
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<Option<String>>,
    /**
     * Whether this emoji is animated
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_animated: Option<Option<bool>>,
    pub title: String,
}

/**
 * @deprecated Use {@link RESTAPIGuildOnboardingPromptOption} instead.
 */
#[deprecated(note = "Use RESTAPIGuildOnboardingPromptOption instead.")]
pub type RESTAPIModifyGuildOnboardingPromptOptionData = RESTAPIGuildOnboardingPromptOption;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-guild-onboarding}
 */
pub type RESTPutAPIGuildOnboardingResult = APIGuildOnboarding;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#modify-incidents-actions}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPutAPIGuildIncidentActionsJSONBody {
    /**
     * When invites will be enabled again
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invites_disabled_until: Option<String>,
    /**
     * When direct messages will be enabled again
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms_disabled_until: Option<String>,
}
