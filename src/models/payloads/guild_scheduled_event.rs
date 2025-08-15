use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::payloads::guild::APIGuildMember;
use crate::models::payloads::user::APIUser;

pub use GuildScheduledEventRecurrenceRuleNWeekday as _Deprecated_GuildScheduledEventRecurrenceRuleNWeekday;

/**
 * Types extracted from https://discord.com/developers/docs/resources/guild-scheduled-event
 */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildScheduledEventBase {
    /**
     * The id of the guild event
     */
    pub id: String,
    /**
     * The guild id which the scheduled event belongs to
     */
    pub guild_id: String,
    /**
     * The id of the user that created the scheduled event
     */
    pub creator_id: Option<Option<String>>,
    /**
     * The name of the scheduled event
     */
    pub name: String,
    /**
     * The description of the scheduled event
     */
    pub description: Option<Option<String>>,
    /**
     * The time the scheduled event will start
     */
    pub scheduled_start_time: String,
    /**
     * The time at which the guild event will end, or `null` if the event does not have a scheduled time to end
     */
    pub scheduled_end_time: Option<String>,
    /**
     * The privacy level of the scheduled event
     */
    pub privacy_level: GuildScheduledEventPrivacyLevel,
    /**
     * The status of the scheduled event
     */
    pub status: GuildScheduledEventStatus,
    /**
     * The type of hosting entity associated with the scheduled event
     */
    pub entity_type: GuildScheduledEventEntityType,
    /**
     * The id of the hosting entity associated with the scheduled event
     */
    pub entity_id: Option<String>,
    /**
     * The user that created the scheduled event
     */
    pub creator: Option<APIUser>,
    /**
     * The number of users subscribed to the scheduled event
     */
    pub user_count: Option<i64>,
    /**
     * The cover image of the scheduled event
     */
    pub image: Option<Option<String>>,
    /**
     * The definition for how often this event should recur
     */
    pub recurrence_rule: Option<Option<APIGuildScheduledEventRecurrenceRule>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildScheduledEventRecurrenceRule {
    /**
     * Starting time of the recurrence interval
     */
    pub start: String,
    /**
     * Ending time of the recurrence interval
     */
    pub end: Option<String>,
    /**
     * How often the event occurs
     */
    pub frequency: GuildScheduledEventRecurrenceRuleFrequency,
    /**
     * The spacing between the events, defined by `frequency`.
     * For example, `frequency` of {@link GuildScheduledEventRecurrenceRuleFrequency.Weekly} and an `interval` of `2`
     * would be "every-other week"
     */
    pub interval: i64,
    /**
     * Set of specific days within a week for the event to recur on
     */
    pub by_weekday: Option<Vec<GuildScheduledEventRecurrenceRuleWeekday>>,
    /**
     * List of specific days within a specific week (1-5) to recur on
     */
    pub by_n_weekday: Option<Vec<APIGuildScheduledEventRecurrenceRuleNWeekday>>,
    /**
     * Set of specific months to recur on
     */
    pub by_month: Option<Vec<GuildScheduledEventRecurrenceRuleMonth>>,
    /**
     * Set of specific dates within a month to recur on
     */
    pub by_month_day: Option<Vec<i64>>,
    /**
     * Set of days within a year to recur on (1-364)
     */
    pub by_year_day: Option<Vec<i64>>,
    /**
     * The total amount of times that the event is allowed to recur before stopping
     */
    pub count: Option<i64>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-frequency}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleFrequency {
    Yearly = 0,
    Monthly = 1,
    Weekly = 2,
    Daily = 3,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-weekday}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleWeekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-month}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleMonth {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-recurrence-rule-object-guild-scheduled-event-recurrence-rule-nweekday-structure}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildScheduledEventRecurrenceRuleNWeekday {
    /**
     * The week to reoccur on.
     */
    pub n: u8,
    /**
     * The day within the week to reoccur on
     */
    pub day: GuildScheduledEventRecurrenceRuleWeekday,
}

/**
 * @deprecated Use {@link APIGuildScheduledEventRecurrenceRuleNWeekday} instead
 */
pub type GuildScheduledEventRecurrenceRuleNWeekday = APIGuildScheduledEventRecurrenceRuleNWeekday;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIStageInstanceGuildScheduledEvent {
    /**
     * The id of the guild event
     */
    pub id: String,
    /**
     * The guild id which the scheduled event belongs to
     */
    pub guild_id: String,
    /**
     * The channel id in which the scheduled event will be hosted, or `null` if entity type is `EXTERNAL`
     */
    pub channel_id: String,
    /**
     * The id of the user that created the scheduled event
     */
    pub creator_id: Option<Option<String>>,
    /**
     * The name of the scheduled event
     */
    pub name: String,
    /**
     * The description of the scheduled event
     */
    pub description: Option<Option<String>>,
    /**
     * The time the scheduled event will start
     */
    pub scheduled_start_time: String,
    /**
     * The time at which the guild event will end, or `null` if the event does not have a scheduled time to end
     */
    pub scheduled_end_time: Option<String>,
    /**
     * The privacy level of the scheduled event
     */
    pub privacy_level: GuildScheduledEventPrivacyLevel,
    /**
     * The status of the scheduled event
     */
    pub status: GuildScheduledEventStatus,
    /**
     * The type of hosting entity associated with the scheduled event
     */
    pub entity_type: GuildScheduledEventEntityType,
    /**
     * The id of the hosting entity associated with the scheduled event
     */
    pub entity_id: Option<String>,
    /**
     * The entity metadata for the scheduled event
     */
    pub entity_metadata: Option<()>,
    /**
     * The user that created the scheduled event
     */
    pub creator: Option<APIUser>,
    /**
     * The number of users subscribed to the scheduled event
     */
    pub user_count: Option<i64>,
    /**
     * The cover image of the scheduled event
     */
    pub image: Option<Option<String>>,
    /**
     * The definition for how often this event should recur
     */
    pub recurrence_rule: Option<Option<APIGuildScheduledEventRecurrenceRule>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIVoiceGuildScheduledEvent {
    /**
     * The id of the guild event
     */
    pub id: String,
    /**
     * The guild id which the scheduled event belongs to
     */
    pub guild_id: String,
    /**
     * The channel id in which the scheduled event will be hosted, or `null` if entity type is `EXTERNAL`
     */
    pub channel_id: String,
    /**
     * The id of the user that created the scheduled event
     */
    pub creator_id: Option<Option<String>>,
    /**
     * The name of the scheduled event
     */
    pub name: String,
    /**
     * The description of the scheduled event
     */
    pub description: Option<Option<String>>,
    /**
     * The time the scheduled event will start
     */
    pub scheduled_start_time: String,
    /**
     * The time at which the guild event will end, or `null` if the event does not have a scheduled time to end
     */
    pub scheduled_end_time: Option<String>,
    /**
     * The privacy level of the scheduled event
     */
    pub privacy_level: GuildScheduledEventPrivacyLevel,
    /**
     * The status of the scheduled event
     */
    pub status: GuildScheduledEventStatus,
    /**
     * The type of hosting entity associated with the scheduled event
     */
    pub entity_type: GuildScheduledEventEntityType,
    /**
     * The id of the hosting entity associated with the scheduled event
     */
    pub entity_id: Option<String>,
    /**
     * The entity metadata for the scheduled event
     */
    pub entity_metadata: Option<()>,
    /**
     * The user that created the scheduled event
     */
    pub creator: Option<APIUser>,
    /**
     * The number of users subscribed to the scheduled event
     */
    pub user_count: Option<i64>,
    /**
     * The cover image of the scheduled event
     */
    pub image: Option<Option<String>>,
    /**
     * The definition for how often this event should recur
     */
    pub recurrence_rule: Option<Option<APIGuildScheduledEventRecurrenceRule>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIExternalGuildScheduledEvent {
    /**
     * The id of the guild event
     */
    pub id: String,
    /**
     * The guild id which the scheduled event belongs to
     */
    pub guild_id: String,
    /**
     * The channel id in which the scheduled event will be hosted, or `null` if entity type is `EXTERNAL`
     */
    pub channel_id: Option<()>,
    /**
     * The id of the user that created the scheduled event
     */
    pub creator_id: Option<Option<String>>,
    /**
     * The name of the scheduled event
     */
    pub name: String,
    /**
     * The description of the scheduled event
     */
    pub description: Option<Option<String>>,
    /**
     * The time the scheduled event will start
     */
    pub scheduled_start_time: String,
    /**
     * The time at which the guild event will end, or `null` if the event does not have a scheduled time to end
     */
    pub scheduled_end_time: Option<String>,
    /**
     * The privacy level of the scheduled event
     */
    pub privacy_level: GuildScheduledEventPrivacyLevel,
    /**
     * The status of the scheduled event
     */
    pub status: GuildScheduledEventStatus,
    /**
     * The type of hosting entity associated with the scheduled event
     */
    pub entity_type: GuildScheduledEventEntityType,
    /**
     * The id of the hosting entity associated with the scheduled event
     */
    pub entity_id: Option<String>,
    /**
     * The entity metadata for the scheduled event
     */
    pub entity_metadata: APIGuildScheduledEventEntityMetadataRequired,
    /**
     * The user that created the scheduled event
     */
    pub creator: Option<APIUser>,
    /**
     * The number of users subscribed to the scheduled event
     */
    pub user_count: Option<i64>,
    /**
     * The cover image of the scheduled event
     */
    pub image: Option<Option<String>>,
    /**
     * The definition for how often this event should recur
     */
    pub recurrence_rule: Option<Option<APIGuildScheduledEventRecurrenceRule>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIGuildScheduledEvent {
    APIExternalGuildScheduledEvent(APIExternalGuildScheduledEvent),
    APIStageInstanceGuildScheduledEvent(APIStageInstanceGuildScheduledEvent),
    APIVoiceGuildScheduledEvent(APIVoiceGuildScheduledEvent),
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-metadata}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildScheduledEventEntityMetadata {
    /**
     * The location of the scheduled event
     */
    pub location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIGuildScheduledEventEntityMetadataRequired {
    /**
     * The location of the scheduled event
     */
    pub location: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventEntityType {
    StageInstance = 1,
    Voice = 2,
    External = 3,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-status}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventStatus {
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Canceled = 4,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-privacy-level}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventPrivacyLevel {
    /**
     * The scheduled event is only accessible to guild members
     */
    GuildOnly = 2,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-user-object-guild-scheduled-event-user-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildScheduledEventUser {
    /**
     * The scheduled event id which the user subscribed to
     */
    pub guild_scheduled_event_id: String,
    /**
     * The user which subscribed to the event
     */
    pub user: APIUser,
    /**
     * The guild member data for this user for the guild which this event belongs to, if any
     */
    pub member: Option<APIGuildMember>,
}
