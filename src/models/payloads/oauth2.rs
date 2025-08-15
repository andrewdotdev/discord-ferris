use serde::{Deserialize, Serialize};

/**
 * Types extracted from https://discord.com/developers/docs/topics/oauth2
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OAuth2Scopes {
    /**
     * For oauth2 bots, this puts the bot in the user's selected guild by default
     */
    #[serde(rename = "bot")]
    Bot,
    /**
     * Allows {@link https://discord.com/developers/docs/resources/user#get-user-connections | `/users/@me/connections`}
     * to return linked third-party accounts
     *
     * @see {@link https://discord.com/developers/docs/resources/user#get-user-connections}
     */
    #[serde(rename = "connections")]
    Connections,
    /**
     * Allows your app to see information about the user's DMs and group DMs - requires Discord approval
     */
    #[serde(rename = "dm_channels.read")]
    DMChannelsRead,
    /**
     * Enables {@link https://discord.com/developers/docs/resources/user#get-current-user | `/users/@me`} to return an `email`
     *
     * @see {@link https://discord.com/developers/docs/resources/user#get-current-user}
     */
    #[serde(rename = "email")]
    Email,
    /**
     * Allows {@link https://discord.com/developers/docs/resources/user#get-current-user | `/users/@me`} without `email`
     *
     * @see {@link https://discord.com/developers/docs/resources/user#get-current-user}
     */
    #[serde(rename = "identify")]
    Identify,
    /**
     * Allows {@link https://discord.com/developers/docs/resources/user#get-current-user-guilds | `/users/@me/guilds`}
     * to return basic information about all of a user's guilds
     *
     * @see {@link https://discord.com/developers/docs/resources/user#get-current-user-guilds}
     */
    #[serde(rename = "guilds")]
    Guilds,
    /**
     * Allows {@link https://discord.com/developers/docs/resources/guild#add-guild-member | `/guilds/[guild.id]/members/[user.id]`}
     * to be used for joining users to a guild
     *
     * @see {@link https://discord.com/developers/docs/resources/guild#add-guild-member}
     */
    #[serde(rename = "guilds.join")]
    GuildsJoin,
    /**
     * Allows /users/\@me/guilds/\{guild.id\}/member to return a user's member information in a guild
     *
     * @see {@link https://discord.com/developers/docs/resources/user#get-current-user-guild-member}
     */
    #[serde(rename = "guilds.members.read")]
    GuildsMembersRead,
    /**
     * Allows your app to join users to a group dm
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#group-dm-add-recipient}
     */
    #[serde(rename = "gdm.join")]
    GroupDMJoins,
    /**
     * For local rpc server api access, this allows you to read messages from all client channels
     * (otherwise restricted to channels/guilds your app creates)
     */
    #[serde(rename = "messages.read")]
    MessagesRead,
    /**
     * Allows your app to update a user's connection and metadata for the app
     */
    #[serde(rename = "role_connections.write")]
    RoleConnectionsWrite,
    /**
     * For local rpc server access, this allows you to control a user's local Discord client - requires Discord approval
     */
    #[serde(rename = "rpc")]
    RPC,
    /**
     * For local rpc server access, this allows you to update a user's activity - requires Discord approval
     */
    #[serde(rename = "rpc.activities.write")]
    RPCActivitiesWrite,
    /**
     * For local rpc server access, this allows you to read a user's voice settings and listen for voice events - requires Discord approval
     */
    #[serde(rename = "rpc.voice.read")]
    RPCVoiceRead,
    /**
     * For local rpc server access, this allows you to update a user's voice settings - requires Discord approval
     */
    #[serde(rename = "rpc.voice.write")]
    RPCVoiceWrite,
    /**
     * For local rpc server api access, this allows you to receive notifications pushed out to the user - requires Discord approval
     */
    #[serde(rename = "rpc.notifications.read")]
    RPCNotificationsRead,
    /**
     * This generates a webhook that is returned in the oauth token response for authorization code grants
     */
    #[serde(rename = "webhook.incoming")]
    WebhookIncoming,
    /**
     * Allows your app to connect to voice on user's behalf and see all the voice members - requires Discord approval
     */
    #[serde(rename = "voice")]
    Voice,
    /**
     * Allows your app to upload/update builds for a user's applications - requires Discord approval
     */
    #[serde(rename = "applications.builds.upload")]
    ApplicationsBuildsUpload,
    /**
     * Allows your app to read build data for a user's applications
     */
    #[serde(rename = "applications.builds.read")]
    ApplicationsBuildsRead,
    /**
     * Allows your app to read and update store data (SKUs, store listings, achievements, etc.) for a user's applications
     */
    #[serde(rename = "applications.store.update")]
    ApplicationsStoreUpdate,
    /**
     * Allows your app to read entitlements for a user's applications
     */
    #[serde(rename = "applications.entitlements")]
    ApplicationsEntitlements,
    /**
     * Allows your app to know a user's friends and implicit relationships - requires Discord approval
     */
    #[serde(rename = "relationships.read")]
    RelationshipsRead,
    /**
     * Allows your app to fetch data from a user's "Now Playing/Recently Played" list - requires Discord approval
     */
    #[serde(rename = "activities.read")]
    ActivitiesRead,
    /**
     * Allows your app to update a user's activity - requires Discord approval (NOT REQUIRED FOR GAMESDK ACTIVITY MANAGER)
     *
     * @see {@link https://discord.com/developers/docs/game-sdk/activities}
     */
    #[serde(rename = "activities.write")]
    ActivitiesWrite,
    /**
     * Allows your app to use Application Commands in a guild
     *
     * @see {@link https://discord.com/developers/docs/interactions/application-commands}
     */
    #[serde(rename = "applications.commands")]
    ApplicationsCommands,
    /**
     * Allows your app to update its Application Commands via this bearer token - client credentials grant only
     *
     * @see {@link https://discord.com/developers/docs/interactions/application-commands}
     */
    #[serde(rename = "applications.commands.update")]
    ApplicationsCommandsUpdate,
    /**
     * Allows your app to update permissions for its commands using a Bearer token - client credentials grant only
     *
     * @see {@link https://discord.com/developers/docs/interactions/application-commands}
     */
    #[serde(rename = "applications.commands.permissions.update")]
    ApplicationCommandsPermissionsUpdate,
}
