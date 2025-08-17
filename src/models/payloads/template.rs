// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use serde::{Deserialize, Serialize};

use super::user::APIUser;
use crate::models::rest::{Locale, RESTPostAPIGuildsJSONBody};

/**
 * Types extracted from https://discord.com/developers/docs/resources/guild-template
 */

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-template#guild-template-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APITemplate {
    /**
     * The template code (unique ID)
     */
    pub code: String,
    /**
     * Template name
     */
    pub name: String,
    /**
     * The description for the template
     */
    pub description: Option<String>,
    /**
     * Number of times this template has been used
     */
    pub usage_count: u64,
    /**
     * The ID of the user who created the template
     */
    pub creator_id: String,
    /**
     * The user who created the template
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub creator: APIUser,
    /**
     * When this template was created
     */
    pub created_at: String,
    /**
     * When this template was last synced to the source guild
     */
    pub updated_at: String,
    /**
     * The ID of the guild this template is based on
     */
    pub source_guild_id: String,
    /**
     * The guild snapshot this template contains
     */
    pub serialized_source_guild: APITemplateSerializedSourceGuild,
    /**
     * Whether the template has unsynced changes
     */
    pub is_dirty: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APITemplateSerializedSourceGuild {
    #[serde(flatten)]
    pub base: RESTPostAPIGuildsJSONBody,
    pub description: Option<String>,
    pub preferred_locale: Locale,
    pub icon_hash: Option<String>,
}
