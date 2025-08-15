use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/**
 * Types extracted from https://discord.com/developers/docs/topics/permissions
 */

/**
 * @see {@link https://discord.com/developers/docs/topics/permissions#role-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIRole {
    /**
     * Role id
     */
    pub id: String,
    /**
     * Role name
     */
    pub name: String,
    /**
     * Integer representation of hexadecimal color code
     *
     * @remarks `color` will still be returned by the API, but using the `colors` field is recommended when doing requests.
     */
    pub color: u32,
    /**
     * The role's colors
     */
    pub colors: Option<APIRoleColors>,
    /**
     * If this role is pinned in the user listing
     */
    pub hoist: bool,
    /**
     * The role icon hash
     */
    pub icon: Option<String>,
    /**
     * The role unicode emoji as a standard emoji
     */
    pub unicode_emoji: Option<String>,
    /**
     * Position of this role
     */
    pub position: i32,
    /**
     * Permission bit set
     *
     * @see {@link https://en.wikipedia.org/wiki/Bit_field}
     */
    pub permissions: String,
    /**
     * Whether this role is managed by an integration
     */
    pub managed: bool,
    /**
     * Whether this role is mentionable
     */
    pub mentionable: bool,
    /**
     * The tags this role has
     */
    pub tags: Option<APIRoleTags>,
    /**
     * Role flags
     */
    pub flags: RoleFlags,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIRoleTags {
    /**
     * The id of the bot this role belongs to
     */
    pub bot_id: Option<String>,
    /**
     * Whether this is the guild's premium subscriber role
     */
    pub premium_subscriber: Option<()>,
    /**
     * The id of the integration this role belongs to
     */
    pub integration_id: Option<String>,
    /**
     * The id of this role's subscription sku and listing
     */
    pub subscription_listing_id: Option<String>,
    /**
     * Whether this role is available for purchase
     */
    pub available_for_purchase: Option<()>,
    /**
     * Whether this role is a guild's linked role
     */
    pub guild_connections: Option<()>,
}

bitflags! {
    /**
    * @see {@link https://discord.com/developers/docs/topics/permissions#role-object-role-flags}
    */
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct RoleFlags: u32 {
        /**
         * Role can be selected by members in an onboarding prompt
         */
        const InPrompt = 1 << 0;
    }
}

/**
 * @see {@link https://discord.com/developers/docs/topics/permissions#role-colors-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIRoleColors {
    /**
     * The primary color for the role
     */
    pub primary_color: u32,
    /**
     * The secondary color for the role, this will make the role a gradient between the other provided colors
     */
    pub secondary_color: Option<u32>,
    /**
     * The tertiary color for the role, this will turn the gradient into a holographic style
     *
     * @remarks When sending `tertiary_color` the API enforces the role color to be a holographic style with values of `primary_color = 11127295`, `secondary_color = 16759788`, and `tertiary_color = 16761760`.
     */
    pub tertiary_color: Option<u32>,
}
