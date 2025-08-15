use crate::models::payloads::{
    application_commands::APIAutocompleteApplicationCommandInteractionData,
    base::{APIBaseInteraction, APIDMInteractionWrapper, APIGuildInteractionWrapper},
    responses::InteractionType,
};

pub type APIApplicationCommandAutocompleteInteraction =
    APIBaseInteraction<InteractionType, APIAutocompleteApplicationCommandInteractionData>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIApplicationCommandAutocompleteDMInteraction =
    APIDMInteractionWrapper<APIApplicationCommandAutocompleteInteraction>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIApplicationCommandAutocompleteGuildInteraction =
    APIGuildInteractionWrapper<APIApplicationCommandAutocompleteInteraction>;
