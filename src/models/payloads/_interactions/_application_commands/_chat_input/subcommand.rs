use serde::{Deserialize, Serialize};

use crate::models::payloads::_application_commands::_chat_input::base::APIApplicationCommandOptionBase;
use crate::models::payloads::_application_commands::_chat_input::shared::ApplicationCommandOptionType;
use crate::models::payloads::_application_commands::chat_input::{
    APIApplicationCommandBasicOption, APIApplicationCommandInteractionDataBasicOption,
};
use crate::models::payloads::responses::InteractionType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandSubcommandOption {
    #[serde(flatten)]
    pub base: APIApplicationCommandOptionBase<ApplicationCommandOptionType>,
    pub options: Option<Vec<APIApplicationCommandBasicOption>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandInteractionDataSubcommandOption<Type = InteractionType> {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: ApplicationCommandOptionType,
    pub options: Option<Vec<APIApplicationCommandInteractionDataBasicOption>>,
    #[serde(skip)]
    pub _phantom: std::marker::PhantomData<Type>,
}
