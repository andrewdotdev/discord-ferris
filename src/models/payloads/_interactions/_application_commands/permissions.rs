// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-guild-application-command-permissions-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildApplicationCommandPermissions {
    /**
     * The id of the command or the application id if that permission applies to all commands
     */
    pub id: String,
    /**
     * The id of the application the command belongs to
     */
    pub application_id: String,
    /**
     * The id of the guild
     */
    pub guild_id: String,
    /**
     * The permissions for the command in the guild
     */
    pub permissions: Vec<APIApplicationCommandPermission>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permissions-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandPermission {
    /**
     * The id of the role, user or channel. Can also be a permission constant
     */
    pub id: String,
    /**
     * Role, user or channel
     */
    #[serde(rename = "type")]
    pub r#type: ApplicationCommandPermissionType,
    /**
     * `true` to allow, `false`, to disallow
     */
    pub permission: bool,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permission-type}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandPermissionType {
    Role = 1,
    User = 2,
    Channel = 3,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permissions-constants}
 */
pub struct APIApplicationCommandPermissionsConstant;

impl APIApplicationCommandPermissionsConstant {
    // eslint-disable-next-line unicorn/prefer-native-coercion-functions
    pub fn everyone(guild_id: String) -> String {
        guild_id
    }

    pub fn all_channels(guild_id: String) -> String {
        let n = guild_id.parse::<u128>().unwrap_or(0);
        (n.saturating_sub(1)).to_string()
    }
}
