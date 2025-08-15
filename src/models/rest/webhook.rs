use super::channel::RESTAPIAttachment;
use super::poll::RESTAPIPoll;
use crate::models::payloads::{
    APIAllowedMentions, APIEmbed, APIMessage, APIMessageTopLevelComponent, APIWebhook, MessageFlags,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#create-webhook}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIChannelWebhookJSONBody {
    /**
     * Name of the webhook (1-80 characters)
     */
    pub name: String,
    /**
     * Image for the default webhook avatar
     *
     * @see {@link https://discord.com/developers/docs/reference#image-data}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#create-webhook}
 */
pub type RESTPostAPIChannelWebhookResult = APIWebhook;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#get-channel-webhooks}
 */
pub type RESTGetAPIChannelWebhooksResult = Vec<APIWebhook>;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#get-guild-webhooks}
 */
pub type RESTGetAPIGuildWebhooksResult = Vec<APIWebhook>;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#get-webhook}
 */
pub type RESTGetAPIWebhookResult = APIWebhook;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#get-webhook-with-token}
 */
pub type RESTGetAPIWebhookWithTokenResult = APIWebhook;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#modify-webhook}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPatchAPIWebhookJSONBody {
    /**
     * The default name of the webhook
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**
     * Image for the default webhook avatar
     *
     * @see {@link https://discord.com/developers/docs/reference#image-data}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
    /**
     * The new channel id this webhook should be moved to
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#modify-webhook}
 */
pub type RESTPatchAPIWebhookResult = APIWebhook;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#modify-webhook-with-token}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPatchAPIWebhookWithTokenJSONBody {
    /**
     * The default name of the webhook
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**
     * Image for the default webhook avatar
     *
     * @see {@link https://discord.com/developers/docs/reference#image-data}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#modify-webhook-with-token}
 */
pub type RESTPatchAPIWebhookWithTokenResult = RESTGetAPIWebhookWithTokenResult;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#delete-webhook}
 */
pub type RESTDeleteAPIWebhookResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#delete-webhook-with-token}
 */
pub type RESTDeleteAPIWebhookWithTokenResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-webhook}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIWebhookWithTokenJSONBody {
    /**
     * The message contents (up to 2000 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /**
     * Override the default username of the webhook
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /**
     * Override the default avatar of the webhook
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /**
     * `true` if this is a TTS message
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    /**
     * Embedded `rich` content
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<APIEmbed>>,
    /**
     * Allowed mentions for the message
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#allowed-mentions-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<APIAllowedMentions>,
    /**
     * The components to include with the message
     *
     * Application-owned webhooks can always send components. Non-application-owned webhooks cannot send interactive components, and the `components` field will be ignored unless they set the `with_components` query param.
     *
     * @see {@link https://discord.com/developers/docs/components/reference}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<APIMessageTopLevelComponent>>,
    /**
     * Attachment objects with filename and description
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<RESTAPIAttachment>>,
    /**
     * Message flags combined as a bitfield
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    /**
     * Name of thread to create
     *
     * Available only if the webhook is in a forum channel and a thread is not specified in {@link RESTPostAPIWebhookWithTokenQuery.thread_id} query parameter
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_name: Option<String>,
    /**
     * Array of tag ids to apply to the thread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<String>>,
    /**
     * A poll!
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<RESTAPIPoll>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-webhook}
 */
pub type RESTPostAPIWebhookWithTokenFormDataBody = Value;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-webhook-query-string-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIWebhookWithTokenQuery {
    /**
     * Waits for server confirmation of message send before response, and returns the created message body
     * (when `false` a message that is not saved does not return an error)
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    /**
     * Send a message to the specified thread within a webhook's channel. The thread will automatically be unarchived.
     *
     * Available only if the {@link RESTPostAPIWebhookWithTokenJSONBody.thread_name} JSON body property is not specified
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /**
     * Whether to allow sending (non-interactive) components for non-application-owned webhooks
     * (ignored for application-owned webhooks)
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_components: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-webhook}
 */
pub type RESTPostAPIWebhookWithTokenResult = ();

/**
 * Received when a call to https://discord.com/developers/docs/resources/webhook#execute-webhook receives
 * the `wait` query parameter set to `true`
 *
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-webhook-query-string-params}
 */
pub type RESTPostAPIWebhookWithTokenWaitResult = APIMessage;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-slackcompatible-webhook-query-string-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIWebhookWithTokenSlackQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-slackcompatible-webhook}
 */
pub type RESTPostAPIWebhookWithTokenSlackResult = ();

/**
 * Received when a call to https://discord.com/developers/docs/resources/webhook#execute-webhook receives
 * the `wait` query parameter set to `true`
 *
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-slackcompatible-webhook-query-string-params}
 */
pub type RESTPostAPIWebhookWithTokenSlackWaitResult = APIMessage;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-githubcompatible-webhook-query-string-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIWebhookWithTokenGitHubQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-githubcompatible-webhook}
 */
pub type RESTPostAPIWebhookWithTokenGitHubResult = ();

/**
 * Received when a call to https://discord.com/developers/docs/resources/webhook#execute-webhook receives
 * the `wait` query parameter set to `true`
 *
 * @see {@link https://discord.com/developers/docs/resources/webhook#execute-githubcompatible-webhook-query-string-params}
 */
pub type RESTPostAPIWebhookWithTokenGitHubWaitResult = APIMessage;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#get-webhook-message}
 */
pub type RESTGetAPIWebhookWithTokenMessageResult = APIMessage;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#get-webhook-message-query-string-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIWebhookWithTokenMessageQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#edit-webhook-message}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPatchAPIWebhookWithTokenMessageJSONBody {
    /**
     * Allowed mentions for the message
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#allowed-mentions-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<Option<APIAllowedMentions>>,
    /**
     * The components to include with the message
     *
     * @see {@link https://discord.com/developers/docs/components/reference}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Option<Vec<APIMessageTopLevelComponent>>>,
    /**
     * The message contents (up to 2000 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
    /**
     * Embedded `rich` content
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Option<Vec<APIEmbed>>>,
    /**
     * Attached files to keep
     *
     * Starting with API v10, the `attachments` array must contain all attachments that should be present after edit, including **retained and new** attachments provided in the request body.
     *
     * @see {@link https://discord.com/developers/docs/resources/message#attachment-object-attachment-structure}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<RESTAPIAttachment>>,
    /**
     * A poll!
     *
     * @remarks
     * Polls can only be added when editing a deferred interaction response.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<RESTAPIPoll>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#edit-webhook-message}
 */
pub type RESTPatchAPIWebhookWithTokenMessageFormDataBody = Value;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#edit-webhook-message-query-string-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIWebhookWithTokenMessageQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_components: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#edit-webhook-message}
 */
pub type RESTPatchAPIWebhookWithTokenMessageResult = APIMessage;

/**
 * @see {@link https://discord.com/developers/docs/resources/webhook#delete-webhook-message}
 */
pub type RESTDeleteAPIWebhookWithTokenMessageResult = ();
