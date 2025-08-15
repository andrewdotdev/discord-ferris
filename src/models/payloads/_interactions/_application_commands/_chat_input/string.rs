use serde::{Deserialize, Serialize};

use crate::models::payloads::_application_commands::_chat_input::base::{
    APIApplicationCommandOptionBase, APIApplicationCommandOptionWithAutocompleteOrChoicesWrapper,
    APIInteractionDataOptionBase,
};
use crate::models::payloads::_application_commands::_chat_input::shared::{
    APIApplicationCommandOptionChoice, ApplicationCommandOptionType,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandStringOptionBase {
    #[serde(flatten)]
    pub base: APIApplicationCommandOptionBase<ApplicationCommandOptionType>,
    /**
     * For option type `STRING`, the minimum allowed length (minimum of `0`, maximum of `6000`).
     */
    pub min_length: Option<u32>,
    /**
     * For option type `STRING`, the maximum allowed length (minimum of `1`, maximum of `6000`).
     */
    pub max_length: Option<u32>,
}

pub type APIApplicationCommandStringOption =
    APIApplicationCommandOptionWithAutocompleteOrChoicesWrapper<
        APIApplicationCommandStringOptionBase,
        APIApplicationCommandOptionChoice<String>,
    >;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandInteractionDataStringOption {
    #[serde(flatten)]
    pub base: APIInteractionDataOptionBase<String, ApplicationCommandOptionType>,
    pub focused: Option<bool>,
}
