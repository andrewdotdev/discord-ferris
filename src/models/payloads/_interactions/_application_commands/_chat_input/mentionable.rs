// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use crate::models::payloads::_application_commands::_chat_input::base::{
    APIApplicationCommandOptionBase, APIInteractionDataOptionBase,
};
use crate::models::payloads::_application_commands::_chat_input::shared::ApplicationCommandOptionType;

pub type APIApplicationCommandMentionableOption =
    APIApplicationCommandOptionBase<ApplicationCommandOptionType>;

pub type APIApplicationCommandInteractionDataMentionableOption =
    APIInteractionDataOptionBase<String, ApplicationCommandOptionType>;
