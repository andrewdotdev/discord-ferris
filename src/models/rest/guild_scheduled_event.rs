// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use crate::models::payloads::{
    APIGuildScheduledEvent, APIGuildScheduledEventEntityMetadata,
    APIGuildScheduledEventRecurrenceRule, APIGuildScheduledEventUser,
    GuildScheduledEventEntityType, GuildScheduledEventPrivacyLevel, GuildScheduledEventStatus,
};
use serde::{Deserialize, Serialize};

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#list-scheduled-events-for-guild}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIGuildScheduledEventsQuery {
    /**
     * Whether to include number of users subscribed to each event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_user_count: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#list-scheduled-events-for-guild}
 */
pub type RESTGetAPIGuildScheduledEventsResult = Vec<APIGuildScheduledEvent>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#create-guild-scheduled-event}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIGuildScheduledEventJSONBody {
    /**
     * The stage channel id of the guild event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /**
     * The name of the guild event
     */
    pub name: String,
    /**
     * The privacy level of the guild event
     */
    pub privacy_level: GuildScheduledEventPrivacyLevel,
    /**
     * The time to schedule the guild event at
     */
    pub scheduled_start_time: String,
    /**
     * The time when the scheduled event is scheduled to end
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_time: Option<String>,
    /**
     * The description of the guild event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /**
     * The scheduled entity type of the guild event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<GuildScheduledEventEntityType>,
    /**
     * The entity metadata of the scheduled event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_metadata: Option<APIGuildScheduledEventEntityMetadata>,
    /**
     * The cover image of the scheduled event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<String>>,
    /**
     * The definition for how often this event should recur
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_rule: Option<APIGuildScheduledEventRecurrenceRule>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#create-guild-scheduled-event}
 */
pub type RESTPostAPIGuildScheduledEventResult = APIGuildScheduledEvent;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#get-guild-scheduled-event}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIGuildScheduledEventQuery {
    /**
     * Whether to include number of users subscribed to this event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_user_count: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#get-guild-scheduled-event}
 */
pub type RESTGetAPIGuildScheduledEventResult = APIGuildScheduledEvent;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#modify-guild-scheduled-event}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIGuildScheduledEventJSONBody {
    /**
     * The description of the guild event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /**
     * The entity metadata of the scheduled event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_metadata: Option<Option<APIGuildScheduledEventEntityMetadata>>,
    /**
     * The definition for how often this event should recur
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_rule: Option<Option<APIGuildScheduledEventRecurrenceRule>>,

    /**
     * The stage channel id of the guild event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /**
     * The name of the guild event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**
     * The privacy level of the guild event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<GuildScheduledEventPrivacyLevel>,
    /**
     * The time to schedule the guild event at
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start_time: Option<String>,
    /**
     * The time when the scheduled event is scheduled to end
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_time: Option<String>,
    /**
     * The scheduled entity type of the guild event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<GuildScheduledEventEntityType>,
    /**
     * The cover image of the scheduled event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<String>>,

    /**
     * The status of the scheduled event
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GuildScheduledEventStatus>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#modify-guild-scheduled-event}
 */
pub type RESTPatchAPIGuildScheduledEventResult = APIGuildScheduledEvent;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#delete-guild-scheduled-event}
 */
pub type RESTDeleteAPIGuildScheduledEventResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#get-guild-scheduled-event-users}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIGuildScheduledEventUsersQuery {
    /**
     * Number of users to receive from the event
     *
     * @defaultValue `100`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /**
     * Whether to include guild member data if it exists
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_member: Option<bool>,
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
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#get-guild-scheduled-event-users}
 */
pub type RESTGetAPIGuildScheduledEventUsersResult = Vec<APIGuildScheduledEventUser>;
