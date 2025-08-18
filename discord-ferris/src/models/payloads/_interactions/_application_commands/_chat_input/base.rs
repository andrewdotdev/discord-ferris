// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use serde::{Deserialize, Serialize};

use crate::models::payloads::{
    _application_commands::_chat_input::shared::{
        APIApplicationCommandOptionChoice, ApplicationCommandOptionType,
    },
    LocalizationMap,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandOptionBase<T = ApplicationCommandOptionType> {
    #[serde(rename = "type")]
    pub r#type: T,
    pub name: String,
    pub name_localizations: Option<LocalizationMap>,
    pub description: String,
    pub description_localizations: Option<LocalizationMap>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionDataOptionBase<D, T = ApplicationCommandOptionType> {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: T,
    pub value: D,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandOptionWithAutocompleteOrChoicesWrapper<
    Base,
    ChoiceType = APIApplicationCommandOptionChoice,
> {
    #[serde(flatten)]
    pub base: Base,
    pub autocomplete: Option<bool>,
    pub choices: Option<Vec<ChoiceType>>,
}
