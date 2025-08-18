// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use serde::{Deserialize, Serialize};

pub use crate::models::payloads::_interactions::application_commands::*;
pub use crate::models::payloads::_interactions::autocomplete::*;
pub use crate::models::payloads::_interactions::base::*;
pub use crate::models::payloads::_interactions::message_components::*;
pub use crate::models::payloads::_interactions::modal_submit::*;
pub use crate::models::payloads::_interactions::ping::*;
pub use crate::models::payloads::_interactions::responses::*;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIInteraction {
    APIApplicationCommandAutocompleteInteraction(APIApplicationCommandAutocompleteInteraction),
    APIApplicationCommandInteraction(APIApplicationCommandInteraction),
    APIMessageComponentInteraction(APIMessageComponentInteraction),
    APIModalSubmitInteraction(APIModalSubmitInteraction),
    APIPingInteraction(APIPingInteraction),
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIDMInteraction {
    APIApplicationCommandAutocompleteDMInteraction(APIApplicationCommandAutocompleteDMInteraction),
    APIApplicationCommandDMInteraction(APIApplicationCommandDMInteraction),
    APIMessageComponentDMInteraction(APIMessageComponentDMInteraction),
    APIModalSubmitDMInteraction(APIModalSubmitDMInteraction),
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIGuildInteraction {
    APIApplicationCommandAutocompleteGuildInteraction(
        APIApplicationCommandAutocompleteGuildInteraction,
    ),
    APIApplicationCommandGuildInteraction(APIApplicationCommandGuildInteraction),
    APIMessageComponentGuildInteraction(APIMessageComponentGuildInteraction),
    APIModalSubmitGuildInteraction(APIModalSubmitGuildInteraction),
}
