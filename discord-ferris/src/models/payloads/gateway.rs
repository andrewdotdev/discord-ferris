// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::payloads::{APIThreadChannel, APIThreadMember};

/**
 * Types extracted from
 *  - https://discord.com/developers/docs/topics/gateway
 *  - https://discord.com/developers/docs/topics/gateway-events
 */

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway#get-gateway}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGatewayInfo {
    /**
     * The WSS URL that can be used for connecting to the gateway
     */
    pub url: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway#get-gateway-bot}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGatewayBotInfo {
    #[serde(flatten)]
    pub base: APIGatewayInfo,
    /**
     * The recommended number of shards to use when connecting
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway#sharding}
     */
    pub shards: i64,
    /**
     * Information on the current session start limit
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway#session-start-limit-object}
     */
    pub session_start_limit: APIGatewaySessionStartLimit,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway#session-start-limit-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGatewaySessionStartLimit {
    /**
     * The total number of session starts the current user is allowed
     */
    pub total: i64,
    /**
     * The remaining number of session starts the current user is allowed
     */
    pub remaining: i64,
    /**
     * The number of milliseconds after which the limit resets
     */
    pub reset_after: i64,
    /**
     * The number of identify requests allowed per 5 seconds
     */
    pub max_concurrency: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#update-presence}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GatewayGuildMembersChunkPresence {
    /**
     * The user presence is being updated for
     *
     * **The user object within this event can be partial, the only field which must be sent is the `id` field,
     * everything else is optional.**
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub user: GatewayPresenceUser,
    /**
     * Either "idle", "dnd", "online", or "offline"
     */
    pub status: Option<PresenceUpdateReceiveStatus>,
    /**
     * User's current activities
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object}
     */
    pub activities: Option<Vec<GatewayActivity>>,
    /**
     * User's platform-dependent status
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#client-status-object}
     */
    pub client_status: Option<GatewayPresenceClientStatus>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#presence-update-presence-update-event-fields}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GatewayPresenceUpdate {
    #[serde(flatten)]
    pub base: GatewayGuildMembersChunkPresence,
    /**
     * ID of the guild
     */
    pub guild_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#update-presence-status-types}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PresenceUpdateStatus {
    Online,
    DoNotDisturb,
    Idle,
    /**
     * Invisible and shown as offline
     */
    Invisible,
    Offline,
}

pub type PresenceUpdateReceiveStatus = PresenceUpdateReceiveStatusEnum;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PresenceUpdateReceiveStatusEnum {
    Online,
    DoNotDisturb,
    Idle,
    Offline,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#client-status-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayPresenceClientStatus {
    /**
     * The user's status set for an active desktop (Windows, Linux, Mac) application session
     */
    pub desktop: Option<PresenceUpdateReceiveStatus>,
    /**
     * The user's status set for an active mobile (iOS, Android) application session
     */
    pub mobile: Option<PresenceUpdateReceiveStatus>,
    /**
     * The user's status set for an active web (browser, bot account) application session
     */
    pub web: Option<PresenceUpdateReceiveStatus>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GatewayActivity {
    /**
     * The activity's id
     *
     * @unstable
     */
    pub id: String,
    /**
     * The activity's name
     */
    pub name: String,
    /**
     * Activity type
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-types}
     */
    #[serde(rename = "type")]
    pub r#type: ActivityType,
    /**
     * Stream url, is validated when type is `1`
     */
    pub url: Option<String>,
    /**
     * Unix timestamp of when the activity was added to the user's session
     */
    pub created_at: i64,
    /**
     * Unix timestamps for start and/or end of the game
     */
    pub timestamps: Option<GatewayActivityTimestamps>,
    /**
     * The Spotify song id
     *
     * @unstable
     */
    pub sync_id: Option<String>,
    /**
     * The platform this activity is being done on
     *
     * @unstable You can use {@link ActivityPlatform} as a stepping stone, but this might be inaccurate
     */
    pub platform: Option<String>,
    /**
     * Application id for the game
     */
    pub application_id: Option<String>,
    /**
     * Controls which field is displayed in the user's status text in the member list
     *
     * @see {@link https://discord.com/developers/docs/events/gateway-events#activity-object-status-display-types}
     */
    pub status_display_type: Option<StatusDisplayType>,
    /**
     * What the player is currently doing
     */
    pub details: Option<String>,
    /**
     * URL that is linked when clicking on the details text
     */
    pub details_url: Option<String>,
    /**
     * The user's current party status, or the text used for a custom status
     */
    pub state: Option<String>,
    /**
     * URL that is linked when clicking on the state text
     */
    pub state_url: Option<String>,
    /**
     * The emoji used for a custom status
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-emoji}
     */
    pub emoji: Option<GatewayActivityEmoji>,
    /**
     * @unstable
     */
    pub session_id: Option<String>,
    /**
     * Information for the current party of the player
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-party}
     */
    pub party: Option<GatewayActivityParty>,
    /**
     * Images for the presence and their hover texts
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-assets}
     */
    pub assets: Option<GatewayActivityAssets>,
    /**
     * Secrets for Rich Presence joining and spectating
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-secrets}
     */
    pub secrets: Option<GatewayActivitySecrets>,
    /**
     * Whether or not the activity is an instanced game session
     */
    pub instance: Option<bool>,
    /**
     * Activity flags `OR`d together, describes what the payload includes
     *
     * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-flags}
     * @see {@link https://en.wikipedia.org/wiki/Bit_field}
     */
    pub flags: Option<ActivityFlags>,
    /**
     * The custom buttons shown in the Rich Presence (max 2)
     */
    pub buttons: Option<GatewayActivityButtons>,
}

/**
 * @unstable This enum is currently not documented by Discord but has known values which we will try to keep up to date.
 * Values might be added or removed without a major version bump.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActivityPlatform {
    Desktop,
    Xbox,
    Samsung,
    IOS,
    Android,
    Embedded,
    PS4,
    PS5,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActivityType {
    /**
     * Playing \{game\}
     */
    Playing = 0,
    /**
     * Streaming \{details\}
     */
    Streaming = 1,
    /**
     * Listening to \{name\}
     */
    Listening = 2,
    /**
     * Watching \{details\}
     */
    Watching = 3,
    /**
     * \{emoji\} \{state\}
     */
    Custom = 4,
    /**
     * Competing in \{name\}
     */
    Competing = 5,
}

/**
 * Controls which field is used in the user's status message
 *
 * @see {@link https://discord.com/developers/docs/events/gateway-events#activity-object-status-display-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StatusDisplayType {
    /**
     * Playing \{name\}
     */
    Name = 0,
    /**
     * Playing \{state\}
     */
    State = 1,
    /**
     * Playing \{details\}
     */
    Details = 2,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-timestamps}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayActivityTimestamps {
    /**
     * Unix time (in milliseconds) of when the activity started
     */
    pub start: Option<i64>,
    /**
     * Unix time (in milliseconds) of when the activity ends
     */
    pub end: Option<i64>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-emoji}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayActivityEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-party}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayActivityParty {
    /**
     * The id of the party
     */
    pub id: Option<String>,
    /**
     * Used to show the party's current and maximum size
     */
    pub size: Option<[i64; 2]>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-assets}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub large_url: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub small_url: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-secrets}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayActivitySecrets {
    pub join: Option<String>,
    #[serde(rename = "match")]
    pub r#match: Option<String>,
    pub spectate: Option<String>,
}

bitflags! {
    /**
    * @see {@link https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-flags}
    */
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ActivityFlags: u32 {
        const Instance = 1 << 0;
        const Join = 1 << 1;
        const Spectate = 1 << 2;
        const JoinRequest = 1 << 3;
        const Sync = 1 << 4;
        const Play = 1 << 5;
        const PartyPrivacyFriends = 1 << 6;
        const PartyPrivacyVoiceChannel = 1 << 7;
        const Embedded = 1 << 8;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GatewayActivityButtons {
    Buttons(Vec<GatewayActivityButton>),
    Strings(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayActivityButton {
    /**
     * The text shown on the button (1-32 characters)
     */
    pub label: String,
    /**
     * The url opened when clicking the button (1-512 characters)
     */
    pub url: String,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-list-sync-thread-list-sync-event-fields}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GatewayThreadListSync {
    /**
     * ID of the guild
     */
    pub guild_id: String,
    /**
     * The ids of all the parent channels whose threads are being synced, otherwise the entire guild
     */
    pub channel_ids: Option<Vec<String>>,
    /**
     * Array of the synced threads
     */
    pub threads: Vec<APIThreadChannel>,
    /**
     * The member objects for the client user in each joined thread that was synced
     */
    pub members: Vec<APIThreadMember>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/gateway-events#thread-members-update-thread-members-update-event-fields}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GatewayThreadMembersUpdate {
    /**
     * The id of the thread for which members are being synced
     */
    pub id: String,
    /**
     * The id of the guild that the thread is in
     */
    pub guild_id: String,
    /**
     * The approximate member count of the thread, does not count above 50 even if there are more members
     */
    pub member_count: i64,
    /**
     * The members that were added to the thread
     */
    pub added_members: Option<Vec<APIThreadMember>>,
    /**
     * The ids of the members that were removed from the thread
     */
    pub removed_member_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GatewayPresenceUser {
    pub id: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
