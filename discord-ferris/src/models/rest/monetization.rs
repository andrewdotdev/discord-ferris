// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use crate::models::payloads::{APIEntitlement, APISKU, APISubscription};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/**
 * @see {@link https://discord.com/developers/docs/resources/entitlement#list-entitlements}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIEntitlementsQuery {
    /**
     * User ID to look up entitlements for
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /**
     * Optional list of SKU IDs to check entitlements for
     * Comma-delimited set of snowflakes
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_ids: Option<String>,
    /**
     * Retrieve entitlements before this entitlement ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /**
     * Retrieve entitlements after this entitlement ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /**
     * Number of entitlements to return (1-100)
     *
     * @defaultValue `100`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /**
     * Guild ID to look up entitlements for
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /**
     * Whether ended entitlements should be omitted
     *
     * @defaultValue `false` ended entitlements are included by default
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_ended: Option<bool>,
    /**
     * Whether deleted entitlements should be omitted
     *
     * @defaultValue `true` deleted entitlements are not included by default
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_deleted: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/entitlement#list-entitlements}
 */
pub type RESTGetAPIEntitlementsResult = Vec<APIEntitlement>;

/**
 * @see {@link https://discord.com/developers/docs/resources/entitlement#get-entitlement}
 */
pub type RESTGetAPIEntitlementResult = APIEntitlement;

/**
 * @see {@link https://discord.com/developers/docs/resources/entitlement#create-test-entitlement}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIEntitlementJSONBody {
    /**
     * ID of the SKU to grant the entitlement to
     */
    pub sku_id: String,
    /**
     * ID of the guild or user to grant the entitlement to
     */
    pub owner_id: String,
    /**
     * The type of entitlement owner
     */
    pub owner_type: EntitlementOwnerType,
}

/**
 * @deprecated Use {@link RESTPostAPIEntitlementJSONBody} instead
 */
#[deprecated(note = "Use RESTPostAPIEntitlementJSONBody instead")]
pub type RESTPostAPIEntitlementBody = RESTPostAPIEntitlementJSONBody;

/**
 * @see {@link https://discord.com/developers/docs/resources/entitlement#create-test-entitlement}
 */
pub type RESTPostAPIEntitlementResult = Value;

/**
 * @see {@link https://discord.com/developers/docs/resources/entitlement#create-test-entitlement}
 */
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum EntitlementOwnerType {
    Guild = 1,
    User = 2,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/entitlement#delete-test-entitlement}
 */
pub type RESTDeleteAPIEntitlementResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/sku#list-skus}
 */
pub type RESTGetAPISKUsResult = Vec<APISKU>;

/**
 * @see {@link https://discord.com/developers/docs/resources/entitlement#consume-an-entitlement}
 */
pub type RESTPostAPIEntitlementConsumeResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/subscription#query-string-params}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPISKUSubscriptionsQuery {
    /**
     * List subscriptions before this ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /**
     * List subscriptions after this ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /**
     * Number of subscriptions to return (1-100)
     *
     * @defaultValue `50`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /**
     * User ID for which to return subscriptions. Required except for OAuth queries.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/subscription#list-sku-subscriptions}
 */
pub type RESTGetAPISKUSubscriptionsResult = Vec<APISubscription>;

/**
 * @see {@link https://discord.com/developers/docs/resources/subscription#get-sku-subscription}
 */
pub type RESTGetAPISKUSubscriptionResult = APISubscription;
