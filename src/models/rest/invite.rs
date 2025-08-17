// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use crate::models::payloads::APIInvite;
use serde::{Deserialize, Serialize};

/**
 * @see {@link https://discord.com/developers/docs/resources/invite#get-invite}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIInviteQuery {
    /**
     * Whether the invite should contain approximate member counts
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_counts: Option<bool>,
    /**
     * Whether the invite should contain the expiration date
     *
     * @deprecated The expiration date is always returned, regardless of this query parameter.
     * @see {@link https://github.com/discord/discord-api-docs/pull/7424}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_expiration: Option<bool>,
    /**
     * The guild scheduled event to include with the invite
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_event_id: Option<String>,
}

pub type RESTGetAPIInviteResult = APIInvite;

/**
 * @see {@link https://discord.com/developers/docs/resources/invite#delete-invite}
 */
pub type RESTDeleteAPIInviteResult = APIInvite;
