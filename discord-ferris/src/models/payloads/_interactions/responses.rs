// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use crate::models::payloads::_application_commands::_chat_input::shared::APIApplicationCommandOptionChoice;
use crate::models::payloads::channel::{APIActionRowComponent, APIComponentInModalActionRow};
use crate::models::rest::RESTPostAPIWebhookWithTokenJSONBody;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-type}
 */
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIInteractionResponse {
    APIApplicationCommandAutocompleteResponse(APIApplicationCommandAutocompleteResponse),
    APIInteractionResponseChannelMessageWithSource(APIInteractionResponseChannelMessageWithSource),
    APIInteractionResponseDeferredChannelMessageWithSource(
        APIInteractionResponseDeferredChannelMessageWithSource,
    ),
    APIInteractionResponseDeferredMessageUpdate(APIInteractionResponseDeferredMessageUpdate),
    APIInteractionResponseLaunchActivity(APIInteractionResponseLaunchActivity),
    APIInteractionResponsePong(APIInteractionResponsePong),
    APIInteractionResponseUpdateMessage(APIInteractionResponseUpdateMessage),
    APIModalInteractionResponse(APIModalInteractionResponse),
    APIPremiumRequiredInteractionResponse(APIPremiumRequiredInteractionResponse),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionResponsePong {
    pub r#type: InteractionResponseType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandAutocompleteResponse {
    pub r#type: InteractionResponseType,
    pub data: APICommandAutocompleteInteractionResponseCallbackData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIModalInteractionResponse {
    pub r#type: InteractionResponseType,
    pub data: APIModalInteractionResponseCallbackData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPremiumRequiredInteractionResponse {
    pub r#type: InteractionResponseType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionResponseChannelMessageWithSource {
    pub r#type: InteractionResponseType,
    pub data: APIInteractionResponseCallbackData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionResponseDeferredChannelMessageWithSource {
    pub r#type: InteractionResponseType,
    pub data: Option<APIInteractionResponseCallbackDataFlagsOnly>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionResponseDeferredMessageUpdate {
    pub r#type: InteractionResponseType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionResponseUpdateMessage {
    pub r#type: InteractionResponseType,
    pub data: Option<APIInteractionResponseCallbackData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionResponseLaunchActivity {
    pub r#type: InteractionResponseType,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-type}
 */
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
pub enum InteractionResponseType {
    /**
     * ACK a `Ping`
     */
    Pong = 1,
    /**
     * Respond to an interaction with a message
     */
    ChannelMessageWithSource = 4,
    /**
     * ACK an interaction and edit to a response later, the user sees a loading state
     */
    DeferredChannelMessageWithSource = 5,
    /**
     * ACK a button interaction and update it to a loading state
     */
    DeferredMessageUpdate = 6,
    /**
     * ACK a button interaction and edit the message to which the button was attached
     */
    UpdateMessage = 7,
    /**
     * For autocomplete interactions
     */
    ApplicationCommandAutocompleteResult = 8,
    /**
     * Respond to an interaction with an modal for a user to fill-out
     */
    Modal = 9,
    /**
     * Respond to an interaction with an upgrade button, only available for apps with monetization enabled
     *
     * @deprecated Send a button with Premium type instead.
     * {@link https://discord.com/developers/docs/change-log#premium-apps-new-premium-button-style-deep-linking-url-schemes | Learn more here}
     */
    PremiumRequired = 10,

    /**
     * Launch the Activity associated with the app.
     *
     * @remarks
     * Only available for apps with Activities enabled
     */
    LaunchActivity = 12,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-data-structure}
 */
pub type APIInteractionResponseCallbackData = RESTPostAPIWebhookWithTokenJSONBody;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionResponseCallbackDataFlagsOnly {
    pub flags: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APICommandAutocompleteInteractionResponseCallbackData {
    pub choices: Option<Vec<APIApplicationCommandOptionChoice>>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-modal}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIModalInteractionResponseCallbackData {
    /**
     * A developer-defined identifier for the component, max 100 characters
     */
    pub custom_id: String,
    /**
     * The title of the popup modal
     */
    pub title: String,
    /**
     * Between 1 and 5 (inclusive) components that make up the modal
     */
    pub components: Vec<APIActionRowComponent<APIComponentInModalActionRow>>,
}
