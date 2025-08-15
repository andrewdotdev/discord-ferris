//! @see https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure

use serde::{Deserialize, Serialize};

use crate::models::payloads::_interactions::application_commands::{
    APIApplicationCommandInteractionWrapper, ApplicationCommandType,
};
use crate::models::payloads::_interactions::base::{
    APIDMInteractionWrapper, APIGuildInteractionWrapper,
};

// Re-exports mirroring `export type *` / `export *` from TS
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::attachment::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::base::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::boolean::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::channel::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::integer::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::mentionable::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::number::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::role::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::shared::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::string::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::subcommand::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::subcommand_group::*;
pub use crate::models::payloads::_interactions::_application_commands::_chat_input::user::*;
use crate::models::payloads::base::APIInteractionDataResolved;

/// Union of "basic" option definitions for an application command.
///
/// Uses `serde(untagged)` because the underlying objects have distinct shapes.
/// This mirrors the TS union type.
///
/// @see https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIApplicationCommandBasicOption {
    Attachment(APIApplicationCommandAttachmentOption),
    Boolean(APIApplicationCommandBooleanOption),
    Channel(APIApplicationCommandChannelOption),
    Integer(APIApplicationCommandIntegerOption),
    Mentionable(APIApplicationCommandMentionableOption),
    Number(APIApplicationCommandNumberOption),
    Role(APIApplicationCommandRoleOption),
    String(APIApplicationCommandStringOption),
    User(APIApplicationCommandUserOption),
}

/// Union of all option definitions (basic + subcommand/group).
///
/// @see https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIApplicationCommandOption {
    Basic(APIApplicationCommandBasicOption),
    SubcommandGroup(APIApplicationCommandSubcommandGroupOption),
    Subcommand(APIApplicationCommandSubcommandOption),
}

/// Union of interaction data option values for chat input interactions.
/// The concrete shape depends on the `InteractionType` (e.g., autocomplete vs execution).
///
/// @see https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-interaction-data-option-structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIApplicationCommandInteractionDataOption {
    Basic(APIApplicationCommandInteractionDataBasicOption),
    SubcommandGroup(APIApplicationCommandInteractionDataSubcommandGroupOption),
    Subcommand(APIApplicationCommandInteractionDataSubcommandOption),
}

/// Union of "basic" interaction data option values (no subcommands).
///
/// Integer/Number variants may carry different `value` semantics across interaction types,
/// but this union preserves the TS surface.
///
/// @see https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-interaction-data-option-structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIApplicationCommandInteractionDataBasicOption {
    Attachment(APIApplicationCommandInteractionDataAttachmentOption),
    Boolean(APIApplicationCommandInteractionDataBooleanOption),
    Channel(APIApplicationCommandInteractionDataChannelOption),
    Integer(APIApplicationCommandInteractionDataIntegerOption),
    Mentionable(APIApplicationCommandInteractionDataMentionableOption),
    Number(APIApplicationCommandInteractionDataNumberOption),
    Role(APIApplicationCommandInteractionDataRoleOption),
    String(APIApplicationCommandInteractionDataStringOption),
    User(APIApplicationCommandInteractionDataUserOption),
}

/// Interaction data payload for a chat input command execution.
/// This extends the base application command interaction data with optional
/// resolved references and provided options.
///
/// @see https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIChatInputApplicationCommandInteractionData {
    /// Discriminator for the command type (always `ChatInput`).
    #[serde(rename = "type")]
    pub r#type: ApplicationCommandType,
    /// Command name as registered.
    pub name: String,
    /// Command id (if available in the base data; kept for symmetry with TS base type).
    pub id: Option<String>,
    /// Options supplied by the user (if any).
    pub options: Option<Vec<APIApplicationCommandInteractionDataOption>>,
    /// Resolved data (users/roles/channels/attachments referenced by options).
    pub resolved: Option<APIInteractionDataResolved>,
}

/// Interaction data payload for **autocomplete** of chat input commands.
///
/// @see https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAutocompleteApplicationCommandInteractionData {
    /// Discriminator for the command type (always `ChatInput`).
    #[serde(rename = "type")]
    pub r#type: ApplicationCommandType,
    /// Command name as registered.
    pub name: String,
    /// Options being autocompleted (subset of `options` tree).
    pub options: Option<Vec<APIApplicationCommandInteractionDataOption>>,
    /// Resolved payload supplied for autocomplete (if any).
    pub resolved: Option<APIInteractionDataResolved>,
    // #[serde(flatten)]
    // pub base: APIBaseApplicationCommandInteractionData, // See note above
}

/// Wrapper type for a chat input command interaction.
///
/// @see https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
pub type APIChatInputApplicationCommandInteraction =
    APIApplicationCommandInteractionWrapper<APIChatInputApplicationCommandInteractionData>;

/// DM-scoped wrapper for a chat input command interaction.
///
/// @see https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
pub type APIChatInputApplicationCommandDMInteraction =
    APIDMInteractionWrapper<APIChatInputApplicationCommandInteraction>;

/// Guild-scoped wrapper for a chat input command interaction.
///
/// @see https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
pub type APIChatInputApplicationCommandGuildInteraction =
    APIGuildInteractionWrapper<APIChatInputApplicationCommandInteraction>;
