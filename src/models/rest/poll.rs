use crate::models::payloads::{
    APIBasePoll, APIBasePollAnswer, APIMessage, APIPollDefaults, APIUser,
};
use serde::{Deserialize, Serialize};

/**
 * @see {@link https://discord.com/developers/docs/resources/poll#get-answer-voters}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIPollAnswerVotersQuery {
    /**
     * Get users after this user ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /**
     * Max number of users to return (1-100)
     *
     * @defaultValue `25`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/poll#poll-create-request-object-poll-create-request-object-structure}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIPoll {
    #[serde(flatten)]
    pub base: APIBasePoll,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<APIPollDefaults>,
    /**
     * Each of the answers available in the poll, up to 10
     */
    pub answers: Vec<APIBasePollAnswer>,
    /**
     * Number of hours the poll should be open for, up to 32 days
     *
     * @defaultValue `24`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
}

/**
 * @deprecated Use {@link RESTAPIPoll} instead
 */
pub type RESTAPIPollCreate = RESTAPIPoll;

/**
 * @see {@link https://discord.com/developers/docs/resources/poll#get-answer-voters}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTGetAPIPollAnswerVotersResult {
    /**
     * Users who voted for this answer
     */
    pub users: Vec<APIUser>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/poll#expire-poll}
 */
pub type RESTPostAPIPollExpireResult = APIMessage;
