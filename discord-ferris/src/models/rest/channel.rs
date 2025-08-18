// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use super::poll::RESTAPIPoll;
use crate::models::payloads::{
    APIAllowedMentions, APIChannel, APIEmbed, APIExtendedInvite, APIFollowedChannel,
    APIGuildForumDefaultReactionEmoji, APIGuildForumTag, APIMessage, APIMessagePin,
    APIMessageTopLevelComponent, APIThreadList, APIThreadMember, APIUser, ChannelFlags,
    ChannelType, ForumLayoutType, InviteTargetType, MessageFlags, OverwriteType, SortOrderType,
    ThreadAutoArchiveDuration, ThreadChannelType, VideoQualityMode,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A helper for fields that accept either a `String` or an ISO8601 string timestamp.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SnowflakeOrIso8601 {
    String(String),
    Iso8601(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AttachmentId {
    String(String),
    Index(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIAttachment {
    /**
     * Attachment id or a number that matches `n` in `files[n]`
     */
    pub id: AttachmentId,
    // The following fields correspond to: Pick<APIAttachment, 'description' | 'duration_secs' | 'filename' | 'title' | 'waveform'>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPutAPIChannelPermissionJSONBody {
    /**
     * The bitwise value of all allowed permissions
     *
     * @see {@link https://en.wikipedia.org/wiki/Bit_field}
     * @defaultValue `"0"`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Option<String>>,
    /**
     * The bitwise value of all disallowed permissions
     *
     * @see {@link https://en.wikipedia.org/wiki/Bit_field}
     * @defaultValue `"0"`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Option<String>>,
    /**
     * `0` for a role or `1` for a member
     */
    pub r#type: OverwriteType,
}

pub type RESTPutAPIChannelPermissionResult = ();

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTAPIChannelPatchOverwrite {
    pub id: String,
    #[serde(flatten)]
    pub perms: RESTPutAPIChannelPermissionJSONBody,
}

/**
 * @deprecated Use {@link RESTAPIChannelPatchOverwrite} instead
 */
#[deprecated(note = "Use RESTAPIChannelPatchOverwrite instead")]
pub type APIChannelPatchOverwrite = RESTAPIChannelPatchOverwrite;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#get-channel}
 */
pub type RESTGetAPIChannelResult = APIChannel;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#modify-channel}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIChannelJSONBody {
    /**
     * 1-100 character channel name
     *
     * Channel types: all
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /**
     * The type of channel; only conversion between `text` and `news`
     * is supported and only in guilds with the "NEWS" feature
     *
     * Channel types: text, news
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ChannelType>,

    /**
     * The position of the channel in the left-hand listing
     *
     * Channel types: all excluding newsThread, publicThread, privateThread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Option<i32>>,

    /**
     * 0-1024 character channel topic (0-4096 characters for thread-only channels)
     *
     * Channel types: text, news, forum, media
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<Option<String>>,

    /**
     * Whether the channel is nsfw
     *
     * Channel types: text, voice, news, forum, media
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<Option<bool>>,

    /**
     * Amount of seconds a user has to wait before sending another message (0-21600);
     * bots, as well as users with the permission `MANAGE_MESSAGES` or `MANAGE_CHANNELS`,
     * are unaffected
     *
     * Channel types: text, newsThread, publicThread, privateThread, forum, media
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<Option<u64>>,

    /**
     * The bitrate (in bits) of the voice channel; 8000 to 96000 (128000 for VIP servers)
     *
     * Channel types: voice
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<Option<u32>>,

    /**
     * The user limit of the voice channel; 0 refers to no limit, 1 to 99 refers to a user limit
     *
     * Channel types: voice
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<Option<u16>>,

    /**
     * Channel or category-specific permissions
     *
     * Channel types: all excluding newsThread, publicThread, privateThread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Option<Vec<RESTAPIChannelPatchOverwrite>>>,

    /**
     * ID of the new parent category for a channel
     *
     * Channel types: text, voice, news, stage, forum, media
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<String>>,

    /**
     * Voice region id for the voice or stage channel, automatic when set to `null`
     *
     * @see {@link https://discord.com/developers/docs/resources/voice#voice-region-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<Option<String>>,

    /**
     * The camera video quality mode of the voice channel
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object-video-quality-modes}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<Option<VideoQualityMode>>,

    /**
     * Whether the thread should be archived
     *
     * Channel types: newsThread, publicThread, privateThread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    /**
     * The amount of time in minutes to wait before automatically archiving the thread
     *
     * Channel types: newsThread, publicThread, privateThread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,

    /**
     * Whether the thread should be locked
     *
     * Channel types: newsThread, publicThread, privateThread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    /**
     * Default duration for newly created threads, in minutes, to automatically archive the thread after recent activity
     *
     * Channel types: text, news, forum, media
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<ThreadAutoArchiveDuration>,

    /**
     * Channel flags combined as a bit field.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,

    /**
     * The set of tags that can be used in a thread-only channel; limited to 20
     *
     * Channel types: forum, media
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Vec<APIGuildForumTag>>,

    /**
     * Whether non-moderators can add other non-moderators to the thread
     *
     * Channel types: privateThread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,

    /**
     * The emoji to show in the add reaction button on a thread in a thread-only channel
     *
     * Channel types: forum, media
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<APIGuildForumDefaultReactionEmoji>,

    /**
     * The initial `rate_limit_per_user` to set on newly created threads in a channel.
     * This field is copied to the thread at creation time and does not live update
     *
     * Channel types: text, forum, media
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<Option<u64>>,

    /**
     * The default sort order type used to order posts in a thread-only channel
     *
     * Channel types: forum, media
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<Option<SortOrderType>>,

    /**
     * The default layout type used to display posts in a forum channel
     *
     * Channel types: forum
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayoutType>,

    /**
     * The ids of the set of tags that have been applied to a thread-only channel; limited to 5
     *
     * Channel types: forum, media
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#modify-channel}
 */
pub type RESTPatchAPIChannelResult = APIChannel;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#deleteclose-channel}
 */
pub type RESTDeleteAPIChannelResult = APIChannel;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#get-channel-messages}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct RESTGetAPIChannelMessagesQuery {
    /**
     * Get messages around this message ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub around: Option<String>,
    /**
     * Get messages before this message ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /**
     * Get messages after this message ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /**
     * Max number of messages to return (1-100)
     *
     * @defaultValue `50`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#get-channel-messages}
 */
pub type RESTGetAPIChannelMessagesResult = Vec<APIMessage>;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#get-channel-message}
 */
pub type RESTGetAPIChannelMessageResult = APIMessage;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-reference-object-message-reference-structure}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct RESTAPIMessageReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    pub message_id: String,
    /**
     * Whether to error if the referenced message doesn't exist instead of sending as a normal (non-reply) message
     *
     * @defaultValue `true`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_if_not_exists: Option<bool>,
}

/**
 * @deprecated Use {@link RESTAPIMessageReference} instead
 */
#[deprecated(note = "Use RESTAPIMessageReference instead")]
pub type APIMessageReferenceSend = RESTAPIMessageReference;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#create-message}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIChannelMessageJSONBody {
    /**
     * The message contents (up to 2000 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /**
     * A nonce that can be used for optimistic message sending
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Value>,
    /**
     * `true` if this is a TTS message
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    /**
     * Embedded `rich` content (up to 6000 characters)
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<APIEmbed>>,
    /**
     * Allowed mentions for a message
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#allowed-mentions-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<APIAllowedMentions>,
    /**
     * Include to make your message a reply or a forward
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#message-reference-object-message-reference-structure}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<RESTAPIMessageReference>,
    /**
     * The components to include with the message
     *
     * @see {@link https://discord.com/developers/docs/components/reference}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<APIMessageTopLevelComponent>>,
    /**
     * IDs of up to 3 stickers in the server to send in the message
     *
     * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_ids: Option<Vec<String>>,
    /**
     * Attachment objects with filename and description
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<RESTAPIAttachment>>,
    /**
     * Message flags combined as a bitfield
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    /**
     * If `true` and nonce is present, it will be checked for uniqueness in the past few minutes.
     * If another message was created by the same author with the same nonce, that message will be returned and no new message will be created.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_nonce: Option<bool>,
    /**
     * A poll!
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<RESTAPIPoll>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#create-message}
 */
pub type RESTPostAPIChannelMessageFormDataBody = Value;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#create-message}
 */
pub type RESTPostAPIChannelMessageResult = APIMessage;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#crosspost-message}
 */
pub type RESTPostAPIChannelMessageCrosspostResult = APIMessage;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#create-reaction}
 */
pub type RESTPutAPIChannelMessageReactionResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#delete-own-reaction}
 */
pub type RESTDeleteAPIChannelMessageOwnReactionResult = ();

/**
 * @deprecated Use {@link RESTDeleteAPIChannelMessageOwnReactionResult} instead
 */
#[deprecated(note = "Use RESTDeleteAPIChannelMessageOwnReactionResult instead")]
pub type RESTDeleteAPIChannelMessageOwnReaction = RESTDeleteAPIChannelMessageOwnReactionResult;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#delete-user-reaction}
 */
pub type RESTDeleteAPIChannelMessageUserReactionResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#get-reactions}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct RESTGetAPIChannelMessageReactionUsersQuery {
    /**
     * The reaction type
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ReactionType>,
    /**
     * Get users after this user ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /**
     * Max number of users to return (1-100)
     *
     * @defaultValue `25`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#get-reactions-reaction-types}
 */
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ReactionType {
    Normal = 0,
    Super = 1,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#get-reactions}
 */
pub type RESTGetAPIChannelMessageReactionUsersResult = Vec<APIUser>;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#delete-all-reactions}
 */
pub type RESTDeleteAPIChannelAllMessageReactionsResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#delete-all-reactions-for-emoji}
 */
pub type RESTDeleteAPIChannelMessageReactionResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#edit-message}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIChannelMessageJSONBody {
    /**
     * The new message contents (up to 2000 characters)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
    /**
     * Embedded `rich` content (up to 6000 characters)
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Option<Vec<APIEmbed>>>,
    /**
     * Edit the flags of a message (only `SUPPRESS_EMBEDS` can currently be set/unset)
     *
     * When specifying flags, ensure to include all previously set flags/bits
     * in addition to ones that you are modifying
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-flags}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Option<MessageFlags>>,
    /**
     * Allowed mentions for the message
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#allowed-mentions-object}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<Option<APIAllowedMentions>>,
    /**
     * Attached files to keep
     *
     * Starting with API v10, the `attachments` array must contain all attachments that should be present after edit, including **retained and new** attachments provided in the request body.
     *
     * @see {@link https://discord.com/developers/docs/resources/message#attachment-object-attachment-structure}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<RESTAPIAttachment>>,
    /**
     * The components to include with the message
     *
     * @see {@link https://discord.com/developers/docs/components/reference}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Option<Vec<APIMessageTopLevelComponent>>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#edit-message}
 */
pub type RESTPatchAPIChannelMessageFormDataBody = Value;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#edit-message}
 */
pub type RESTPatchAPIChannelMessageResult = APIMessage;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#delete-message}
 */
pub type RESTDeleteAPIChannelMessageResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#bulk-delete-messages}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RESTPostAPIChannelMessagesBulkDeleteJSONBody {
    /**
     * An array of message ids to delete (2-100)
     */
    pub messages: Vec<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#bulk-delete-messages}
 */
pub type RESTPostAPIChannelMessagesBulkDeleteResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#get-channel-invites}
 */
pub type RESTGetAPIChannelInvitesResult = Vec<APIExtendedInvite>;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#create-channel-invite}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIChannelInviteJSONBody {
    /**
     * Duration of invite in seconds before expiry, or 0 for never
     *
     * @defaultValue `86400` (24 hours)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u64>,
    /**
     * Max number of uses or 0 for unlimited
     *
     * @defaultValue `0`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u32>,
    /**
     * Whether this invite only grants temporary membership
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    /**
     * If true, don't try to reuse a similar invite
     * (useful for creating many unique one time use invites)
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    /**
     * The type of target for this voice channel invite
     *
     * @see {@link https://discord.com/developers/docs/resources/invite#invite-object-invite-target-types}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<InviteTargetType>,
    /**
     * The id of the user whose stream to display for this invite
     * - Required if `target_type` is 1
     * - The user must be streaming in the channel
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<String>,
    /**
     * The id of the embedded application to open for this invite
     * - Required if `target_type` is 2
     * - The application must have the `EMBEDDED` flag
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_application_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#create-channel-invite}
 */
pub type RESTPostAPIChannelInviteResult = APIExtendedInvite;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#delete-channel-permission}
 */
pub type RESTDeleteAPIChannelPermissionResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#follow-news-channel}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RESTPostAPIChannelFollowersJSONBody {
    /**
     * ID of target channel
     */
    pub webhook_channel_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#follow-news-channel}
 */
pub type RESTPostAPIChannelFollowersResult = APIFollowedChannel;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#trigger-typing-indicator}
 */
pub type RESTPostAPIChannelTypingResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/message#get-channel-pins}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct RESTGetAPIChannelMessagesPinsQuery {
    /**
     * Get messages pinned before this timestamp
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /**
     * Maximum number of pins to return (1-50).
     *
     * @defaultValue `50`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/message#get-channel-pins}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTGetAPIChannelMessagesPinsResult {
    /**
     * Array of pinned messages
     */
    pub items: Vec<APIMessagePin>,
    /**
     * Whether there are more items available
     */
    pub has_more: bool,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/message#pin-message}
 */
pub type RESTPutAPIChannelMessagesPinResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/message#unpin-message}
 */
pub type RESTDeleteAPIChannelMessagesPinResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/message#get-pinned-messages-deprecated}
 * @deprecated
 */
#[deprecated]
pub type RESTGetAPIChannelPinsResult = Vec<APIMessage>;

/**
 * @see {@link https://discord.com/developers/docs/resources/message#pin-message-deprecated}
 * @deprecated
 */
#[deprecated]
pub type RESTPutAPIChannelPinResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/message#unpin-message-deprecated}
 * @deprecated
 */
#[deprecated]
pub type RESTDeleteAPIChannelPinResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#group-dm-add-recipient}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPutAPIChannelRecipientJSONBody {
    /**
     * Access token of a user that has granted your app the `gdm.join` scope
     */
    pub access_token: String,
    /**
     * Nickname of the user being added
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#group-dm-add-recipient}
 */
pub type RESTPutAPIChannelRecipientResult = Value;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#group-dm-remove-recipient}
 */
pub type RESTDeleteAPIChannelRecipientResult = Value;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#start-thread-from-message}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIChannelMessagesThreadsJSONBody {
    /**
     * 1-100 character thread name
     */
    pub name: String,
    /**
     * The amount of time in minutes to wait before automatically archiving the thread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    /**
     * Amount of seconds a user has to wait before sending another message (0-21600)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#start-thread-in-forum-or-media-channel}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIGuildForumThreadsJSONBody {
    /**
     * 1-100 character thread name
     */
    pub name: String,
    /**
     * The amount of time in minutes to wait before automatically archiving the thread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    /**
     * Amount of seconds a user has to wait before sending another message (0-21600)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,
    /**
     * The initial message of the thread
     */
    pub message: RESTPostAPIChannelMessageJSONBody,
    /**
     * The IDs of the set of tags to apply to the thread; limited to 5
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#start-thread-in-forum-or-media-channel}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIGuildForumThreadsFormDataBody {
    /**
     * 1-100 character thread name
     */
    pub name: String,
    /**
     * The amount of time in minutes to wait before automatically archiving the thread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    /**
     * Amount of seconds a user has to wait before sending another message (0-21600)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,
    /**
     * The initial message of the thread
     */
    pub message: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#start-thread-from-message}
 */
pub type RESTPostAPIChannelMessagesThreadsResult = APIChannel;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#start-thread-without-message}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPostAPIChannelThreadsJSONBody {
    /**
     * 1-100 character thread name
     */
    pub name: String,
    /**
     * The amount of time in minutes to wait before automatically archiving the thread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    /**
     * Amount of seconds a user has to wait before sending another message (0-21600)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,
    /**
     * The type of thread to create
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object-channel-types}
     * @defaultValue `ChannelType.PrivateThread` in API v9 and v10.
     * In a future API version this will be changed to be a required field, with no default.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ThreadChannelType>,
    /**
     * Whether non-moderators can add other non-moderators to the thread; only available when creating a private thread
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#start-thread-without-message}
 */
pub type RESTPostAPIChannelThreadsResult = APIChannel;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#join-thread}
 */
pub type RESTPutAPIChannelThreadMembersResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#leave-thread}
 */
pub type RESTDeleteAPIChannelThreadMembersResult = ();

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#get-thread-member}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct RESTGetAPIChannelThreadMemberQuery {
    /**
     * Whether to include a guild member object for the thread member
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_member: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#get-thread-member}
 */
pub type RESTGetAPIChannelThreadMemberResult = APIThreadMember;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#list-thread-members}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct RESTGetAPIChannelThreadMembersQuery {
    /**
     * Whether to include a guild member object for each thread member
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_member: Option<bool>,
    /**
     * Get thread members after this user ID
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /**
     * Max number of thread members to return (1-100)
     *
     * @defaultValue `100`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#list-thread-members}
 */
pub type RESTGetAPIChannelThreadMembersResult = Vec<APIThreadMember>;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#list-public-archived-threads}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct RESTGetAPIChannelThreadsArchivedQuery {
    /**
     * Get threads before this id or ISO8601 timestamp
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<SnowflakeOrIso8601>,
    /**
     * Max number of thread to return
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#list-public-archived-threads}
 */
pub type RESTGetAPIChannelThreadsArchivedPublicResult = RESTGetAPIChannelUsersThreadsArchivedResult;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#list-private-archived-threads}
 */
pub type RESTGetAPIChannelThreadsArchivedPrivateResult =
    RESTGetAPIChannelUsersThreadsArchivedResult;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#list-joined-private-archived-threads}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTGetAPIChannelUsersThreadsArchivedResult {
    #[serde(flatten)]
    pub list: APIThreadList,
    /**
     * Whether there are potentially additional threads
     */
    pub has_more: bool,
}
