use serde::{Deserialize, Serialize};

use super::guild::APIGuildMember;

/**
 * @see {@link https://discord.com/developers/docs/resources/stage-instance#stage-instance-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIStageInstance {
    /**
     * The id of the stage instance
     */
    pub id: String,
    /**
     * The guild id of the associated stage channel
     */
    pub guild_id: String,
    /**
     * The id of the associated stage channel
     */
    pub channel_id: String,
    /**
     * The topic of the stage instance (1-120 characters)
     */
    pub topic: String,
    /**
     * The privacy level of the stage instance
     *
     * @see {@link https://discord.com/developers/docs/resources/stage-instance#stage-instance-object-privacy-level}
     */
    pub privacy_level: StageInstancePrivacyLevel,
    /**
     * Whether or not stage discovery is disabled
     *
     * @deprecated
     * {@link https://github.com/discord/discord-api-docs/pull/4296 | discord-api-docs#4296}
     */
    pub discoverable_disabled: bool,
    /**
     * The id of the scheduled event for this stage instance
     */
    pub guild_scheduled_event_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/stage-instance#stage-instance-object-privacy-level}
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageInstancePrivacyLevel {
    /**
     * The stage instance is visible publicly, such as on stage discovery
     *
     * @deprecated
     * {@link https://github.com/discord/discord-api-docs/pull/4296 | discord-api-docs#4296}
     */
    Public = 1,
    /**
     * The stage instance is visible to only guild members
     */
    GuildOnly,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/invite#invite-stage-instance-object-invite-stage-instance-structure}
 * @deprecated
 * {@link https://github.com/discord/discord-api-docs/pull/4479 | discord-api-docs#4479}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInviteStageInstance {
    /**
     * The topic of the stage instance (1-120 characters)
     */
    pub topic: String,
    /**
     * The number of users in the stage
     */
    pub participant_count: i32,
    /**
     * The number of users speaking in the stage
     */
    pub speaker_count: i32,
    /**
     * The members speaking in the stage
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object-guild-member-structure}
     */
    pub members: Vec<APIGuildMember>,
}
