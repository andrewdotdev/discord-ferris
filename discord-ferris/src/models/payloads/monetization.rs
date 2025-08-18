// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 * Types extracted from https://discord.com/developers/docs/monetization/entitlements#entitlement-object-entitlement-structure
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIEntitlement {
    /**
     * ID of the entitlement
     */
    pub id: String,
    /**
     * ID of the SKU
     */
    pub sku_id: String,
    /**
     * ID of the user that is granted access to the entitlement's sku
     */
    pub user_id: Option<String>,
    /**
     * ID of the guild that is granted access to the entitlement's sku
     */
    pub guild_id: Option<String>,
    /**
     * ID of the parent application
     */
    pub application_id: String,
    /**
     * Type of entitlement
     */
    pub r#type: EntitlementType,
    /**
     * Whether the entitlement was deleted
     */
    pub deleted: bool,
    /**
     * Start date at which the entitlement is valid.
     */
    pub starts_at: Option<String>,
    /**
     * Date at which the entitlement is no longer valid.
     */
    pub ends_at: Option<String>,
    /**
     * For consumable items, whether or not the entitlement has been consumed
     */
    pub consumed: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/monetization/entitlements#entitlement-object-entitlement-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EntitlementType {
    /**
     * Entitlement was purchased by user
     */
    Purchase = 1,
    /**
     * Entitlement for Discord Nitro subscription
     */
    PremiumSubscription = 2,
    /**
     * Entitlement was gifted by developer
     */
    DeveloperGift = 3,
    /**
     * Entitlement was purchased by a dev in application test mode
     */
    TestModePurchase = 4,
    /**
     * Entitlement was granted when the SKU was free
     */
    FreePurchase = 5,
    /**
     * Entitlement was gifted by another user
     */
    UserGift = 6,
    /**
     * Entitlement was claimed by user for free as a Nitro Subscriber
     */
    PremiumPurchase = 7,
    /**
     * Entitlement was purchased as an app subscription
     */
    ApplicationSubscription = 8,
}

/**
 * @see {@link https://discord.com/developers/docs/monetization/skus#sku-object-sku-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APISKU {
    /**
     * ID of SKU
     */
    pub id: String,
    /**
     * Type of SKU
     */
    pub r#type: SKUType,
    /**
     * ID of the parent application
     */
    pub application_id: String,
    /**
     * Customer-facing name of your premium offering
     */
    pub name: String,
    /**
     * System-generated URL slug based on the SKU's name
     */
    pub slug: String,
    /**
     * SKU flags combined as a bitfield
     *
     * @see {@link https://en.wikipedia.org/wiki/Bit_field}
     */
    pub flags: SKUFlags,
}

bitflags! {
    /**
    * @see {@link https://discord.com/developers/docs/monetization/skus#sku-object-sku-flags}
    */
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct SKUFlags: u32 {
        /**
         * SKU is available for purchase
         */
        const Available = 1 << 2;
        /**
         * Recurring SKU that can be purchased by a user and applied to a single server.
         * Grants access to every user in that server.
         */
        const GuildSubscription = 1 << 7;
        /**
         * Recurring SKU purchased by a user for themselves. Grants access to the purchasing user in every server.
         */
        const UserSubscription = 1 << 8;
    }
}

/**
 * @see {@link https://discord.com/developers/docs/resources/sku#sku-object-sku-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SKUType {
    /**
     * Durable one-time purchase
     */
    Durable = 2,
    /**
     * Consumable one-time purchase
     */
    Consumable = 3,
    /**
     * Represents a recurring subscription
     */
    Subscription = 5,
    /**
     * System-generated group for each Subscription SKU created
     */
    SubscriptionGroup = 6,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/subscription#subscription-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APISubscription {
    /**
     * ID of the subscription
     */
    pub id: String,
    /**
     * ID of the user who is subscribed
     */
    pub user_id: String,
    /**
     * List of SKUs subscribed to
     */
    pub sku_ids: Vec<String>,
    /**
     * List of entitlements granted for this subscription
     */
    pub entitlement_ids: Vec<String>,
    /**
     * List of SKUs that this user will be subscribed to at renewal
     */
    pub renewal_sku_ids: Option<Vec<String>>,
    /**
     * Start of the current subscription period
     */
    pub current_period_start: String,
    /**
     * End of the current subscription period
     */
    pub current_period_end: String,
    /**
     * Current status of the subscription
     */
    pub status: SubscriptionStatus,
    /**
     * When the subscription was canceled
     */
    pub canceled_at: Option<String>,
    /**
     * ISO3166-1 alpha-2 country code of the payment source used to purchase the subscription. Missing unless queried with a private OAuth scope.
     */
    pub country: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/subscription#subscription-statuses}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SubscriptionStatus {
    /**
     * Subscription is active and scheduled to renew.
     */
    Active = 0,
    /**
     * Subscription is active but will not renew.
     */
    Ending = 1,
    /**
     * Subscription is inactive and not being charged.
     */
    Inactive = 2,
}
