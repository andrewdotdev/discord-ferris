use crate::models::payloads::{APIStageInstance, StageInstancePrivacyLevel};
use serde::{Deserialize, Serialize};

/**
 * @see {@link https://discord.com/developers/docs/resources/stage-instance#create-stage-instance}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RESTPostAPIStageInstanceJSONBody {
    /**
     * The id of the stage channel
     */
    pub channel_id: String,
    /**
     * The topic of the stage instance (1-120 characters)
     */
    pub topic: String,
    /**
     * The privacy level of the stage instance
     *
     * @defaultValue `StageInstancePrivacyLevel.GuildOnly`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<StageInstancePrivacyLevel>,
    /**
     * Notify `@everyone` that a stage instance has started
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_start_notification: Option<bool>,
    /**
     * The guild scheduled event associated with this stage instance
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_event_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/stage-instance#create-stage-instance}
 */
pub type RESTPostAPIStageInstanceResult = APIStageInstance;

/**
 * @see {@link https://discord.com/developers/docs/resources/stage-instance#get-stage-instance}
 */
pub type RESTGetAPIStageInstanceResult = APIStageInstance;

/**
 * @see {@link https://discord.com/developers/docs/resources/stage-instance#modify-stage-instance}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RESTPatchAPIStageInstanceJSONBody {
    /**
     * The topic of the stage instance (1-120 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /**
     * The privacy level of the stage instance
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<StageInstancePrivacyLevel>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/stage-instance#modify-stage-instance}
 */
pub type RESTPatchAPIStageInstanceResult = APIStageInstance;

/**
 * @see {@link https://discord.com/developers/docs/resources/stage-instance#delete-stage-instance}
 */
pub type RESTDeleteAPIStageInstanceResult = ();
