// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use serde::{Deserialize, Serialize};

use crate::models::payloads::_application_commands::_chat_input::base::{
    APIApplicationCommandOptionBase, APIInteractionDataOptionBase,
};
use crate::models::payloads::_application_commands::_chat_input::shared::ApplicationCommandOptionType;
use crate::models::payloads::channel::ChannelType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandChannelOption {
    #[serde(flatten)]
    pub base: APIApplicationCommandOptionBase<ApplicationCommandOptionType>,
    pub channel_types: Option<Vec<ChannelType>>,
}

pub type APIApplicationCommandInteractionDataChannelOption =
    APIInteractionDataOptionBase<String, ApplicationCommandOptionType>;
