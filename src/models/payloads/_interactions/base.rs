use crate::models::payloads::APIRole;
use crate::models::payloads::application_commands::{
    ApplicationIntegrationType, InteractionContextType,
};
use crate::models::payloads::channel::{APIAttachment, APIMessage, ChannelType};
use crate::models::payloads::guild::{
    APIBaseGuildMember, APIFlaggedGuildMember, APIGuildMember, APIGuildMemberAvatar,
    APIGuildMemberJoined, APIPartialInteractionGuild,
};
use crate::models::payloads::monetization::APIEntitlement;
use crate::models::payloads::responses::InteractionType;
use crate::models::payloads::user::APIUser;
use crate::models::rest::Locale;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-interaction-metadata-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIMessageInteractionMetadata {
    APIApplicationCommandInteractionMetadata(APIApplicationCommandInteractionMetadata),
    APIMessageComponentInteractionMetadata(APIMessageComponentInteractionMetadata),
    APIModalSubmitInteractionMetadata(APIModalSubmitInteractionMetadata),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBaseInteractionMetadata {
    /**
     * ID of the interaction
     */
    pub id: String,
    /**
     * Type of interaction
     */
    pub r#type: InteractionType,
    /**
     * User who triggered the interaction
     */
    pub user: APIUser,
    /**
     * IDs for installation context(s) related to an interaction
     */
    pub authorizing_integration_owners: APIAuthorizingIntegrationOwnersMap,
    /**
     * ID of the original response message, present only on follow-up messages
     */
    pub original_response_message_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/message#message-interaction-metadata-object-application-command-interaction-metadata-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIApplicationCommandInteractionMetadata {
    #[serde(flatten)]
    pub base: APIBaseInteractionMetadata,
    /**
     * The user the command was run on, present only on user commands interactions
     */
    pub target_user: Option<APIUser>,
    /**
     * The ID of the message the command was run on, present only on message command interactions.
     * The original response message will also have `message_reference` and `referenced_message` pointing to this message.
     */
    pub target_message_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/message#message-command-interaction-metadata-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageComponentInteractionMetadata {
    #[serde(flatten)]
    pub base: APIBaseInteractionMetadata,
    /**
     * ID of the message that contained the interactive component
     */
    pub interacted_message_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/message#message-interaction-metadata-object-modal-submit-interaction-metadata-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIModalSubmitInteractionMetadata {
    #[serde(flatten)]
    pub base: APIBaseInteractionMetadata,
    /**
     * Metadata for the interaction that was used to open the modal
     */
    pub triggering_interaction_metadata:
        Box<APIApplicationCommandInteractionMetadataOrMessageComponentInteractionMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIApplicationCommandInteractionMetadataOrMessageComponentInteractionMetadata {
    APIApplicationCommandInteractionMetadata(APIApplicationCommandInteractionMetadata),
    APIMessageComponentInteractionMetadata(APIMessageComponentInteractionMetadata),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartialAPIMessageInteractionGuildMember {
    pub avatar: Option<String>,
    pub communication_disabled_until: Option<String>,
    pub deaf: Option<bool>,
    pub joined_at: Option<String>,
    pub mute: Option<bool>,
    pub nick: Option<String>,
    pub pending: Option<bool>,
    pub premium_since: Option<String>,
    pub roles: Vec<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#message-interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageInteraction {
    /**
     * ID of the interaction
     */
    pub id: String,
    /**
     * The type of interaction
     */
    pub r#type: InteractionType,
    /**
     * The name of the application command, including subcommands and subcommand groups
     */
    pub name: String,
    /**
     * The user who invoked the interaction
     */
    pub user: APIUser,
    /**
     * The guild member who invoked the interaction, only sent in MESSAGE_CREATE events
     */
    pub member: Option<PartialAPIMessageInteractionGuildMember>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionGuildMember {
    #[serde(flatten)]
    pub base: APIGuildMember,
    pub permissions: String,
    pub user: APIUser,
}

// INTERACTIONS RECEIVED

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBaseInteraction<TType, Data> {
    /**
     * ID of the interaction
     */
    pub id: String,
    /**
     * ID of the application this interaction is for
     */
    pub application_id: String,
    /**
     * The type of interaction
     */
    pub r#type: TType,
    /**
     * The command data payload
     */
    pub data: Option<Data>,
    /**
     * Guild that the interaction was sent from
     */
    pub guild: Option<APIPartialInteractionGuild>,
    /**
     * Guild that the interaction was sent from
     */
    pub guild_id: Option<String>,
    /**
     * The channel it was sent from
     */
    pub channel: Option<APIInteractionChannelRef>,
    /**
     * The id of the channel it was sent from
     *
     * @deprecated Use {@link APIBaseInteraction.channel} instead
     */
    pub channel_id: Option<String>,
    /**
     * Guild member data for the invoking user, including permissions
     *
     * **This is only sent when an interaction is invoked in a guild**
     */
    pub member: Option<APIInteractionGuildMember>,
    /**
     * User object for the invoking user, if invoked in a DM
     */
    pub user: Option<APIUser>,
    /**
     * A continuation token for responding to the interaction
     */
    pub token: String,
    /**
     * Read-only property, always `1`
     */
    pub version: u8,
    /**
     * For components, the message they were attached to
     */
    pub message: Option<APIMessage>,
    /**
     * Bitwise set of permissions the app or bot has within the channel the interaction was sent from
     */
    pub app_permissions: String,
    /**
     * The selected language of the invoking user
     */
    pub locale: Locale,
    /**
     * The guild's preferred locale, if invoked in a guild
     */
    pub guild_locale: Option<Locale>,
    /**
     * For monetized apps, any entitlements for the invoking user, representing access to premium SKUs
     */
    pub entitlements: Vec<APIEntitlement>,
    /**
     * Mapping of installation contexts that the interaction was authorized for to related user or guild IDs.
     */
    pub authorizing_integration_owners: APIAuthorizingIntegrationOwnersMap,
    /**
     * Context where the interaction was triggered from
     */
    pub context: Option<InteractionContextType>,
    /**
     * Attachment size limit in bytes
     */
    pub attachment_size_limit: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionChannelRef {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: ChannelType,
}

/**
 * Mapping of installation contexts that the interaction was authorized for to related user or guild IDs.
 */
pub type APIAuthorizingIntegrationOwnersMap = HashMap<ApplicationIntegrationType, String>;

pub type APIDMInteractionWrapper<Original> = Original;

pub type APIGuildInteractionWrapper<Original> = Original;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionDataResolvedChannelBase {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: ChannelType,
    pub permissions: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#channel-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIInteractionDataResolvedChannel {
    NonThread(APIInteractionDataResolvedChannelBase),
    Thread {
        #[serde(flatten)]
        base: APIInteractionDataResolvedChannelBase,
        #[serde(default)]
        parent_id: Option<String>,
        #[serde(default)]
        thread_metadata: Option<serde_json::Value>,
    },
}

/**
 * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionDataResolvedGuildMember {
    #[serde(flatten)]
    pub base: APIBaseGuildMember,
    #[serde(flatten)]
    pub flagged: APIFlaggedGuildMember,
    #[serde(flatten)]
    pub avatar: APIGuildMemberAvatar,
    #[serde(flatten)]
    pub joined: APIGuildMemberJoined,
    pub permissions: String,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-resolved-data-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInteractionDataResolved {
    pub users: Option<HashMap<String, APIUser>>,
    pub roles: Option<HashMap<String, APIRole>>,
    pub members: Option<HashMap<String, APIInteractionDataResolvedGuildMember>>,
    pub channels: Option<HashMap<String, APIInteractionDataResolvedChannel>>,
    pub attachments: Option<HashMap<String, APIAttachment>>,
}

/**
 * @deprecated Renamed to `APIInteractionDataResolved`
 */
pub type APIChatInputApplicationCommandInteractionDataResolved = APIInteractionDataResolved;

/**
 * `users` and optional `members` from APIInteractionDataResolved, for user commands and user selects
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIUserInteractionDataResolved {
    pub members: Option<HashMap<String, APIInteractionDataResolvedGuildMember>>,
    pub users: HashMap<String, APIUser>,
}

/**
 * @deprecated Renamed to `APIUserInteractionDataResolved`
 */
pub type APIUserApplicationCommandInteractionDataResolved = APIUserInteractionDataResolved;
