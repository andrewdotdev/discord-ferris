use crate::models::payloads::_application_commands::_chat_input::base::{
    APIApplicationCommandOptionBase, APIInteractionDataOptionBase,
};
use crate::models::payloads::_application_commands::_chat_input::shared::ApplicationCommandOptionType;

pub type APIApplicationCommandRoleOption =
    APIApplicationCommandOptionBase<ApplicationCommandOptionType>;

pub type APIApplicationCommandInteractionDataRoleOption =
    APIInteractionDataOptionBase<String, ApplicationCommandOptionType>;
