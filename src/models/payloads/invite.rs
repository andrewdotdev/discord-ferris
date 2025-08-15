use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::payloads::application::APIApplication;
use crate::models::payloads::channel::APIInviteChannel;
use crate::models::payloads::guild::{
    GuildFeature, GuildNSFWLevel, GuildVerificationLevel,
};
use crate::models::payloads::guild_scheduled_event::APIGuildScheduledEvent;
use crate::models::payloads::stage_instance::APIInviteStageInstance;
use crate::models::payloads::user::APIUser;

/**
 * Types extracted from https://discord.com/developers/docs/resources/invite
 */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInviteGuild {
    /**
     * Banner
     */
    pub banner: Option<String>,
    /**
     * Description
     */
    pub description: Option<String>,
    /**
     * Enabled guild features
     */
    pub features: Vec<GuildFeature>,
    /**
     * Icon hash
     */
    pub icon: Option<String>,
    /**
     * Guild id
     */
    pub id: String,
    /**
     * Guild name (2-100 characters, excluding trailing and leading whitespace)
     */
    pub name: String,
    /**
     * The nsfw level of the guild
     */
    pub nsfw_level: GuildNSFWLevel,
    /**
     * The number of boosts this guild currently has
     */
    pub premium_subscription_count: Option<i64>,
    /**
     * Splash hash
     */
    pub splash: Option<String>,
    /**
     * The vanity url code for the guild
     */
    pub vanity_url_code: Option<String>,
    /**
     * Verification level required for the guild
     */
    pub verification_level: GuildVerificationLevel,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/invite#invite-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInvite {
    /**
     * The invite code (unique ID)
     */
    pub code: String,
    /**
     * The guild this invite is for
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object}
     */
    pub guild: Option<APIInviteGuild>,
    /**
     * The channel this invite is for
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object}
     */
    pub channel: Option<APIInviteChannel>,
    /**
     * The user who created the invite
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub inviter: Option<APIUser>,
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
     *
     * @see {@link https://discord.com/developers/docs/resources/application#application-object}
     */
    pub target_application: Option<PartialAPIApplication>,
    /**
     * Approximate count of online members, returned from the `GET /invites/<code>` endpoint when `with_counts` is `true`
     */
    pub approximate_presence_count: Option<i64>,
    /**
     * Approximate count of total members, returned from the `GET /invites/<code>` endpoint when `with_counts` is `true`
     */
    pub approximate_member_count: Option<i64>,
    /**
     * The expiration date of this invite
     */
    pub expires_at: Option<String>,
    /**
     * The stage instance data if there is a public stage instance in the stage channel this invite is for
     *
     * @deprecated
     * {@link https://github.com/discord/discord-api-docs/pull/4479 | discord-api-docs#4479}
     */
    pub stage_instance: Option<APIInviteStageInstance>,
    /**
     * The guild scheduled event data, returned from the `GET /invites/<code>` endpoint when `guild_scheduled_event_id` is a valid guild scheduled event id
     */
    pub guild_scheduled_event: Option<APIGuildScheduledEvent>,
    /**
     * The invite type
     */
    pub r#type: InviteType,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/invite#invite-object-invite-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum InviteType {
    Guild = 0,
    GroupDM = 1,
    Friend = 2,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/invite#invite-object-invite-target-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum InviteTargetType {
    Stream = 1,
    EmbeddedApplication = 2,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/invite#invite-metadata-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIExtendedInvite {
    /**
     * The invite code (unique ID)
     */
    pub code: String,
    /**
     * The guild this invite is for
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-object}
     */
    pub guild: Option<APIInviteGuild>,
    /**
     * The channel this invite is for
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object}
     */
    pub channel: Option<APIInviteChannel>,
    /**
     * The user who created the invite
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub inviter: Option<APIUser>,
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
     *
     * @see {@link https://discord.com/developers/docs/resources/application#application-object}
     */
    pub target_application: Option<PartialAPIApplication>,
    /**
     * Approximate count of online members, returned from the `GET /invites/<code>` endpoint when `with_counts` is `true`
     */
    pub approximate_presence_count: Option<i64>,
    /**
     * Approximate count of total members, returned from the `GET /invites/<code>` endpoint when `with_counts` is `true`
     */
    pub approximate_member_count: Option<i64>,
    /**
     * The expiration date of this invite
     */
    pub expires_at: Option<String>,
    /**
     * The stage instance data if there is a public stage instance in the stage channel this invite is for
     *
     * @deprecated
     * {@link https://github.com/discord/discord-api-docs/pull/4479 | discord-api-docs#4479}
     */
    pub stage_instance: Option<APIInviteStageInstance>,
    /**
     * The guild scheduled event data, returned from the `GET /invites/<code>` endpoint when `guild_scheduled_event_id` is a valid guild scheduled event id
     */
    pub guild_scheduled_event: Option<APIGuildScheduledEvent>,
    /**
     * The invite type
     */
    pub r#type: InviteType,
    /**
     * Number of times this invite has been used
     */
    pub uses: i64,
    /**
     * Max number of times this invite can be used
     */
    pub max_uses: i64,
    /**
     * Duration (in seconds) after which the invite expires
     */
    pub max_age: i64,
    /**
     * Whether this invite only grants temporary membership
     */
    pub temporary: bool,
    /**
     * When this invite was created
     */
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartialAPIApplication {
    Full(APIApplication),
    Other(serde_json::Value),
}
