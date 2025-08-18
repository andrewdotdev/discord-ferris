// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use crate::models::payloads::{
    application_commands::APIAutocompleteApplicationCommandInteractionData,
    base::{APIBaseInteraction, APIDMInteractionWrapper, APIGuildInteractionWrapper},
    responses::InteractionType,
};

pub type APIApplicationCommandAutocompleteInteraction =
    APIBaseInteraction<InteractionType, APIAutocompleteApplicationCommandInteractionData>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIApplicationCommandAutocompleteDMInteraction =
    APIDMInteractionWrapper<APIApplicationCommandAutocompleteInteraction>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIApplicationCommandAutocompleteGuildInteraction =
    APIGuildInteractionWrapper<APIApplicationCommandAutocompleteInteraction>;
