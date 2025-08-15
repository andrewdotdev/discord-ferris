use serde::{Deserialize, Serialize};

use super::guild::APIGuildIntegration;

/**
 * Types extracted from https://discord.com/developers/docs/resources/user
 */

/**
 * @see {@link https://discord.com/developers/docs/resources/user#user-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIUser {
    /**
     * The user's id
     */
    pub id: String,
    /**
     * The user's username, not unique across the platform
     */
    pub username: String,
    /**
     * The user's Discord-tag
     */
    pub discriminator: String,
    /**
     * The user's display name, if it is set. For bots, this is the application name
     */
    pub global_name: Option<String>,
    /**
     * The user's avatar hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub avatar: Option<String>,
    /**
     * Whether the user belongs to an OAuth2 application
     */
    pub bot: Option<bool>,
    /**
     * Whether the user is an Official Discord System user (part of the urgent message system)
     */
    pub system: Option<bool>,
    /**
     * Whether the user has two factor enabled on their account
     */
    pub mfa_enabled: Option<bool>,
    /**
     * The user's banner hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub banner: Option<String>,
    /**
     * The user's banner color encoded as an integer representation of hexadecimal color code
     */
    pub accent_color: Option<u64>,
    /**
     * The user's chosen language option
     */
    pub locale: Option<String>,
    /**
     * Whether the email on this account has been verified
     */
    pub verified: Option<bool>,
    /**
     * The user's email
     */
    pub email: Option<String>,
    /**
     * The flags on a user's account
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object-user-flags}
     */
    pub flags: Option<u64>,
    /**
     * The type of Nitro subscription on a user's account
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object-premium-types}
     */
    pub premium_type: Option<UserPremiumType>,
    /**
     * The public flags on a user's account
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object-user-flags}
     */
    pub public_flags: Option<u64>,
    /**
     * The user's avatar decoration hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     * @deprecated Use {@link APIUser.avatar_decoration_data} instead
     */
    pub avatar_decoration: Option<String>,
    /**
     * The data for the user's avatar decoration
     *
     * @see {@link https://discord.com/developers/docs/resources/user#avatar-decoration-data-object}
     */
    pub avatar_decoration_data: Option<APIAvatarDecorationData>,
    /**
     * The data for the user's collectibles
     *
     * @see {@link https://discord.com/developers/docs/resources/user#collectibles}
     */
    pub collectibles: Option<APICollectibles>,
    /**
     * The user's primary guild
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object-user-primary-guild}
     */
    pub primary_guild: Option<APIUserPrimaryGuild>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#user-object-user-flags}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u64)]
pub enum UserFlags {
    /**
     * Discord Employee
     */
    Staff = 1 << 0,
    /**
     * Partnered Server Owner
     */
    Partner = 1 << 1,
    /**
     * HypeSquad Events Member
     */
    Hypesquad = 1 << 2,
    /**
     * Bug Hunter Level 1
     */
    BugHunterLevel1 = 1 << 3,
    /**
     * @unstable This user flag is currently not documented by Discord but has a known value which we will try to keep up to date.
     */
    MFASMS = 1 << 4,
    /**
     * @unstable This user flag is currently not documented by Discord but has a known value which we will try to keep up to date.
     */
    PremiumPromoDismissed = 1 << 5,
    /**
     * House Bravery Member
     */
    HypeSquadOnlineHouse1 = 1 << 6,
    /**
     * House Brilliance Member
     */
    HypeSquadOnlineHouse2 = 1 << 7,
    /**
     * House Balance Member
     */
    HypeSquadOnlineHouse3 = 1 << 8,
    /**
     * Early Nitro Supporter
     */
    PremiumEarlySupporter = 1 << 9,
    /**
     * User is a {@link https://discord.com/developers/docs/topics/teams | team}
     */
    TeamPseudoUser = 1 << 10,
    /**
     * @unstable This user flag is currently not documented by Discord but has a known value which we will try to keep up to date.
     */
    HasUnreadUrgentMessages = 1 << 13,
    /**
     * Bug Hunter Level 2
     */
    BugHunterLevel2 = 1 << 14,
    /**
     * Verified Bot
     */
    VerifiedBot = 1 << 16,
    /**
     * Early Verified Bot Developer
     */
    VerifiedDeveloper = 1 << 17,
    /**
     * Moderator Programs Alumni
     */
    CertifiedModerator = 1 << 18,
    /**
     * Bot uses only {@link https://discord.com/developers/docs/interactions/receiving-and-responding#receiving-an-interaction | HTTP interactions} and is shown in the online member list
     */
    BotHTTPInteractions = 1 << 19,
    /**
     * User has been identified as spammer
     *
     * @unstable This user flag is currently not documented by Discord but has a known value which we will try to keep up to date.
     */
    Spammer = 1 << 20,
    /**
     * @unstable This user flag is currently not documented by Discord but has a known value which we will try to keep up to date.
     */
    DisablePremium = 1 << 21,
    /**
     * User is an {@link https://support-dev.discord.com/hc/articles/10113997751447 | Active Developer}
     */
    ActiveDeveloper = 1 << 22,
    /**
     * User's account has been {@link https://support.discord.com/hc/articles/6461420677527 | quarantined} based on recent activity
     *
     * @unstable This user flag is currently not documented by Discord but has a known value which we will try to keep up to date.
     * @privateRemarks
     *
     * This value would be `1 << 44`, but bit shifting above `1 << 30` requires bigints
     */
    Quarantined = 17_592_186_044_416,
    /**
     * @unstable This user flag is currently not documented by Discord but has a known value which we will try to keep up to date.
     * @privateRemarks
     *
     * This value would be `1 << 50`, but bit shifting above `1 << 30` requires bigints
     */
    Collaborator = 1_125_899_906_842_624,
    /**
     * @unstable This user flag is currently not documented by Discord but has a known value which we will try to keep up to date.
     * @privateRemarks
     *
     * This value would be `1 << 51`, but bit shifting above `1 << 30` requires bigints
     */
    RestrictedCollaborator = 2_251_799_813_685_248,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#user-object-premium-types}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum UserPremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
    NitroBasic = 3,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#connection-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIConnection {
    /**
     * ID of the connection account
     */
    pub id: String,
    /**
     * The username of the connection account
     */
    pub name: String,
    /**
     * The service of the connection
     *
     * @see {@link https://discord.com/developers/docs/resources/user#connection-object-services}
     */
    pub r#type: ConnectionService,
    /**
     * Whether the connection is revoked
     */
    pub revoked: Option<bool>,
    /**
     * An array of partial server integrations
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#integration-object}
     */
    pub integrations: Option<Vec<APIGuildIntegration>>,
    /**
     * Whether the connection is verified
     */
    pub verified: bool,
    /**
     * Whether friend sync is enabled for this connection
     */
    pub friend_sync: bool,
    /**
     * Whether activities related to this connection will be shown in presence updates
     */
    pub show_activity: bool,
    /**
     * Whether this connection supports console voice transfer
     */
    pub two_way_link: bool,
    /**
     * Visibility of this connection
     *
     * @see {@link https://discord.com/developers/docs/resources/user#connection-object-visibility-types}
     */
    pub visibility: ConnectionVisibility,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionService {
    #[serde(rename = "amazon-music")]
    AmazonMusic,
    #[serde(rename = "battlenet")]
    BattleNet,
    #[serde(rename = "bluesky")]
    Bluesky,
    #[serde(rename = "bungie")]
    BungieNet,
    #[serde(rename = "crunchyroll")]
    Crunchyroll,
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "ebay")]
    EBay,
    #[serde(rename = "epicgames")]
    EpicGames,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "leagueoflegends")]
    LeagueOfLegends,
    #[serde(rename = "mastodon")]
    Mastodon,
    #[serde(rename = "paypal")]
    PayPal,
    #[serde(rename = "playstation")]
    PlayStationNetwork,
    #[serde(rename = "reddit")]
    Reddit,
    #[serde(rename = "riotgames")]
    RiotGames,
    #[serde(rename = "roblox")]
    Roblox,
    #[serde(rename = "spotify")]
    Spotify,
    #[serde(rename = "skype")]
    Skype,
    #[serde(rename = "steam")]
    Steam,
    #[serde(rename = "tiktok")]
    TikTok,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "twitter")]
    X,
    #[serde(rename = "xbox")]
    Xbox,
    #[serde(rename = "youtube")]
    YouTube,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ConnectionVisibility {
    /**
     * Invisible to everyone except the user themselves
     */
    None = 0,
    /**
     * Visible to everyone
     */
    Everyone = 1,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#application-role-connection-object-application-role-connection-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationRoleConnection {
    /**
     * The vanity name of the platform a bot has connected (max 50 characters)
     */
    pub platform_name: Option<String>,
    /**
     * The username on the platform a bot has connected (max 100 characters)
     */
    pub platform_username: Option<String>,
    /**
     * Object mapping application role connection metadata keys to their `string`-ified value (max 100 characters) for the user on the platform a bot has connected
     */
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#avatar-decoration-data-object}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIAvatarDecorationData {
    /**
     * The avatar decoration hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub asset: String,
    /**
     * The id of the avatar decoration's SKU
     */
    pub sku_id: String,
}

/**
 * The collectibles the user has, excluding Avatar Decorations and Profile Effects.
 *
 * @see {@link https://discord.com/developers/docs/resources/user#collectibles}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APICollectibles {
    /**
     * Object mapping of {@link APINameplateData}
     */
    pub nameplate: Option<APINameplateData>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#nameplate}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APINameplateData {
    /**
     * ID of the nameplate SKU
     */
    pub sku_id: String,
    /**
     * Path to the nameplate asset
     *
     * @example `nameplates/nameplates/twilight/`
     */
    pub asset: String,
    /**
     * The label of this nameplate. Currently unused
     */
    pub label: String,
    /**
     * Background color of the nameplate
     */
    pub palette: NameplatePalette,
}

/**
 * Background color of a nameplate.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NameplatePalette {
    #[serde(rename = "berry")]
    Berry,
    #[serde(rename = "bubble_gum")]
    BubbleGum,
    #[serde(rename = "clover")]
    Clover,
    #[serde(rename = "cobalt")]
    Cobalt,
    #[serde(rename = "crimson")]
    Crimson,
    #[serde(rename = "forest")]
    Forest,
    #[serde(rename = "lemon")]
    Lemon,
    #[serde(rename = "sky")]
    Sky,
    #[serde(rename = "teal")]
    Teal,
    #[serde(rename = "violet")]
    Violet,
    #[serde(rename = "white")]
    White,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/user#user-object-user-primary-guild}
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIUserPrimaryGuild {
    /**
     * The id of the user's primary guild
     */
    pub identity_guild_id: Option<String>,
    /**
     * Whether the user is displaying the primary guild's server tag.
     * This can be `null` if the system clears the identity, e.g. because the server no longer supports tags
     */
    pub identity_enabled: Option<bool>,
    /**
     * The text of the user's server tag. Limited to 4 characters
     */
    pub tag: Option<String>,
    /**
     * The server tag badge hash
     *
     * @see {@link https://discord.com/developers/docs/reference#image-formatting}
     */
    pub badge: Option<String>,
}
