use crate::utils::serde::flags_numeric;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::payloads::application_commands::ApplicationIntegrationType;
use crate::models::payloads::guild::APIPartialGuild;
use crate::models::payloads::oauth2::OAuth2Scopes;
use crate::models::payloads::teams::APITeam;
use crate::models::payloads::user::APIUser;
use crate::models::payloads::{ApplicationWebhookEventType, LocalizationMap};

/**
 * Types extracted from https://discord.com/developers/docs/resources/application
 */

/**
 * @see {@link https://discord.com/developers/docs/resources/application#application-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplication {
    /**
     * The id of the app
     */
    pub id: String,
    /**
     * The name of the app
     */
    pub name: String,
    /**
     * The icon hash of the app
     */
    pub icon: Option<String>,
    /**
     * The description of the app
     */
    pub description: String,
    /**
     * An array of rpc origin urls, if rpc is enabled
     */
    pub rpc_origins: Option<Vec<String>>,
    /**
     * When `false` only app owner can join the app's bot to guilds
     */
    pub bot_public: bool,
    /**
     * When `true` the app's bot will only join upon completion of the full oauth2 code grant flow
     */
    pub bot_require_code_grant: bool,
    /**
     * Partial user object for the bot user associated with the application
     */
    pub bot: Option<APIUser>,
    /**
     * The url of the application's terms of service
     */
    pub terms_of_service_url: Option<String>,
    /**
     * The url of the application's privacy policy
     */
    pub privacy_policy_url: Option<String>,
    /**
     * Partial user object containing info on the owner of the application
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub owner: Option<APIUser>,
    /**
     * An empty string
     *
     * @deprecated This field will be removed in v11
     * @unstable This field is no longer documented by Discord and will be removed in v11
     */
    pub summary: String,
    /**
     * The hexadecimal encoded key for verification in interactions and the GameSDK's GetTicket function
     *
     * @see {@link https://discord.com/developers/docs/game-sdk/applications#getticket}
     */
    pub verify_key: String,
    /**
     * The team this application belongs to
     *
     * @see {@link https://discord.com/developers/docs/topics/teams#data-models-team-object}
     */
    pub team: Option<APITeam>,
    /**
     * If this application is a game sold on Discord, this field will be the guild to which it has been linked
     */
    pub guild_id: Option<String>,
    /**
     * A partial object of the associated guild
     */
    pub guild: Option<APIPartialGuild>,
    /**
     * If this application is a game sold on Discord, this field will be the id of the "Game SKU" that is created, if exists
     */
    pub primary_sku_id: Option<String>,
    /**
     * If this application is a game sold on Discord, this field will be the URL slug that links to the store page
     */
    pub slug: Option<String>,
    /**
     * If this application is a game sold on Discord, this field will be the hash of the image on store embeds
     */
    pub cover_image: Option<String>,
    /**
     * The application's public flags
     *
     * @see {@link https://discord.com/developers/docs/resources/application#application-object-application-flags}
     */
    #[serde(with = "flags_numeric")]
    pub flags: ApplicationFlags,
    /**
     * Approximate count of guilds the application has been added to
     */
    pub approximate_guild_count: Option<i64>,
    /**
     * Approximate count of users that have installed the app (authorized with `application.commands` as a scope)
     */
    pub approximate_user_install_count: Option<i64>,
    /**
     * Approximate count of users that have OAuth2 authorizations for the app
     */
    pub approximate_user_authorization_count: Option<i64>,
    /**
     * Array of redirect URIs for the application
     */
    pub redirect_uris: Option<Vec<String>>,
    /**
     * The interactions endpoint URL for the application
     */
    pub interactions_endpoint_url: Option<String>,
    /**
     * The application's role connection verification entry point,
     * which when configured will render the app as a verification method in the guild role verification configuration
     */
    pub role_connections_verification_url: Option<String>,
    /**
     * Up to 5 tags of max 20 characters each describing the content and functionality of the application
     */
    pub tags: Option<Vec<String>>,
    /**
     * Settings for the application's default in-app authorization link, if enabled
     */
    pub install_params: Option<APIApplicationInstallParams>,
    /**
     * Default scopes and permissions for each supported installation context. Value for each key is an integration type configuration object
     */
    pub integration_types_config: Option<APIApplicationIntegrationTypesConfigMap>,
    /**
     * The application's default custom authorization link, if enabled
     */
    pub custom_install_url: Option<String>,
    /**
     * Event webhook URL for the app to receive webhook events
     */
    pub event_webhooks_url: Option<String>,
    /**
     * If webhook events are enabled for the app
     */
    pub event_webhooks_status: ApplicationWebhookEventStatus,
    /**
     * List of webhook event types the app subscribes to
     */
    pub event_webhooks_types: Option<Vec<ApplicationWebhookEventType>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationInstallParams {
    pub scopes: Vec<OAuth2Scopes>,
    pub permissions: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationIntegrationTypeConfiguration {
    pub oauth2_install_params: Option<APIApplicationInstallParams>,
}

pub type APIApplicationIntegrationTypesConfigMap = std::collections::HashMap<
    ApplicationIntegrationType,
    APIApplicationIntegrationTypeConfiguration,
>;

bitflags! {
    /**
    * @see {@link https://discord.com/developers/docs/resources/application#application-object-application-flags}
    */
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ApplicationFlags: u32 {
        /**
         * @unstable This application flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const EmbeddedReleased = 1 << 1;
        /**
         * @unstable This application flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const ManagedEmoji = 1 << 2;
        /**
         * @unstable This application flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const EmbeddedIAP = 1 << 3;
        /**
         * @unstable This application flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const GroupDMCreate = 1 << 4;
        /**
         * Indicates if an app uses the Auto Moderation API
         */
        const ApplicationAutoModerationRuleCreateBadge = 1 << 6;
        /**
         * @unstable This application flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const RPCHasConnected = 1 << 11;
        /**
         * Intent required for bots in 100 or more servers to receive `presence_update` events
         */
        const GatewayPresence = 1 << 12;
        /**
         * Intent required for bots in under 100 servers to receive `presence_update` events, found in Bot Settings
         */
        const GatewayPresenceLimited = 1 << 13;
        /**
         * Intent required for bots in 100 or more servers to receive member-related events like `guild_member_add`.
         *
         * @see List of member-related events {@link https://discord.com/developers/docs/topics/gateway#list-of-intents | under `GUILD_MEMBERS`}
         */
        const GatewayGuildMembers = 1 << 14;
        /**
         * Intent required for bots in under 100 servers to receive member-related events like `guild_member_add`, found in Bot Settings.
         *
         * @see List of member-related events {@link https://discord.com/developers/docs/topics/gateway#list-of-intents | under `GUILD_MEMBERS`}
         */
        const GatewayGuildMembersLimited = 1 << 15;
        /**
         * Indicates unusual growth of an app that prevents verification
         */
        const VerificationPendingGuildLimit = 1 << 16;
        /**
         * Indicates if an app is embedded within the Discord client (currently unavailable publicly)
         */
        const Embedded = 1 << 17;
        /**
         * Intent required for bots in 100 or more servers to receive {@link https://support-dev.discord.com/hc/articles/6207308062871 | message content}
         */
        const GatewayMessageContent = 1 << 18;
        /**
         * Intent required for bots in under 100 servers to receive {@link https://support-dev.discord.com/hc/articles/6207308062871 | message content},
         * found in Bot Settings
         */
        const GatewayMessageContentLimited = 1 << 19;
        /**
         * @unstable This application flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const EmbeddedFirstParty = 1 << 20;
        /**
         * Indicates if an app has registered global {@link https://discord.com/developers/docs/interactions/application-commands | application commands}
         */
        const ApplicationCommandBadge = 1 << 23;
    }
}

/**
 * @see {@link https://discord.com/developers/docs/resources/application-role-connection-metadata#application-role-connection-metadata-object-application-role-connection-metadata-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationRoleConnectionMetadata {
    /**
     * Type of metadata value
     */
    #[serde(rename = "type")]
    pub r#type: ApplicationRoleConnectionMetadataType,
    /**
     * Dictionary key for the metadata field (must be `a-z`, `0-9`, or `_` characters; 1-50 characters)
     */
    pub key: String,
    /**
     * Name of the metadata field (1-100 characters)
     */
    pub name: String,
    /**
     * Translations of the name
     */
    pub name_localizations: Option<LocalizationMap>,
    /**
     * Description of the metadata field (1-200 characters)
     */
    pub description: String,
    /**
     * Translations of the description
     */
    pub description_localizations: Option<LocalizationMap>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/application-role-connection-metadata#application-role-connection-metadata-object-application-role-connection-metadata-type}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationRoleConnectionMetadataType {
    /**
     * The metadata value (`integer`) is less than or equal to the guild's configured value (`integer`)
     */
    IntegerLessThanOrEqual = 1,
    /**
     * The metadata value (`integer`) is greater than or equal to the guild's configured value (`integer`)
     */
    IntegerGreaterThanOrEqual = 2,
    /**
     * The metadata value (`integer`) is equal to the guild's configured value (`integer`)
     */
    IntegerEqual = 3,
    /**
     * The metadata value (`integer`) is not equal to the guild's configured value (`integer`)
     */
    IntegerNotEqual = 4,
    /**
     * The metadata value (`ISO8601 string`) is less than or equal to the guild's configured value (`integer`; days before current date)
     */
    DatetimeLessThanOrEqual = 5,
    /**
     * The metadata value (`ISO8601 string`) is greater than or equal to the guild's configured value (`integer`; days before current date)
     */
    DatetimeGreaterThanOrEqual = 6,
    /**
     * The metadata value (`integer`) is equal to the guild's configured value (`integer`; `1`)
     */
    BooleanEqual = 7,
    /**
     * The metadata value (`integer`) is not equal to the guild's configured value (`integer`; `1`)
     */
    BooleanNotEqual = 8,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/application#application-object-application-event-webhook-status}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationWebhookEventStatus {
    /**
     * Webhook events are disabled by developer
     */
    Disabled = 1,
    /**
     * Webhook events are enabled by developer
     */
    Enabled = 2,
    /**
     * Webhook events are disabled by Discord, usually due to inactivity
     */
    DisabledByDiscord = 3,
}
