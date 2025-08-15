use crate::models::payloads::_application_commands::_chat_input::base::{
    APIApplicationCommandOptionBase, APIInteractionDataOptionBase,
};
use crate::models::payloads::_application_commands::_chat_input::shared::ApplicationCommandOptionType;

pub type APIApplicationCommandMentionableOption =
    APIApplicationCommandOptionBase<ApplicationCommandOptionType>;

pub type APIApplicationCommandInteractionDataMentionableOption =
    APIInteractionDataOptionBase<String, ApplicationCommandOptionType>;
