use serde::{Deserialize, Serialize};

use crate::models::payloads::_application_commands::_chat_input::base::APIApplicationCommandOptionBase;
use crate::models::payloads::_application_commands::_chat_input::shared::ApplicationCommandOptionType;
use crate::models::payloads::_application_commands::_chat_input::subcommand::{
    APIApplicationCommandInteractionDataSubcommandOption, APIApplicationCommandSubcommandOption,
};
use crate::models::payloads::responses::InteractionType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandSubcommandGroupOption {
    #[serde(flatten)]
    pub base: APIApplicationCommandOptionBase<ApplicationCommandOptionType>,
    pub options: Option<Vec<APIApplicationCommandSubcommandOption>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandInteractionDataSubcommandGroupOption<Type = InteractionType> {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: ApplicationCommandOptionType,
    pub options: Vec<APIApplicationCommandInteractionDataSubcommandOption<Type>>,
}
