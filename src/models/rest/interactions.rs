use super::webhook::{
    RESTDeleteAPIWebhookWithTokenMessageResult, RESTGetAPIWebhookWithTokenMessageResult,
    RESTPatchAPIWebhookWithTokenMessageFormDataBody, RESTPatchAPIWebhookWithTokenMessageJSONBody,
    RESTPatchAPIWebhookWithTokenMessageResult, RESTPostAPIWebhookWithTokenWaitResult,
};
use crate::models::payloads::{
    APIApplicationCommand, APIApplicationCommandPermission, APIGuildApplicationCommandPermissions,
    APIInteractionResponse, APIInteractionResponseCallbackData, APIMessage, ApplicationCommandType,
    InteractionResponseType, InteractionType,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#get-global-application-commands}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIApplicationCommandsQuery {
    /**
     * Whether to include full localization dictionaries (name_localizations and description_localizations)
     * in the returned objects, instead of the name_localized and description_localized fields.
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_localizations: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#get-global-application-commands}
 */
pub type RESTGetAPIApplicationCommandsResult = Vec<APIApplicationCommand>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#get-global-application-command}
 */
pub type RESTGetAPIApplicationCommandResult = APIApplicationCommand;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIBaseApplicationCommandsJSONBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Value>,
    #[serde(flatten)]
    pub other: Value,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#create-global-application-command}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIChatInputApplicationCommandsJSONBody {
    #[serde(flatten)]
    pub base: RESTPostAPIBaseApplicationCommandsJSONBody,
    pub description: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ApplicationCommandType>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#create-global-application-command}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIContextMenuApplicationCommandsJSONBody {
    #[serde(flatten)]
    pub base: RESTPostAPIBaseApplicationCommandsJSONBody,
    #[serde(rename = "type")]
    pub r#type: ApplicationCommandType,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#create-global-application-command}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIPrimaryEntryPointApplicationCommandJSONBody {
    #[serde(flatten)]
    pub base: RESTPostAPIBaseApplicationCommandsJSONBody,
    #[serde(rename = "type")]
    pub r#type: ApplicationCommandType,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#create-global-application-command}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RESTPostAPIApplicationCommandsJSONBody {
    ChatInput(RESTPostAPIChatInputApplicationCommandsJSONBody),
    ContextMenu(RESTPostAPIContextMenuApplicationCommandsJSONBody),
    PrimaryEntryPoint(RESTPostAPIPrimaryEntryPointApplicationCommandJSONBody),
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#create-global-application-command}
 */
pub type RESTPostAPIApplicationCommandsResult = APIApplicationCommand;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#edit-global-application-command}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIApplicationCommandJSONBody {
    #[serde(flatten)]
    pub data: Value,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#edit-global-application-command}
 */
pub type RESTPatchAPIApplicationCommandResult = APIApplicationCommand;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#bulk-overwrite-global-application-commands}
 */
pub type RESTPutAPIApplicationCommandsJSONBody = Vec<RESTPostAPIApplicationCommandsJSONBody>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#bulk-overwrite-global-application-commands}
 */
pub type RESTPutAPIApplicationCommandsResult = Vec<APIApplicationCommand>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#get-guild-application-commands}
 */
pub type RESTGetAPIApplicationGuildCommandsQuery = RESTGetAPIApplicationCommandsQuery;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#get-guild-application-commands}
 */
pub type RESTGetAPIApplicationGuildCommandsResult = Vec<APIApplicationCommand>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#get-guild-application-commands}
 */
pub type RESTGetAPIApplicationGuildCommandResult = APIApplicationCommand;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command}
 */
pub type RESTPostAPIApplicationGuildCommandsJSONBody = Value;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command}
 */
pub type RESTPostAPIApplicationGuildCommandsResult = APIApplicationCommand;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#edit-guild-application-command}
 */
pub type RESTPatchAPIApplicationGuildCommandJSONBody = Value;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#edit-guild-application-command}
 */
pub type RESTPatchAPIApplicationGuildCommandResult = APIApplicationCommand;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#bulk-overwrite-guild-application-commands}
 */
pub type RESTPutAPIApplicationGuildCommandsJSONBody = Vec<Value>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#bulk-overwrite-guild-application-commands}
 */
pub type RESTPutAPIApplicationGuildCommandsResult = Vec<APIApplicationCommand>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#create-interaction-response}
 */
pub type RESTPostAPIInteractionCallbackJSONBody = APIInteractionResponse;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#create-interaction-response}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIInteractionCallbackQuery {
    /**
     * Whether to include a interaction callback response as the response instead of a 204
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_response: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#create-interaction-response}
 */
pub type RESTPostAPIInteractionCallbackFormDataBody = Value;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#create-interaction-response}
 */
pub type RESTPostAPIInteractionCallbackResult = ();

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-callback-interaction-callback-response-object}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIInteractionCallbackWithResponseResult {
    /**
     * The interaction object associated with the interaction
     */
    pub interaction: RESTAPIInteractionCallbackObject,
    /**
     * The resource that was created by the interaction response
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<RESTAPIInteractionCallbackResourceObject>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-callback-interaction-callback-object}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIInteractionCallbackObject {
    /**
     * ID of the interaction
     */
    pub id: String,
    /**
     * Interaction type
     */
    pub r#type: InteractionType,
    /**
     * Instance ID of the Activity if one was launched or joined
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_instance_id: Option<String>,
    /**
     * ID of the message that was created by the interaction
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message_id: Option<String>,
    /**
     * Whether or not the message is in a loading state
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message_loading: Option<bool>,
    /**
     * Whether or not the response message was ephemeral
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message_ephemeral: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-callback-interaction-callback-resource-object}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIInteractionCallbackResourceObject {
    /**
     * Interaction callback type
     */
    pub r#type: InteractionResponseType,
    /**
     * Represents the Activity launched by this interaction
     *
     * @remarks
     * Only present if `type` is {@link InteractionResponseType.LaunchActivity}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_instance: Option<RESTAPIInteractionCallbackActivityInstanceResource>,
    /**
     * Message created by the interaction
     *
     * @remarks
     * Only present if `type` is {@link InteractionResponseType.ChannelMessageWithSource}
     * or {@link InteractionResponseType.UpdateMessage}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<APIMessage>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-callback-interaction-callback-activity-instance-resource}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIInteractionCallbackActivityInstanceResource {
    /**
     * Instance ID of the Activity if one was launched or joined.
     */
    pub id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#get-original-interaction-response}
 */
pub type RESTGetAPIInteractionOriginalResponseResult = RESTGetAPIWebhookWithTokenMessageResult;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#edit-original-interaction-response}
 */
pub type RESTPatchAPIInteractionOriginalResponseJSONBody =
    RESTPatchAPIWebhookWithTokenMessageJSONBody;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#edit-original-interaction-response}
 */
pub type RESTPatchAPIInteractionOriginalResponseFormDataBody =
    RESTPatchAPIWebhookWithTokenMessageFormDataBody;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#edit-original-interaction-response}
 */
pub type RESTPatchAPIInteractionOriginalResponseResult = RESTPatchAPIWebhookWithTokenMessageResult;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#delete-original-interaction-response}
 */
pub type RESTDeleteAPIInteractionOriginalResponseResult =
    RESTDeleteAPIWebhookWithTokenMessageResult;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#create-followup-message}
 */
pub type RESTPostAPIInteractionFollowupJSONBody = APIInteractionResponseCallbackData;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#create-followup-message}
 */
pub type RESTPostAPIInteractionFollowupFormDataBody = Value;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#create-followup-message}
 */
pub type RESTPostAPIInteractionFollowupResult = RESTPostAPIWebhookWithTokenWaitResult;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#get-followup-message}
 */
pub type RESTGetAPIInteractionFollowupResult = RESTGetAPIWebhookWithTokenMessageResult;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#edit-followup-message}
 */
pub type RESTPatchAPIInteractionFollowupJSONBody = RESTPatchAPIWebhookWithTokenMessageJSONBody;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#edit-followup-message}
 */
pub type RESTPatchAPIInteractionFollowupFormDataBody =
    RESTPatchAPIWebhookWithTokenMessageFormDataBody;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#edit-followup-message}
 */
pub type RESTPatchAPIInteractionFollowupResult = RESTPatchAPIWebhookWithTokenMessageResult;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#delete-followup-message}
 */
pub type RESTDeleteAPIInteractionFollowupResult = RESTDeleteAPIWebhookWithTokenMessageResult;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#get-guild-application-command-permissions}
 */
pub type RESTGetAPIGuildApplicationCommandsPermissionsResult =
    Vec<APIGuildApplicationCommandPermissions>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#get-application-command-permissions}
 */
pub type RESTGetAPIApplicationCommandPermissionsResult = APIGuildApplicationCommandPermissions;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#edit-application-command-permissions}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPutAPIApplicationCommandPermissionsJSONBody {
    pub permissions: Vec<APIApplicationCommandPermission>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#edit-application-command-permissions}
 */
pub type RESTPutAPIApplicationCommandPermissionsResult = APIGuildApplicationCommandPermissions;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#batch-edit-application-command-permissions}
 */
pub type RESTPutAPIGuildApplicationCommandsPermissionsJSONBody =
    Vec<APIGuildApplicationCommandPermissions>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#batch-edit-application-command-permissions}
 */
pub type RESTPutAPIGuildApplicationCommandsPermissionsResult =
    Vec<APIGuildApplicationCommandPermissions>;
