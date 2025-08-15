use crate::models::payloads::_application_commands::_chat_input::base::{
    APIApplicationCommandOptionBase, APIInteractionDataOptionBase,
};
use crate::models::payloads::_application_commands::_chat_input::shared::ApplicationCommandOptionType;

pub type APIApplicationCommandBooleanOption =
    APIApplicationCommandOptionBase<ApplicationCommandOptionType>;

pub type APIApplicationCommandInteractionDataBooleanOption =
    APIInteractionDataOptionBase<bool, ApplicationCommandOptionType>;
