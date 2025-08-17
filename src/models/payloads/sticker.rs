// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use serde::{Deserialize, Serialize};

use super::user::APIUser;

/**
 * Types extracted from https://discord.com/developers/docs/resources/sticker
 */

/**
 * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APISticker {
    /**
     * ID of the sticker
     */
    pub id: String,
    /**
     * For standard stickers, ID of the pack the sticker is from
     */
    pub pack_id: Option<String>,
    /**
     * Name of the sticker
     */
    pub name: String,
    /**
     * Description of the sticker
     */
    pub description: Option<String>,
    /**
     * For guild stickers, the Discord name of a unicode emoji representing the sticker's expression. for standard stickers, a comma-separated list of related expressions.
     */
    pub tags: String,
    /**
     * Previously the sticker asset hash, now an empty string
     *
     * @deprecated This field is no longer documented by Discord and will be removed in v11
     * @unstable This field is no longer documented by Discord and will be removed in v11
     */
    pub asset: Option<String>,
    /**
     * Type of sticker
     *
     * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-types}
     */
    pub r#type: StickerType,
    /**
     * Type of sticker format
     *
     * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-format-types}
     */
    pub format_type: StickerFormatType,
    /**
     * Whether this guild sticker can be used, may be false due to loss of Server Boosts
     */
    pub available: Option<bool>,
    /**
     * ID of the guild that owns this sticker
     */
    pub guild_id: Option<String>,
    /**
     * The user that uploaded the guild sticker
     */
    pub user: Option<APIUser>,
    /**
     * The standard sticker's sort order within its pack
     */
    pub sort_value: Option<i32>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-types}
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StickerType {
    /**
     * An official sticker in a pack
     */
    Standard = 1,
    /**
     * A sticker uploaded to a guild for the guild's members
     */
    Guild,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-format-types}
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StickerFormatType {
    PNG = 1,
    APNG,
    Lottie,
    GIF,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-item-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIStickerItem {
    /**
     * Type of sticker format
     *
     * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-format-types}
     */
    pub format_type: StickerFormatType,
    /**
     * ID of the sticker
     */
    pub id: String,
    /**
     * Name of the sticker
     */
    pub name: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-pack-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIStickerPack {
    /**
     * ID of the sticker pack
     */
    pub id: String,
    /**
     * The stickers in the pack
     */
    pub stickers: Vec<APISticker>,
    /**
     * Name of the sticker pack
     */
    pub name: String,
    /**
     * ID of the pack's SKU
     */
    pub sku_id: String,
    /**
     * ID of a sticker in the pack which is shown as the pack's icon
     */
    pub cover_sticker_id: Option<String>,
    /**
     * Description of the sticker pack
     */
    pub description: String,
    /**
     * ID of the sticker pack's banner image
     */
    pub banner_asset_id: Option<String>,
}
