use serde::{Deserialize, Serialize};

use super::guild::APIGuildMember;

/**
 * Types extracted from https://discord.com/developers/docs/resources/voice
 */

/**
 * @see {@link https://discord.com/developers/docs/resources/voice#voice-state-object}
 * @deprecated This is deprecated, use {@link APIVoiceState}
 */
pub type GatewayVoiceState = APIVoiceState;

/**
 * @see {@link https://discord.com/developers/docs/resources/voice#voice-state-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBaseVoiceState {
    /**
     * The channel id this user is connected to
     */
    pub channel_id: Option<String>,
    /**
     * The user id this voice state is for
     */
    pub user_id: String,
    /**
     * The guild member this voice state is for
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
     */
    pub member: Option<APIGuildMember>,
    /**
     * The session id for this voice state
     */
    pub session_id: String,
    /**
     * Whether this user is deafened by the server
     */
    pub deaf: bool,
    /**
     * Whether this user is muted by the server
     */
    pub mute: bool,
    /**
     * Whether this user is locally deafened
     */
    pub self_deaf: bool,
    /**
     * Whether this user is locally muted
     */
    pub self_mute: bool,
    /**
     * Whether this user is streaming using "Go Live"
     */
    pub self_stream: Option<bool>,
    /**
     * Whether this user's camera is enabled
     */
    pub self_video: bool,
    /**
     * Whether this user is muted by the current user
     */
    pub suppress: bool,
    /**
     * The time at which the user requested to speak
     */
    pub request_to_speak_timestamp: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/voice#voice-state-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIVoiceState {
    #[serde(flatten)]
    pub base: APIBaseVoiceState,
    /**
     * The guild id this voice state is for
     */
    pub guild_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/voice#voice-region-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIVoiceRegion {
    /**
     * Unique ID for the region
     */
    pub id: String,
    /**
     * Name of the region
     */
    pub name: String,
    /**
     * `true` for a single server that is closest to the current user's client
     */
    pub optimal: bool,
    /**
     * Whether this is a deprecated voice region (avoid switching to these)
     */
    pub deprecated: bool,
    /**
     * Whether this is a custom voice region (used for events/etc)
     */
    pub custom: bool,
}
