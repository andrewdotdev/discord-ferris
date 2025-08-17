// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use crate::models::payloads::{
    APIApplicationRoleConnection, APIChannel, APIConnection, APIGuildMember, APIUser, GuildFeature,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/**
 * @see {@link https://discord.com/developers/docs/resources/user#get-current-user}
 */
pub type RESTGetAPICurrentUserResult = APIUser;

/**
 * @see {@link https://discord.com/developers/docs/resources/user#get-user}
 */
pub type RESTGetAPIUserResult = APIUser;

/**
 * @see {@link https://discord.com/developers/docs/resources/user#get-current-user-guild-member}
 */
pub type RESTGetCurrentUserGuildMemberResult = APIGuildMember;

/**
 * @see {@link https://discord.com/developers/docs/resources/user#modify-current-user}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RESTPatchAPICurrentUserJSONBody {
    /**
     * User's username, if changed may cause the user's discriminator to be randomized
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /**
     * If passed, modifies the user's avatar
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
    /**
     * If passed, modifies the user's banner
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Option<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#modify-current-user}
 */
pub type RESTPatchAPICurrentUserResult = APIUser;

/**
 * @see {@link https://discord.com/developers/docs/resources/user#get-current-user-guilds}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RESTGetAPICurrentUserGuildsQuery {
    /**
     * Get guilds before this guild ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /**
     * Get guilds after this guild ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /**
     * Max number of guilds to return (1-200)
     *
     * @defaultValue `200`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /**
     * Include approximate member and presence counts in response
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_counts: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RESTAPIPartialCurrentUserGuild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub owner: bool,
    pub features: Vec<GuildFeature>,
    pub permissions: String,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#get-current-user-guilds}
 */
pub type RESTGetAPICurrentUserGuildsResult = Vec<RESTAPIPartialCurrentUserGuild>;

/**
 * @see {@link https://discord.com/developers/docs/resources/user#leave-guild}
 */
pub type RESTDeleteAPICurrentUserGuildResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/user#create-dm}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RESTPostAPICurrentUserCreateDMChannelJSONBody {
    /**
     * The recipient to open a DM channel with
     */
    pub recipient_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#create-dm}
 */
pub type RESTPostAPICurrentUserCreateDMChannelResult = APIChannel;

/**
 * @see {@link https://discord.com/developers/docs/resources/user#get-user-connections}
 */
pub type RESTGetAPICurrentUserConnectionsResult = Vec<APIConnection>;

/**
 * @see {@link https://discord.com/developers/docs/resources/user#get-user-application-role-connection}
 */
pub type RESTGetAPICurrentUserApplicationRoleConnectionResult = APIApplicationRoleConnection;

/**
 * @see {@link https://discord.com/developers/docs/resources/user#update-user-application-role-connection}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RESTPutAPICurrentUserApplicationRoleConnectionJSONBody {
    /**
     * The vanity name of the platform a bot has connected (max 50 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    /**
     * The username on the platform a bot has connected (max 100 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_username: Option<String>,
    /**
     * Object mapping application role connection metadata keys to their `string`-ified value (max 100 characters) for the user on the platform a bot has connected
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#update-user-application-role-connection}
 */
pub type RESTPutAPICurrentUserApplicationRoleConnectionResult = APIApplicationRoleConnection;
