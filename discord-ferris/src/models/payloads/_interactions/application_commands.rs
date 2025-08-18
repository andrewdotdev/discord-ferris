// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use crate::models::payloads::_interactions::_application_commands::entry_point::{
    APIPrimaryEntryPointCommandDMInteraction, APIPrimaryEntryPointCommandGuildInteraction,
    APIPrimaryEntryPointCommandInteraction, APIPrimaryEntryPointCommandInteractionData,
};

pub use crate::models::payloads::_interactions::_application_commands::chat_input::*;
pub use crate::models::payloads::_interactions::_application_commands::context_menu::*;
#[allow(unused_imports)]
pub use crate::models::payloads::_interactions::_application_commands::entry_point::*;
pub use crate::models::payloads::_interactions::_application_commands::internals::*;
pub use crate::models::payloads::_interactions::_application_commands::permissions::*;

use crate::models::payloads::_interactions::base::APIBaseInteraction;
use crate::models::payloads::responses::InteractionType;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#application-command-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommand {
    /**
     * Unique id of the command
     */
    pub id: String,
    /**
     * Type of the command
     */
    pub r#type: ApplicationCommandType,
    /**
     * Unique id of the parent application
     */
    pub application_id: String,
    /**
     * Guild id of the command, if not global
     */
    pub guild_id: Option<String>,
    /**
     * 1-32 character name; `CHAT_INPUT` command names must be all lowercase matching `^[-_\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}$`
     */
    pub name: String,
    /**
     * Localization dictionary for the name field. Values follow the same restrictions as name
     */
    pub name_localizations: Option<crate::models::payloads::LocalizationMap>,
    /**
     * The localized name
     */
    pub name_localized: Option<String>,
    /**
     * 1-100 character description for `CHAT_INPUT` commands, empty string for `USER` and `MESSAGE` commands
     */
    pub description: String,
    /**
     * Localization dictionary for the description field. Values follow the same restrictions as description
     */
    pub description_localizations: Option<crate::models::payloads::LocalizationMap>,
    /**
     * The localized description
     */
    pub description_localized: Option<String>,
    /**
     * The parameters for the `CHAT_INPUT` command, max 25
     */
    pub options: Option<Vec<APIApplicationCommandOption>>,
    /**
     * Set of permissions represented as a bitset
     */
    pub default_member_permissions: Option<String>,
    /**
     * Indicates whether the command is available in DMs with the app, only for globally-scoped commands. By default, commands are visible
     *
     * @deprecated Use {@link APIApplicationCommand.contexts} instead
     */
    pub dm_permission: Option<bool>,
    /**
     * Whether the command is enabled by default when the app is added to a guild
     *
     * If missing, this property should be assumed as `true`
     *
     * @deprecated Use {@link APIApplicationCommand.dm_permission} and/or {@link APIApplicationCommand.default_member_permissions} instead
     */
    pub default_permission: Option<bool>,
    /**
     * Indicates whether the command is age-restricted
     *
     * @defaultValue `false`
     */
    pub nsfw: Option<bool>,
    /**
     * Installation context(s) where the command is available, only for globally-scoped commands
     *
     * @defaultValue `[ApplicationIntegrationType.GuildInstall]`
     */
    pub integration_types: Option<Vec<ApplicationIntegrationType>>,
    /**
     * Interaction context(s) where the command can be used, only for globally-scoped commands
     *
     * @defaultValue `[InteractionContextType.Guild, InteractionContextType.BotDM, InteractionContextType.PrivateChannel]`
     */
    pub contexts: Option<Vec<InteractionContextType>>,
    /**
     * Autoincrementing version identifier updated during substantial record changes
     */
    pub version: String,
    /**
     * Determines whether the interaction is handled by the app's interactions handler or by Discord
     *
     * @remarks
     * This is only available for {@link ApplicationCommandType.PrimaryEntryPoint} commands
     */
    pub handler: Option<EntryPointCommandHandlerType>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types}
 */
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
pub enum ApplicationCommandType {
    /**
     * Slash commands; a text-based command that shows up when a user types `/`
     */
    ChatInput = 1,
    /**
     * A UI-based command that shows up when you right click or tap on a user
     */
    User = 2,
    /**
     * A UI-based command that shows up when you right click or tap on a message
     */
    Message = 3,
    /**
     * A UI-based command that represents the primary way to invoke an app's Activity
     */
    PrimaryEntryPoint = 4,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/application#application-object-application-integration-types}
 */
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
pub enum ApplicationIntegrationType {
    /**
     * App is installable to servers
     */
    GuildInstall = 0,
    /**
     * App is installable to users
     */
    UserInstall = 1,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-context-types}
 */
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
pub enum InteractionContextType {
    /**
     * Interaction can be used within servers
     */
    Guild = 0,
    /**
     * Interaction can be used within DMs with the app's bot user
     */
    BotDM = 1,
    /**
     * Interaction can be used within Group DMs and DMs other than the app's bot user
     */
    PrivateChannel = 2,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#application-command-object-entry-point-command-handler-types}
 */
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
pub enum EntryPointCommandHandlerType {
    /**
     * The app handles the interaction using an interaction token
     */
    AppHandler = 1,
    /**
     * Discord handles the interaction by launching an Activity and sending a follow-up message without coordinating with
     * the app
     */
    DiscordLaunchActivity = 2,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIApplicationCommandInteractionData {
    APIChatInputApplicationCommandInteractionData(APIChatInputApplicationCommandInteractionData),
    APIContextMenuInteractionData(APIContextMenuInteractionData),
    APIPrimaryEntryPointCommandInteractionData(APIPrimaryEntryPointCommandInteractionData),
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIApplicationCommandInteractionWrapper<Data> = APIBaseInteraction<InteractionType, Data>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIApplicationCommandInteraction {
    APIChatInputApplicationCommandInteraction(APIChatInputApplicationCommandInteraction),
    APIContextMenuInteraction(APIContextMenuInteraction),
    APIPrimaryEntryPointCommandInteraction(APIPrimaryEntryPointCommandInteraction),
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIApplicationCommandDMInteraction {
    APIChatInputApplicationCommandDMInteraction(APIChatInputApplicationCommandDMInteraction),
    APIContextMenuDMInteraction(APIContextMenuDMInteraction),
    APIPrimaryEntryPointCommandDMInteraction(APIPrimaryEntryPointCommandDMInteraction),
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIApplicationCommandGuildInteraction {
    APIChatInputApplicationCommandGuildInteraction(APIChatInputApplicationCommandGuildInteraction),
    APIContextMenuGuildInteraction(APIContextMenuGuildInteraction),
    APIPrimaryEntryPointCommandGuildInteraction(APIPrimaryEntryPointCommandGuildInteraction),
}
