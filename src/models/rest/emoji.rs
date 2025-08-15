use crate::models::payloads::{APIApplicationEmoji, APIEmoji};
use serde::{Deserialize, Serialize};

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#list-guild-emojis}
 */
pub type RESTGetAPIGuildEmojisResult = Vec<APIEmoji>;

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#get-guild-emoji}
 */
pub type RESTGetAPIGuildEmojiResult = APIEmoji;

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#create-guild-emoji-json-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIGuildEmojiJSONBody {
    /**
     * Name of the emoji
     */
    pub name: String,
    /**
     * The 128x128 emoji image
     *
     * @see {@link https://discord.com/developers/docs/reference#image-data}
     */
    pub image: String,
    /**
     * Roles for which this emoji will be whitelisted
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#create-guild-emoji}
 */
pub type RESTPostAPIGuildEmojiResult = APIEmoji;

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#modify-guild-emoji}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPatchAPIGuildEmojiJSONBody {
    /**
     * Name of the emoji
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**
     * Roles for which this emoji will be whitelisted
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Option<Vec<String>>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#modify-guild-emoji}
 */
pub type RESTPatchAPIGuildEmojiResult = APIEmoji;

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#delete-guild-emoji}
 */
pub type RESTDeleteAPIGuildEmojiResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#list-application-emojis}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTGetAPIApplicationEmojisResult {
    pub items: Vec<APIApplicationEmoji>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#get-application-emoji}
 */
pub type RESTGetAPIApplicationEmojiResult = APIApplicationEmoji;

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#create-application-emoji-json-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIApplicationEmojiJSONBody {
    pub image: String,
    pub name: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#create-application-emoji}
 */
pub type RESTPostAPIApplicationEmojiResult = APIApplicationEmoji;

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#modify-application-emoji}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPatchAPIApplicationEmojiJSONBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#modify-application-emoji}
 */
pub type RESTPatchAPIApplicationEmojiResult = APIApplicationEmoji;

/**
 * @see {@link https://discord.com/developers/docs/resources/emoji#delete-application-emoji}
 */
pub type RESTDeleteAPIApplicationEmojiResult = ();
