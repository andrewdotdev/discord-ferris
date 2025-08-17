// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use serde::{Deserialize, Serialize};

use crate::models::payloads::_application_commands::_chat_input::base::{
    APIApplicationCommandOptionBase, APIApplicationCommandOptionWithAutocompleteOrChoicesWrapper,
    APIInteractionDataOptionBase,
};
use crate::models::payloads::_application_commands::_chat_input::shared::{
    APIApplicationCommandOptionChoice, ApplicationCommandOptionType,
};
use crate::models::payloads::responses::InteractionType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandIntegerOptionBase {
    #[serde(flatten)]
    pub base: APIApplicationCommandOptionBase<ApplicationCommandOptionType>,
    /**
     * If the option is an `INTEGER` or `NUMBER` type, the minimum value permitted.
     */
    pub min_value: Option<f64>,
    /**
     * If the option is an `INTEGER` or `NUMBER` type, the maximum value permitted.
     */
    pub max_value: Option<f64>,
}

pub type APIApplicationCommandIntegerOption =
    APIApplicationCommandOptionWithAutocompleteOrChoicesWrapper<
        APIApplicationCommandIntegerOptionBase,
        APIApplicationCommandOptionChoice<i64>,
    >;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntegerOptionValue {
    String(String),
    Integer(i64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandInteractionDataIntegerOption<Type = InteractionType> {
    #[serde(flatten)]
    pub base: APIInteractionDataOptionBase<IntegerOptionValue, ApplicationCommandOptionType>,
    pub focused: Option<bool>,
    #[serde(skip)]
    pub _phantom: std::marker::PhantomData<Type>,
}
