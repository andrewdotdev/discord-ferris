// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use crate::models::payloads::base::{
    APIBaseInteraction, APIDMInteractionWrapper, APIGuildInteractionWrapper,
    APIInteractionDataResolved, APIUserInteractionDataResolved,
};
use crate::models::payloads::channel::ComponentType;
use crate::models::payloads::responses::InteractionType;

use serde::{Deserialize, Serialize};

/**
 * The base interaction for message components with strongly-typed `data`.
 */
pub type APIMessageComponentInteraction =
    APIBaseInteraction<InteractionType, APIMessageComponentInteractionData>;

/**
 * Specialized interaction where the component is a button.
 */
pub type APIMessageComponentButtonInteraction =
    APIBaseInteraction<InteractionType, APIMessageButtonInteractionData>;

/**
 * Specialized interaction where the component is any select menu.
 */
pub type APIMessageComponentSelectMenuInteraction =
    APIBaseInteraction<InteractionType, APIMessageSelectMenuInteractionData>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIMessageComponentInteractionData {
    APIMessageButtonInteractionData(APIMessageButtonInteractionData),
    APIMessageSelectMenuInteractionData(APIMessageSelectMenuInteractionData),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageComponentBaseInteractionData {
    /**
     * The `custom_id` of the component
     */
    pub custom_id: String,
    /**
     * The type of the component
     */
    pub component_type: ComponentType,
}

/**
 * Button component interaction payload.
 */
pub type APIMessageButtonInteractionData = APIMessageComponentBaseInteractionData;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageStringSelectInteractionData {
    #[serde(flatten)]
    pub base: APIMessageComponentBaseInteractionData,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageUserSelectInteractionData {
    #[serde(flatten)]
    pub base: APIMessageComponentBaseInteractionData,
    pub values: Vec<String>,
    pub resolved: APIUserInteractionDataResolved,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageRoleSelectInteractionData {
    #[serde(flatten)]
    pub base: APIMessageComponentBaseInteractionData,
    pub values: Vec<String>,
    /**
     * Uses the resolved structure where `roles` is expected to be present.
     */
    pub resolved: APIInteractionDataResolved,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageMentionableSelectInteractionData {
    #[serde(flatten)]
    pub base: APIMessageComponentBaseInteractionData,
    pub values: Vec<String>,
    /**
     * Uses the resolved structure, typically including `members`, `roles`, and `users`.
     */
    pub resolved: APIInteractionDataResolved,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageChannelSelectInteractionData {
    #[serde(flatten)]
    pub base: APIMessageComponentBaseInteractionData,
    pub values: Vec<String>,
    /**
     * Uses the resolved structure where `channels` is expected to be present.
     */
    pub resolved: APIInteractionDataResolved,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIMessageSelectMenuInteractionData {
    APIMessageChannelSelectInteractionData(APIMessageChannelSelectInteractionData),
    APIMessageMentionableSelectInteractionData(APIMessageMentionableSelectInteractionData),
    APIMessageRoleSelectInteractionData(APIMessageRoleSelectInteractionData),
    APIMessageStringSelectInteractionData(APIMessageStringSelectInteractionData),
    APIMessageUserSelectInteractionData(APIMessageUserSelectInteractionData),
}

pub type APIMessageComponentDMInteraction = APIDMInteractionWrapper<APIMessageComponentInteraction>;

pub type APIMessageComponentGuildInteraction =
    APIGuildInteractionWrapper<APIMessageComponentInteraction>;
