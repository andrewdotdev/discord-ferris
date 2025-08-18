// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::payloads::_application_commands::internals::APIBaseApplicationCommandInteractionData;
use crate::models::payloads::application_commands::APIApplicationCommandInteractionWrapper;
use crate::models::payloads::base::{
    APIDMInteractionWrapper, APIGuildInteractionWrapper, APIUserInteractionDataResolved,
};
use crate::models::payloads::channel::APIMessage;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIUserApplicationCommandInteractionData {
    #[serde(flatten)]
    pub base: APIBaseApplicationCommandInteractionData,
    pub target_id: String,
    pub resolved: APIUserInteractionDataResolved,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageApplicationCommandInteractionData {
    #[serde(flatten)]
    pub base: APIBaseApplicationCommandInteractionData,
    pub target_id: String,
    pub resolved: APIMessageApplicationCommandInteractionDataResolved,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-resolved-data-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageApplicationCommandInteractionDataResolved {
    pub messages: HashMap<String, APIMessage>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIContextMenuInteractionData {
    APIMessageApplicationCommandInteractionData(APIMessageApplicationCommandInteractionData),
    APIUserApplicationCommandInteractionData(APIUserApplicationCommandInteractionData),
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIUserApplicationCommandInteraction =
    APIApplicationCommandInteractionWrapper<APIUserApplicationCommandInteractionData>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIUserApplicationCommandDMInteraction =
    APIDMInteractionWrapper<APIUserApplicationCommandInteraction>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIUserApplicationCommandGuildInteraction =
    APIGuildInteractionWrapper<APIUserApplicationCommandInteraction>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIMessageApplicationCommandInteraction =
    APIApplicationCommandInteractionWrapper<APIMessageApplicationCommandInteractionData>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIMessageApplicationCommandDMInteraction =
    APIDMInteractionWrapper<APIMessageApplicationCommandInteraction>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIMessageApplicationCommandGuildInteraction =
    APIGuildInteractionWrapper<APIMessageApplicationCommandInteraction>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIContextMenuInteraction {
    APIMessageApplicationCommandInteraction(APIMessageApplicationCommandInteraction),
    APIUserApplicationCommandInteraction(APIUserApplicationCommandInteraction),
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIContextMenuDMInteraction {
    APIMessageApplicationCommandDMInteraction(APIMessageApplicationCommandDMInteraction),
    APIUserApplicationCommandDMInteraction(APIUserApplicationCommandDMInteraction),
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIContextMenuGuildInteraction {
    APIMessageApplicationCommandGuildInteraction(APIMessageApplicationCommandGuildInteraction),
    APIUserApplicationCommandGuildInteraction(APIUserApplicationCommandGuildInteraction),
}
