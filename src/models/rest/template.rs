use crate::models::payloads::{APIGuild, APITemplate};
use serde::{Deserialize, Serialize};

/// @see {@link https://discord.com/developers/docs/resources/guild-template#get-guild-template}
pub type RESTGetAPITemplateResult = APITemplate;

/**
 * @see {@link https://discord.com/developers/docs/change-log#guild-create-deprecation}
 * @deprecated
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RESTPostAPITemplateCreateGuildJSONBody {
    /**
     * Name of the guild (2-100 characters)
     */
    pub name: String,
    /**
     * base64 1024x1024 png/jpeg image for the guild icon
     *
     * @see {@link https://discord.com/developers/docs/reference#image-data}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/change-log#guild-create-deprecation}
 * @deprecated
 */
pub type RESTPostAPITemplateCreateGuildResult = APIGuild;

/// @see {@link https://discord.com/developers/docs/resources/guild-template#get-guild-templates}
pub type RESTGetAPIGuildTemplatesResult = Vec<APITemplate>;

/**
 * @see {@link https://discord.com/developers/docs/resources/guild-template#create-guild-template}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RESTPostAPIGuildTemplatesJSONBody {
    /**
     * Name of the template (1-100 characters)
     */
    pub name: String,
    /**
     * Description for the template (0-120 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

/// @see {@link https://discord.com/developers/docs/resources/guild-template#create-guild-template}
pub type RESTPostAPIGuildTemplatesResult = APITemplate;

/// @see {@link https://discord.com/developers/docs/resources/guild-template#sync-guild-template}
pub type RESTPutAPIGuildTemplateSyncResult = APITemplate;

/// @see {@link https://discord.com/developers/docs/resources/guild-template#modify-guild-template}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RESTPatchAPIGuildTemplateJSONBody {
    /**
     * Name of the template (1-100 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**
     * Description for the template (0-120 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

/// @see {@link https://discord.com/developers/docs/resources/guild-template#modify-guild-template}
pub type RESTPatchAPIGuildTemplateResult = APITemplate;

/// @see {@link https://discord.com/developers/docs/resources/guild-template#delete-guild-template}
pub type RESTDeleteAPIGuildTemplateResult = APITemplate;
