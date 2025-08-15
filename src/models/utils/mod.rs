#![allow(non_snake_case)]

use crate::models::payloads::_interactions::_application_commands::context_menu;
use crate::models::payloads::APIMessageComponentInteractionData;
use crate::models::payloads::{
    APIApplicationCommandInteraction, APIButtonComponent, APIInteraction,
    APIMessageComponentInteraction,
};

// Interactions

/**
 * A type-guard check for DM interactions
 *
 * @param interaction - The interaction to check against
 * @returns A boolean that indicates if the interaction was received in a DM channel
 */
pub fn isDMInteraction(interaction: &APIInteraction) -> bool {
    match interaction {
        APIInteraction::APIApplicationCommandAutocompleteInteraction(x) => x.guild_id.is_none(),
        APIInteraction::APIApplicationCommandInteraction(i) => match i {
            APIApplicationCommandInteraction::APIChatInputApplicationCommandInteraction(x) => {
                x.guild_id.is_none()
            }
            APIApplicationCommandInteraction::APIContextMenuInteraction(ctx) => match ctx {
                // NOTE: These variant names match our context_menu.rs definitions
                context_menu::APIContextMenuInteraction::APIUserApplicationCommandInteraction(x) => {
                    x.guild_id.is_none()
                }
                context_menu::APIContextMenuInteraction::APIMessageApplicationCommandInteraction(
                    x,
                ) => x.guild_id.is_none(),
            },
            APIApplicationCommandInteraction::APIPrimaryEntryPointCommandInteraction(x) => {
                x.guild_id.is_none()
            }
        },
        APIInteraction::APIMessageComponentInteraction(x) => x.guild_id.is_none(),
        APIInteraction::APIModalSubmitInteraction(x) => x.guild_id.is_none(),
        APIInteraction::APIPingInteraction(x) => x.guild_id.is_none(),
    }
}

/**
 * A type-guard check for guild interactions
 *
 * @param interaction - The interaction to check against
 * @returns A boolean that indicates if the interaction was received in a guild
 */
pub fn isGuildInteraction(interaction: &APIInteraction) -> bool {
    !isDMInteraction(interaction)
}

// ApplicationCommandInteractions

/**
 * A type-guard check for DM application command interactions
 *
 * @param interaction - The application command interaction to check against
 * @returns A boolean that indicates if the application command interaction was received in a DM channel
 */
pub fn isApplicationCommandDMInteraction(interaction: &APIApplicationCommandInteraction) -> bool {
    match interaction {
        APIApplicationCommandInteraction::APIChatInputApplicationCommandInteraction(x) => {
            x.guild_id.is_none()
        }
        APIApplicationCommandInteraction::APIContextMenuInteraction(ctx) => match ctx {
            context_menu::APIContextMenuInteraction::APIUserApplicationCommandInteraction(x) => {
                x.guild_id.is_none()
            }
            context_menu::APIContextMenuInteraction::APIMessageApplicationCommandInteraction(x) => {
                x.guild_id.is_none()
            }
        },
        APIApplicationCommandInteraction::APIPrimaryEntryPointCommandInteraction(x) => {
            x.guild_id.is_none()
        }
    }
}

/**
 * A type-guard check for guild application command interactions
 *
 * @param interaction - The interaction to check against
 * @returns A boolean that indicates if the application command interaction was received in a guild
 */
pub fn isApplicationCommandGuildInteraction(
    interaction: &APIApplicationCommandInteraction,
) -> bool {
    !isApplicationCommandDMInteraction(interaction)
}

// MessageComponentInteractions

/**
 * A type-guard check for DM message component interactions
 *
 * @param interaction - The message component interaction to check against
 * @returns A boolean that indicates if the message component interaction was received in a DM channel
 */
pub fn isMessageComponentDMInteraction(interaction: &APIMessageComponentInteraction) -> bool {
    interaction.guild_id.is_none()
}

/**
 * A type-guard check for guild message component interactions
 *
 * @param interaction - The interaction to check against
 * @returns A boolean that indicates if the message component interaction was received in a guild
 */
pub fn isMessageComponentGuildInteraction(interaction: &APIMessageComponentInteraction) -> bool {
    interaction.guild_id.is_some()
}

// Buttons

/**
 * A type-guard check for buttons that have a `url` attached to them.
 *
 * @param component - The button to check against
 * @returns A boolean that indicates if the button has a `url` attached to it
 */
pub fn isLinkButton(component: &APIButtonComponent) -> bool {
    matches!(component, APIButtonComponent::APIButtonComponentWithURL(_))
}

/**
 * A type-guard check for buttons that have a `custom_id` attached to them.
 *
 * @param component - The button to check against
 * @returns A boolean that indicates if the button has a `custom_id` attached to it
 */
pub fn isInteractionButton(component: &APIButtonComponent) -> bool {
    matches!(
        component,
        APIButtonComponent::APIButtonComponentWithCustomId(_)
            | APIButtonComponent::APIButtonComponentWithSKUId(_)
    )
}

// Message Components

/**
 * A type-guard check for message component interactions
 *
 * @param interaction - The interaction to check against
 * @returns A boolean that indicates if the interaction is a message component
 */
pub fn isMessageComponentInteraction(interaction: &APIInteraction) -> bool {
    matches!(
        interaction,
        APIInteraction::APIMessageComponentInteraction(_)
    )
}

/**
 * A type-guard check for button message component interactions
 *
 * @param interaction - The message component interaction to check against
 * @returns A boolean that indicates if the message component is a button
 */
pub fn isMessageComponentButtonInteraction(interaction: &APIMessageComponentInteraction) -> bool {
    matches!(
        interaction.data.as_ref(),
        Some(APIMessageComponentInteractionData::APIMessageButtonInteractionData(_))
    )
}

/**
 * A type-guard check for select menu message component interactions
 *
 * @param interaction - The message component interaction to check against
 * @returns A boolean that indicates if the message component is a select menu
 */
pub fn isMessageComponentSelectMenuInteraction(
    interaction: &APIMessageComponentInteraction,
) -> bool {
    matches!(
        interaction.data.as_ref(),
        Some(APIMessageComponentInteractionData::APIMessageSelectMenuInteractionData(_))
    )
}

// Application Commands

/**
 * A type-guard check for chat input application commands.
 *
 * @param interaction - The interaction to check against
 * @returns A boolean that indicates if the interaction is a chat input application command
 */
pub fn isChatInputApplicationCommandInteraction(
    interaction: &APIApplicationCommandInteraction,
) -> bool {
    matches!(
        interaction,
        APIApplicationCommandInteraction::APIChatInputApplicationCommandInteraction(_)
    )
}

/**
 * A type-guard check for context menu application commands.
 *
 * @param interaction - The interaction to check against
 * @returns A boolean that indicates if the interaction is a context menu application command
 */
pub fn isContextMenuApplicationCommandInteraction(
    interaction: &APIApplicationCommandInteraction,
) -> bool {
    matches!(
        interaction,
        APIApplicationCommandInteraction::APIContextMenuInteraction(_)
    )
}
