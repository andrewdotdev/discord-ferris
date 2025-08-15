use serde::{Deserialize, Serialize};

use super::user::APIUser;

/**
 * Types extracted from https://discord.com/developers/docs/resources/soundboard
 */

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#soundboard-sound-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APISoundboardSound {
    /**
     * The name of this sound
     */
    pub name: String,
    /**
     * The id of this sound
     */
    pub sound_id: String,
    /**
     * The volume of this sound, from 0 to 1
     */
    pub volume: f64,
    /**
     * The id of this sound's custom emoji
     */
    pub emoji_id: Option<String>,
    /**
     * The unicode character of this sound's standard emoji
     */
    pub emoji_name: Option<String>,
    /**
     * The id of the guild that this sound is in
     */
    pub guild_id: Option<String>,
    /**
     * Whether this sound can be used (for guild sounds), may be false due to loss of Server Boosts
     */
    pub available: bool,
    /**
     * The user who created this sound
     */
    pub user: Option<APIUser>,
}
