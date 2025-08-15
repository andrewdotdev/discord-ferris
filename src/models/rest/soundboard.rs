use crate::models::payloads::APISoundboardSound;
use serde::{Deserialize, Serialize};

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#send-soundboard-sound}
 */
pub type RESTPostAPISendSoundboardSoundResult = APISoundboardSound;

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#send-soundboard-sound-json-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPISoundboardSendSoundJSONBody {
    /**
     * The id of the soundboard sound to play
     */
    pub sound_id: String,
    /**
     * The id of the guild the soundboard sound is from, required to play sounds from different servers
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_guild_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#list-soundboard-default-sounds}
 */
pub type RESTGetAPISoundboardDefaultSoundsResult = Vec<APISoundboardSound>;

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#list-guild-soundboard-sounds}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTGetAPIGuildSoundboardSoundsResult {
    pub items: Vec<APISoundboardSound>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#get-guild-soundboard-sound}
 */
pub type RESTGetAPIGuildSoundboardSoundResult = APISoundboardSound;

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#create-guild-soundboard-sound-json-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIGuildSoundboardSoundJSONBody {
    /**
     * The name of the soundboard sound (2-32 characters)
     */
    pub name: String,
    /**
     * The data uri of the mp3 or ogg sound data, base64 encoded, similar to image data
     *
     * @see {@link https://discord.com/developers/docs/reference#image-data}
     */
    pub sound: String,
    /**
     * The volume of the soundboard sound, from 0 to 1
     *
     * @defaultValue `1`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Option<f32>>,
    /**
     * The id of the custom emoji for the soundboard sound
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<Option<String>>,
    /**
     * The unicode character of a standard emoji for the soundboard sound
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#create-guild-soundboard-sound}
 */
pub type RESTPostAPIGuildSoundboardSoundResult = APISoundboardSound;

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#modify-guild-soundboard-sound-json-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPatchAPIGuildSoundboardSoundJSONBody {
    /**
     * The name of the soundboard sound (2-32 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**
     * The volume of the soundboard sound, from 0 to 1
     *
     * @defaultValue `1`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Option<f32>>,
    /**
     * The id of the custom emoji for the soundboard sound
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<Option<String>>,
    /**
     * The unicode character of a standard emoji for the soundboard sound
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#modify-guild-soundboard-sound}
 */
pub type RESTPatchAPIGuildSoundboardSoundResult = APISoundboardSound;

/**
 * @see {@link https://discord.com/developers/docs/resources/soundboard#delete-guild-soundboard-sound}
 */
pub type RESTDeleteAPIGuildSoundboardSoundResult = ();
