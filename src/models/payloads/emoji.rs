// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use serde::{Deserialize, Serialize};

use crate::models::payloads::user::APIUser;

/**
 * Types extracted from https://discord.com/developers/docs/resources/emoji
 */

/**
 * Not documented but mentioned
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIPartialEmoji {
    /**
     * Emoji id
     */
    pub id: Option<String>,
    /**
     * Emoji name (can be null only in reaction emoji objects)
     */
    pub name: Option<String>,
    /**
     * Whether this emoji is animated
     */
    pub animated: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#emoji-object-emoji-structure}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIEmoji {
    /**
     * Roles this emoji is whitelisted to
     */
    pub roles: Option<Vec<String>>,
    /**
     * User that created this emoji
     */
    pub user: Option<APIUser>,
    /**
     * Whether this emoji must be wrapped in colons
     */
    pub require_colons: Option<bool>,
    /**
     * Whether this emoji is managed
     */
    pub managed: Option<bool>,
    /**
     * Whether this emoji can be used, may be false due to loss of Server Boosts
     */
    pub available: Option<bool>,

    #[serde(flatten)]
    pub base: APIPartialEmoji,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#emoji-object-applicationowned-emoji}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIApplicationEmoji {
    /**
     * Roles allowed to use this emoji.
     *
     * @remarks Always empty.
     */
    pub roles: Vec<String>,
    /**
     * Whether this emoji must be wrapped in colons.
     *
     * @remarks Always `true`.
     */
    pub require_colons: bool,
    /**
     * Whether this emoji is managed.
     *
     * @remarks Always `false`.
     */
    pub managed: bool,
    /**
     * Whether this emoji is available.
     *
     * @remarks Always `true`.
     */
    pub available: bool,

    #[serde(flatten)]
    pub base: APIApplicationEmojiRequiredFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIApplicationEmojiRequiredFields {
    pub animated: bool,
    pub id: String,
    pub name: String,
    pub user: APIUser,
}
