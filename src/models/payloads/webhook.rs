use serde::{Deserialize, Serialize};

use crate::models::payloads::APIEntitlement;

use super::{
    channel::APIWebhookSourceChannel,
    guild::{APIGuild, APIWebhookSourceGuild},
    interactions::ApplicationIntegrationType,
    oauth2::OAuth2Scopes,
    user::APIUser,
};

/**
 * Types extracted from https://discord.com/developers/docs/resources/webhook
 */

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#webhook-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIWebhook {
    /**
     * The id of the webhook
     */
    pub id: String,
    /**
     * The type of the webhook
     *
     * @see {@link https://discord.com/developers/docs/resources/webhook#webhook-object-webhook-types}
     */
    pub r#type: WebhookType,
    /**
     * The guild id this webhook is for
     */
    pub guild_id: Option<String>,
    /**
     * The channel id this webhook is for
     */
    pub channel_id: String,
    /**
     * The user this webhook was created by (not returned when getting a webhook with its token)
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub user: Option<APIUser>,
    /**
     * The default name of the webhook
     */
    pub name: Option<String>,
    /**
     * The default avatar of the webhook
     */
    pub avatar: Option<String>,
    /**
     * The secure token of the webhook (returned for Incoming Webhooks)
     */
    pub token: Option<String>,
    /**
     * The bot/OAuth2 application that created this webhook
     */
    pub application_id: Option<String>,
    /**
     * The guild of the channel that this webhook is following (returned for Channel Follower Webhooks)
     */
    pub source_guild: Option<APIWebhookSourceGuild>,
    /**
     * The channel that this webhook is following (returned for Channel Follower Webhooks)
     */
    pub source_channel: Option<APIWebhookSourceChannel>,
    /**
     * The url used for executing the webhook (returned by the webhooks OAuth2 flow)
     */
    pub url: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/events/webhook-events#webhook-event-payloads}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIWebhookEvent {
    Event(APIWebhookEventBase<ApplicationWebhookType, APIWebhookEventBody>),
    Ping(APIWebhookEventBase<ApplicationWebhookType, ()>),
}

/**
 * @see {@link https://discord.com/developers/docs/events/webhook-events#event-body-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum APIWebhookEventBody {
    #[serde(rename = "APPLICATION_AUTHORIZED")]
    ApplicationAuthorized {
        /**
         * Timestamp of when the event occurred in ISO8601 format
         */
        timestamp: String,
        /**
         * Data for the event. The shape depends on the event type
         */
        data: APIWebhookEventApplicationAuthorizedData,
    },
    #[serde(rename = "APPLICATION_DEAUTHORIZED")]
    ApplicationDeauthorized {
        /**
         * Timestamp of when the event occurred in ISO8601 format
         */
        timestamp: String,
        /**
         * Data for the event. The shape depends on the event type
         */
        data: APIWebhookEventApplicationDeauthorizedData,
    },
    #[serde(rename = "ENTITLEMENT_CREATE")]
    EntitlementCreate {
        /**
         * Timestamp of when the event occurred in ISO8601 format
         */
        timestamp: String,
        /**
         * Data for the event. The shape depends on the event type
         */
        data: APIWebhookEventEntitlementCreateData,
    },
    #[serde(rename = "QUEST_USER_ENROLLMENT")]
    QuestUserEnrollment {
        /**
         * Timestamp of when the event occurred in ISO8601 format
         */
        timestamp: String,
        /**
         * Data for the event. The shape depends on the event type
         */
        data: APIWebhookEventQuestUserEnrollmentData,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIWebhookEventApplicationAuthorizedData {
    /**
     * Installation context for the authorization. Either guild (`0`) if installed to a server or user (`1`) if installed to a user's account
     */
    pub integration_type: Option<ApplicationIntegrationType>,
    /**
     * User who authorized the app
     */
    pub user: APIUser,
    /**
     * List of scopes the user authorized
     */
    pub scopes: Vec<OAuth2Scopes>,
    /**
     * Server which app was authorized for (when integration type is `0`)
     */
    pub guild: Option<APIGuild>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIWebhookEventApplicationDeauthorizedData {
    /**
     * User who deauthorized the app
     */
    pub user: APIUser,
}

pub type APIWebhookEventEntitlementCreateData = APIEntitlement;

pub type APIWebhookEventQuestUserEnrollmentData = ();

/**
 * @see {@link https://discord.com/developers/docs/events/webhook-events#webhook-types}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApplicationWebhookType {
    /**
     * PING event sent to verify your Webhook Event URL is active
     */
    Ping,
    /**
     * Webhook event (details for event in event body object)
     */
    Event,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIWebhookEventBase<Type, Event> {
    /**
     * Version scheme for the webhook event. Currently always `1`
     */
    pub version: u8,
    /**
     * ID of your app
     */
    pub application_id: String,
    /**
     * Type of webhook
     */
    pub r#type: Type,
    /**
     * Event data payload
     */
    pub event: Event,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIWebhookEventEventBase<Type, Data> {
    /**
     * Event type
     */
    pub r#type: Type,
    /**
     * Timestamp of when the event occurred in ISO8601 format
     */
    pub timestamp: String,
    /**
     * Data for the event. The shape depends on the event type
     */
    pub data: Data,
}

/**
 * @see {@link https://discord.com/developers/docs/events/webhook-events#event-types}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApplicationWebhookEventType {
    /**
     * Sent when an app was authorized by a user to a server or their account
     */
    #[serde(rename = "APPLICATION_AUTHORIZED")]
    ApplicationAuthorized,
    /**
     * Sent when an app was deauthorized by a user
     */
    #[serde(rename = "APPLICATION_DEAUTHORIZED")]
    ApplicationDeauthorized,
    /**
     * Entitlement was created
     */
    #[serde(rename = "ENTITLEMENT_CREATE")]
    EntitlementCreate,
    /**
     * User was added to a Quest (currently unavailable)
     */
    #[serde(rename = "QUEST_USER_ENROLLMENT")]
    QuestUserEnrollment,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#webhook-object-webhook-types}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WebhookType {
    /**
     * Incoming Webhooks can post messages to channels with a generated token
     */
    Incoming = 1,
    /**
     * Channel Follower Webhooks are internal webhooks used with Channel Following to post new messages into channels
     */
    ChannelFollower = 2,
    /**
     * Application webhooks are webhooks used with Interactions
     */
    Application = 3,
}
