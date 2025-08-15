use crate::models::payloads::application::APIApplication;
use crate::models::payloads::base::{
    APIInteractionDataResolved, APIMessageInteraction, APIMessageInteractionMetadata,
};
use crate::models::payloads::emoji::APIPartialEmoji;
use crate::models::payloads::guild::APIGuildMember;
use crate::models::payloads::permissions::APIRole;
use crate::models::payloads::poll::APIPoll;
use crate::models::payloads::sticker::{APISticker, APIStickerItem};
use crate::models::payloads::user::APIUser;
use crate::utils::serde::flags_numeric;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 * Types extracted from https://discord.com/developers/docs/resources/channel
 */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBasePartialChannel {
    /**
     * The id of the channel
     */
    pub id: String,
    /**
     * The type of the channel
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object-channel-types}
     */
    #[serde(rename = "type")]
    pub r#type: ChannelType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APINameableChannel {
    /**
     * The name of the channel (1-100 characters)
     */
    pub name: Option<String>,
}

/**
 * Not documented, but partial only includes id, name, and type
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPartialChannel {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: ChannelType,
    pub name: Option<String>,
}

/**
 * A channel obtained from fetching an invite.
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInviteChannel {
    /**
     * Icon hash.
     */
    pub icon: Option<String>,
    /**
     * The invite channel's recipients.
     *
     * @remarks Only includes usernames of users.
     */
    pub recipients: Option<Vec<APIInviteChannelRecipient>>,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: ChannelType,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIInviteChannelRecipient {
    pub username: String,
}

/**
 * Source channel of channel follower webhooks.
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIWebhookSourceChannel {
    pub id: String,
    pub name: String,
}

/**
 * This interface is used to allow easy extension for other channel types. While
 * also allowing `APIPartialChannel` to be used without breaking.
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIChannelBase {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: ChannelType,
    #[serde(with = "flags_numeric")]
    pub flags: ChannelFlags,
}

pub type TextChannelType = ChannelType;

pub type GuildChannelType = ChannelType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APISlowmodeChannel {
    /**
     * Amount of seconds a user has to wait before sending another message (0-21600);
     * bots, as well as users with the permission `MANAGE_MESSAGES` or `MANAGE_CHANNELS`, are unaffected
     *
     * `rate_limit_per_user` also applies to thread creation. Users can send one message and create one thread during each `rate_limit_per_user` interval.
     *
     * For thread channels, `rate_limit_per_user` is only returned if the field is set to a non-zero and non-null value.
     * The absence of this field in API calls and Gateway events should indicate that slowmode has been reset to the default value.
     */
    pub rate_limit_per_user: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APISortableChannel {
    /**
     * Sorting position of the channel
     */
    pub position: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APITextBasedChannel {
    /**
     * The id of the last message sent in this channel (may not point to an existing or valid message)
     */
    pub last_message_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPinChannel {
    /**
     * When the last pinned message was pinned.
     * This may be `null` in events such as `GUILD_CREATE` when a message is not pinned
     */
    pub last_pin_timestamp: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildChannel {
    /**
     * The name of the channel (1-100 characters)
     */
    pub name: String,
    /**
     * The id of the guild (may be missing for some channel objects received over gateway guild dispatches)
     */
    pub guild_id: Option<String>,
    /**
     * Explicit permission overwrites for members and roles
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#overwrite-object}
     */
    pub permission_overwrites: Option<Vec<APIOverwrite>>,
    /**
     * ID of the parent category for a channel (each parent category can contain up to 50 channels)
     *
     * OR
     *
     * ID of the parent channel for a thread
     */
    pub parent_id: Option<String>,
    /**
     * Whether the channel is nsfw
     */
    pub nsfw: Option<bool>,
}

pub type GuildTextChannelType = ChannelType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildTextChannel {
    /**
     * Default duration for newly created threads, in minutes, to automatically archive the thread after recent activity
     */
    pub default_auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    /**
     * The initial `rate_limit_per_user` to set on newly created threads.
     * This field is copied to the thread at creation time and does not live update
     */
    pub default_thread_rate_limit_per_user: Option<i64>,
    /**
     * The channel topic (0-1024 characters)
     */
    pub topic: Option<String>,
}

pub type APITextChannel = APIGuildTextChannel;
pub type APINewsChannel = APIGuildTextChannel;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildCategoryChannel {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIVoiceChannelBase {
    /**
     * The bitrate (in bits) of the voice or stage channel
     */
    pub bitrate: Option<i64>,
    /**
     * The user limit of the voice or stage channel
     */
    pub user_limit: Option<i64>,
    /**
     * Voice region id for the voice or stage channel, automatic when set to `null`
     *
     * @see {@link https://discord.com/developers/docs/resources/voice#voice-region-object}
     */
    pub rtc_region: Option<String>,
    /**
     * The camera video quality mode of the voice or stage channel, `1` when not present
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object-video-quality-modes}
     */
    pub video_quality_mode: Option<VideoQualityMode>,
}

pub type APIGuildVoiceChannel = APIVoiceChannelBase;

pub type APIGuildStageVoiceChannel = APIVoiceChannelBase;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIDMChannelBase {
    /**
     * The recipients of the DM
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub recipients: Option<Vec<APIUser>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIDMChannel {
    /**
     * The name of the channel (always null for DM channels)
     */
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGroupDMChannel {
    /**
     * The name of the channel (1-100 characters)
     */
    pub name: Option<String>,
    /**
     * Application id of the group DM creator if it is bot-created
     */
    pub application_id: Option<String>,
    /**
     * Icon hash
     */
    pub icon: Option<String>,
    /**
     * ID of the DM creator
     */
    pub owner_id: Option<String>,
    /**
     * The id of the last message sent in this channel (may not point to an existing or valid message)
     */
    pub last_message_id: Option<String>,
    /**
     * Whether the channel is managed by an OAuth2 application
     */
    pub managed: Option<bool>,
}

pub type ThreadChannelType = ChannelType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIThreadChannel {
    /**
     * The client users member for the thread, only included in select endpoints
     */
    pub member: Option<APIThreadMember>,
    /**
     * The metadata for a thread channel not shared by other channels
     */
    pub thread_metadata: Option<APIThreadMetadata>,
    /**
     * Number of messages (not including the initial message or deleted messages) in a thread
     *
     * If the thread was created before July 1, 2022, it stops counting at 50 messages
     */
    pub message_count: Option<i64>,
    /**
     * The approximate member count of the thread, does not count above 50 even if there are more members
     */
    pub member_count: Option<i64>,
    /**
     * ID of the thread creator
     */
    pub owner_id: Option<String>,
    /**
     * Number of messages ever sent in a thread
     *
     * Similar to `message_count` on message creation, but won't decrement when a message is deleted
     */
    pub total_message_sent: Option<i64>,
    /**
     * The IDs of the set of tags that have been applied to a thread in a thread-only channel
     */
    pub applied_tags: Vec<String>,
}

pub type APIPublicThreadChannel = APIThreadChannel;
pub type APIPrivateThreadChannel = APIThreadChannel;
pub type APIAnnouncementThreadChannel = APIThreadChannel;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#forum-tag-object-forum-tag-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildForumTag {
    /**
     * The id of the tag
     */
    pub id: String,
    /**
     * The name of the tag (0-20 characters)
     */
    pub name: String,
    /**
     * Whether this tag can only be added to or removed from threads by a member with the `MANAGE_THREADS` permission
     */
    pub moderated: bool,
    /**
     * The id of a guild's custom emoji
     */
    pub emoji_id: Option<String>,
    /**
     * The unicode character of the emoji
     */
    pub emoji_name: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#default-reaction-object-default-reaction-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildForumDefaultReactionEmoji {
    /**
     * The id of a guild's custom emoji
     */
    pub emoji_id: Option<String>,
    /**
     * The unicode character of the emoji
     */
    pub emoji_name: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel/#channel-object-sort-order-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SortOrderType {
    /**
     * Sort forum posts by activity
     */
    LatestActivity = 0,
    /**
     * Sort forum posts by creation time (from most recent to oldest)
     */
    CreationDate = 1,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel/#channel-object-forum-layout-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ForumLayoutType {
    /**
     * No default has been set for forum channel
     */
    NotSet = 0,
    /**
     * Display posts as a list
     */
    ListView = 1,
    /**
     * Display posts as a collection of tiles
     */
    GalleryView = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIThreadOnlyChannel {
    /**
     * The channel topic (0-4096 characters)
     */
    pub topic: Option<String>,
    /**
     * The id of the last thread created in this channel (may not point to an existing or valid thread)
     */
    pub last_message_id: Option<String>,
    /**
     * Amount of seconds a user has to wait before creating another thread (0-21600);
     * bots, as well as users with the permission `MANAGE_MESSAGES` or `MANAGE_CHANNELS`, are unaffected
     *
     * The absence of this field in API calls and Gateway events should indicate that slowmode has been reset to the default value.
     */
    pub rate_limit_per_user: Option<i64>,
    /**
     * When the last pinned message was pinned.
     * This may be `null` in events such as `GUILD_CREATE` when a message is not pinned
     */
    pub last_pin_timestamp: Option<String>,
    /**
     * Default duration for newly created threads, in minutes, to automatically archive the thread after recent activity
     */
    pub default_auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    /**
     * The set of tags that can be used in a thread-only channel
     */
    pub available_tags: Vec<APIGuildForumTag>,
    /**
     * The initial `rate_limit_per_user` to set on newly created threads.
     * This field is copied to the thread at creation time and does not live update
     */
    pub default_thread_rate_limit_per_user: Option<i64>,
    /**
     * The emoji to show in the add reaction button on a thread in a thread-only channel
     */
    pub default_reaction_emoji: Option<APIGuildForumDefaultReactionEmoji>,
    /**
     * The default sort order type used to order posts in a thread-only channel
     */
    pub default_sort_order: Option<SortOrderType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIGuildForumChannel {
    /**
     * The default layout type used to display posts in a forum channel
     *
     * @defaultValue `ForumLayoutType.NotSet` which indicates a layout view has not been set by a channel admin
     */
    pub default_forum_layout: ForumLayoutType,
}

pub type APIGuildMediaChannel = APIThreadOnlyChannel;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#channel-object-channel-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIChannel {
    APIAnnouncementThreadChannel(APIAnnouncementThreadChannel),
    APIDMChannel(APIDMChannel),
    APIGroupDMChannel(APIGroupDMChannel),
    APIGuildCategoryChannel(APIGuildCategoryChannel),
    APIGuildForumChannel(APIGuildForumChannel),
    APIGuildMediaChannel(APIGuildMediaChannel),
    APIGuildStageVoiceChannel(APIGuildStageVoiceChannel),
    APIGuildVoiceChannel(APIGuildVoiceChannel),
    APINewsChannel(APINewsChannel),
    APIPrivateThreadChannel(APIPrivateThreadChannel),
    APIPublicThreadChannel(APIPublicThreadChannel),
    APITextChannel(APITextChannel),
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#channel-object-channel-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ChannelType {
    /**
     * A text channel within a guild
     */
    GuildText = 0,
    /**
     * A direct message between users
     */
    DM = 1,
    /**
     * A voice channel within a guild
     */
    GuildVoice = 2,
    /**
     * A direct message between multiple users
     */
    GroupDM = 3,
    /**
     * An organizational category that contains up to 50 channels
     *
     * @see {@link https://support.discord.com/hc/articles/115001580171}
     */
    GuildCategory = 4,
    /**
     * A channel that users can follow and crosspost into their own guild
     *
     * @see {@link https://support.discord.com/hc/articles/360032008192}
     */
    GuildAnnouncement = 5,
    /**
     * A temporary sub-channel within a Guild Announcement channel
     */
    AnnouncementThread = 10,
    /**
     * A temporary sub-channel within a Guild Text or Guild Forum channel
     */
    PublicThread = 11,
    /**
     * A temporary sub-channel within a Guild Text channel that is only viewable by those invited and those with the Manage Threads permission
     */
    PrivateThread = 12,
    /**
     * A voice channel for hosting events with an audience
     *
     * @see {@link https://support.discord.com/hc/articles/1500005513722}
     */
    GuildStageVoice = 13,
    /**
     * The channel in a Student Hub containing the listed servers
     *
     * @see {@link https://support.discord.com/hc/articles/4406046651927}
     */
    GuildDirectory = 14,
    /**
     * A channel that can only contain threads
     */
    GuildForum = 15,
    /**
     * A channel like forum channels but contains media for server subscriptions
     *
     * @see {@link https://creator-support.discord.com/hc/articles/14346342766743}
     */
    GuildMedia = 16,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VideoQualityMode {
    /**
     * Discord chooses the quality for optimal performance
     */
    Auto = 1,
    /**
     * 720p
     */
    Full = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageMentions {
    /**
     * Users specifically mentioned in the message
     *
     * The `member` field is only present in `MESSAGE_CREATE` and `MESSAGE_UPDATE` events
     * from text-based guild channels
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     * @see {@link https://discord.com/developers/docs/resources/guild#guild-member-object}
     */
    pub mentions: Vec<APIUser>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBaseMessageNoChannel {
    /**
     * ID of the message
     */
    pub id: String,
    /**
     * The author of this message (only a valid user in the case where the message is generated by a user or bot user)
     *
     * If the message is generated by a webhook, the author object corresponds to the webhook's id,
     * username, and avatar. You can tell if a message is generated by a webhook by checking for the `webhook_id` property
     *
     * @see {@link https://discord.com/developers/docs/resources/user#user-object}
     */
    pub author: APIUser,
    /**
     * Contents of the message
     *
     * The `MESSAGE_CONTENT` privileged gateway intent is required for verified applications to receive a non-empty value from this field
     *
     * In the Discord Developers Portal, you need to enable the toggle of this intent of your application in **Bot \> Privileged Gateway Intents**.
     * You also need to specify the intent bit value (`1 << 15`) if you are connecting to the gateway
     *
     * @see {@link https://support-dev.discord.com/hc/articles/6207308062871}
     */
    pub content: String,
    /**
     * When this message was sent
     */
    pub timestamp: String,
    /**
     * When this message was edited (or null if never)
     */
    pub edited_timestamp: Option<String>,
    /**
     * Whether this was a TTS message
     */
    pub tts: bool,
    /**
     * Whether this message mentions everyone
     */
    pub mention_everyone: bool,
    /**
     * Roles specifically mentioned in this message
     *
     * @see {@link https://discord.com/developers/docs/topics/permissions#role-object}
     */
    pub mention_roles: Vec<APIRole>,
    /**
     * Channels specifically mentioned in this message
     *
     * Not all channel mentions in a message will appear in `mention_channels`.
     * - Only textual channels that are visible to everyone in a lurkable guild will ever be included
     * - Only crossposted messages (via Channel Following) currently include `mention_channels` at all
     *
     * If no mentions in the message meet these requirements, this field will not be sent
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-mention-object}
     */
    pub mention_channels: Option<Vec<APIChannelMention>>,
    /**
     * Any attached files
     *
     * @see {@link https://discord.com/developers/docs/resources/message#attachment-object-attachment-structure}
     *
     * The `MESSAGE_CONTENT` privileged gateway intent is required for verified applications to receive a non-empty value from this field
     *
     * In the Discord Developers Portal, you need to enable the toggle of this intent of your application in **Bot \> Privileged Gateway Intents**.
     * You also need to specify the intent bit value (`1 << 15`) if you are connecting to the gateway
     * @see {@link https://support-dev.discord.com/hc/articles/6207308062871}
     */
    pub attachments: Vec<APIAttachment>,
    /**
     * Any embedded content
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object}
     *
     * The `MESSAGE_CONTENT` privileged gateway intent is required for verified applications to receive a non-empty value from this field
     *
     * In the Discord Developers Portal, you need to enable the toggle of this intent of your application in **Bot \> Privileged Gateway Intents**.
     * You also need to specify the intent bit value (`1 << 15`) if you are connecting to the gateway
     * @see {@link https://support-dev.discord.com/hc/articles/6207308062871}
     */
    pub embeds: Vec<APIEmbed>,
    /**
     * Reactions to the message
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#reaction-object}
     */
    pub reactions: Option<Vec<APIReaction>>,
    /**
     * A nonce that can be used for optimistic message sending (up to 25 characters)
     *
     * **You will not receive this from further fetches. This is received only once from a `MESSAGE_CREATE`
     * event to ensure it got sent**
     */
    pub nonce: Option<serde_json::Value>,
    /**
     * Whether this message is pinned
     */
    pub pinned: bool,
    /**
     * If the message is generated by a webhook, this is the webhook's id
     */
    pub webhook_id: Option<String>,
    /**
     * Type of message
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-types}
     */
    pub r#type: MessageType,
    /**
     * Sent with Rich Presence-related chat embeds
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-activity-structure}
     */
    pub activity: Option<APIMessageActivity>,
    /**
     * Sent with Rich Presence-related chat embeds
     *
     * @see {@link https://discord.com/developers/docs/resources/application#application-object}
     */
    pub application: Option<APIApplication>,
    /**
     * If the message is a response to an Interaction, this is the id of the interaction's application
     */
    pub application_id: Option<String>,
    /**
     * Reference data sent with crossposted messages, replies, pins, and thread starter messages
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#message-reference-object-message-reference-structure}
     */
    pub message_reference: Option<APIMessageReference>,
    /**
     * Message flags combined as a bitfield
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-flags}
     * @see {@link https://en.wikipedia.org/wiki/Bit_field}
     */
    #[serde(default, with = "crate::utils::serde::flags_numeric_opt")]
    pub flags: Option<MessageFlags>,
    /**
     * The message associated with the `message_reference`
     *
     * This field is only returned for messages with a `type` of `19` (REPLY).
     *
     * If the message is a reply but the `referenced_message` field is not present,
     * the backend did not attempt to fetch the message that was being replied to,
     * so its state is unknown.
     *
     * If the field exists but is `null`, the referenced message was deleted
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#message-object}
     */
    pub referenced_message: Option<Box<APIMessage>>,
    /**
     * Sent if the message is sent as a result of an interaction
     */
    pub interaction_metadata: Option<APIMessageInteractionMetadata>,
    /**
     * Sent if the message is a response to an Interaction
     *
     * @deprecated In favor of `interaction_metadata`
     */
    pub interaction: Option<APIMessageInteraction>,
    /**
     * Sent if a thread was started from this message
     */
    pub thread: Option<APIChannel>,
    /**
     * Sent if the message contains components like buttons, action rows, or other interactive components
     *
     * The `MESSAGE_CONTENT` privileged gateway intent is required for verified applications to receive a non-empty value from this field
     *
     * In the Discord Developers Portal, you need to enable the toggle of this intent of your application in **Bot \> Privileged Gateway Intents**.
     * You also need to specify the intent bit value (`1 << 15`) if you are connecting to the gateway
     *
     * @see {@link https://support-dev.discord.com/hc/articles/6207308062871}
     */
    pub components: Option<Vec<APIMessageTopLevelComponent>>,
    /**
     * Sent if the message contains stickers
     *
     * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-item-object}
     */
    pub sticker_items: Option<Vec<APIStickerItem>>,
    /**
     * The stickers sent with the message
     *
     * @see {@link https://discord.com/developers/docs/resources/sticker#sticker-object}
     * @deprecated Use {@link APIBaseMessageNoChannel.sticker_items} instead
     */
    pub stickers: Option<Vec<APISticker>>,
    /**
     * A generally increasing integer (there may be gaps or duplicates) that represents the approximate position of the message in a thread
     *
     * It can be used to estimate the relative position of the message in a thread in company with `total_message_sent` on parent thread
     */
    pub position: Option<i64>,
    /**
     * Data of the role subscription purchase or renewal that prompted this `ROLE_SUBSCRIPTION_PURCHASE` message
     */
    pub role_subscription_data: Option<APIMessageRoleSubscriptionData>,
    /**
     * Data for users, members, channels, and roles in the message's auto-populated select menus
     *
     * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-resolved-data-structure}
     */
    pub resolved: Option<APIInteractionDataResolved>,
    /**
     * A poll!
     *
     * The `MESSAGE_CONTENT` privileged gateway intent is required for verified applications to receive a non-empty value from this field
     *
     * In the Discord Developers Portal, you need to enable the toggle of this intent of your application in **Bot \> Privileged Gateway Intents**.
     * You also need to specify the intent bit value (`1 << 15`) if you are connecting to the gateway
     *
     * @see {@link https://support-dev.discord.com/hc/articles/6207308062871}
     */
    pub poll: Option<APIPoll>,
    /**
     * The message associated with the message_reference. This is a minimal subset of fields in a message (e.g. author is excluded.)
     */
    pub message_snapshots: Option<Vec<APIMessageSnapshot>>,
    /**
     * The call associated with the message
     */
    pub call: Option<APIMessageCall>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBaseMessage {
    /**
     * ID of the channel the message was sent in
     */
    pub channel_id: String,
    #[serde(flatten)]
    pub base: APIBaseMessageNoChannel,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-structure}
 */
pub type APIMessage = APIBaseMessage;

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,
    ChannelFollowAdd = 12,

    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    AutoModerationAction = 24,
    RoleSubscriptionPurchase = 25,
    InteractionPremiumUpsell = 26,
    StageStart = 27,
    StageEnd = 28,
    StageSpeaker = 29,
    /**
     * @unstable https://github.com/discord/discord-api-docs/pull/5927#discussion_r1107678548
     */
    StageRaiseHand = 30,
    StageTopic = 31,
    GuildApplicationPremiumSubscription = 32,

    GuildIncidentAlertModeEnabled = 36,
    GuildIncidentAlertModeDisabled = 37,
    GuildIncidentReportRaid = 38,
    GuildIncidentReportFalseAlarm = 39,

    PurchaseNotification = 44,

    PollResult = 46,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-activity-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageActivity {
    /**
     * Type of message activity
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-activity-types}
     */
    pub r#type: MessageActivityType,
    /**
     * `party_id` from a Rich Presence event
     *
     * @see {@link https://discord.com/developers/docs/rich-presence/how-to#updating-presence-update-presence-payload-fields}
     */
    pub party_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-reference-object-message-reference-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageReference {
    /**
     * Type of reference
     */
    pub r#type: Option<MessageReferenceType>,
    /**
     * ID of the originating message
     */
    pub message_id: Option<String>,
    /**
     * ID of the originating message's channel
     */
    pub channel_id: String,
    /**
     * ID of the originating message's guild
     */
    pub guild_id: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-object-message-activity-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-reference-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageReferenceType {
    /**
     * A standard reference used by replies
     */
    Default = 0,
    /**
     * Reference used to point to a message at a point in time
     */
    Forward = 1,
}

bitflags! {
    /// Numeric bitflags as per Discord "message.flags".
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
    #[serde(transparent)]
    pub struct MessageFlags: u64 {
        const CROSSPOSTED                         = 1 << 0;
        const IS_CROSSPOST                        = 1 << 1;
        const SUPPRESS_EMBEDS                     = 1 << 2;
        const SOURCE_MESSAGE_DELETED              = 1 << 3;
        const URGENT                              = 1 << 4;
        const HAS_THREAD                          = 1 << 5;
        const EPHEMERAL                           = 1 << 6;
        const LOADING                             = 1 << 7;
        const FAILED_TO_MENTION_SOME_ROLES        = 1 << 8;
        const SHOULD_SHOW_LINK_NOT_DISCORD_WARN   = 1 << 10;
        const SUPPRESS_NOTIFICATIONS              = 1 << 12;
        const IS_VOICE_MESSAGE                    = 1 << 13;
        const HAS_SNAPSHOT                        = 1 << 14;
        const IS_COMPONENTS_V2                    = 1 << 15;
    }
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-call-object-message-call-object-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageCall {
    /**
     * Array of user ids that participated in the call
     */
    pub participants: Vec<String>,
    /**
     * ISO8601 timestamp when the call ended
     */
    pub ended_timestamp: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#role-subscription-data-object-role-subscription-data-object-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageRoleSubscriptionData {
    /**
     * The id of the SKU and listing the user is subscribed to
     */
    pub role_subscription_listing_id: String,
    /**
     * The name of the tier the user is subscribed to
     */
    pub tier_name: String,
    /**
     * The number of months the user has been subscribed for
     */
    pub total_months_subscribed: i64,
    /**
     * Whether this notification is for a renewal
     */
    pub is_renewal: bool,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#followed-channel-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIFollowedChannel {
    /**
     * Source channel id
     */
    pub channel_id: String,
    /**
     * Created target webhook id
     */
    pub webhook_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#reaction-object-reaction-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIReaction {
    /**
     * Total number of times this emoji has been used to react (including super reacts)
     */
    pub count: i64,
    /**
     * An object detailing the individual reaction counts for different types of reactions
     */
    pub count_details: APIReactionCountDetails,
    /**
     * Whether the current user reacted using this emoji
     */
    pub me: bool,
    /**
     * Whether the current user super-reacted using this emoji
     */
    pub me_burst: bool,
    /**
     * Emoji information
     *
     * @see {@link https://discord.com/developers/docs/resources/emoji#emoji-object}
     */
    pub emoji: APIPartialEmoji,
    /**
     * Hexadecimal colors used for this super reaction
     */
    pub burst_colors: Vec<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#reaction-count-details-object-reaction-count-details-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIReactionCountDetails {
    /**
     * Count of super reactions
     */
    pub burst: i64,
    /**
     * Count of normal reactions
     */
    pub normal: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#overwrite-object-overwrite-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIOverwrite {
    /**
     * Role or user id
     */
    pub id: String,
    /**
     * Either 0 (role) or 1 (member)
     */
    pub r#type: OverwriteType,
    /**
     * Permission bit set
     *
     * @see {@link https://discord.com/developers/docs/topics/permissions#permissions-bitwise-permission-flags}
     * @see {@link https://en.wikipedia.org/wiki/Bit_field}
     */
    pub allow: String,
    /**
     * Permission bit set
     *
     * @see {@link https://discord.com/developers/docs/topics/permissions#permissions-bitwise-permission-flags}
     * @see {@link https://en.wikipedia.org/wiki/Bit_field}
     */
    pub deny: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OverwriteType {
    Role = 0,
    Member = 1,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#thread-metadata-object-thread-metadata-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIThreadMetadata {
    /**
     * Whether the thread is archived
     */
    pub archived: bool,
    /**
     * Duration in minutes to automatically archive the thread after recent activity, can be set to: 60, 1440, 4320, 10080
     */
    pub auto_archive_duration: ThreadAutoArchiveDuration,
    /**
     * An ISO8601 timestamp when the thread's archive status was last changed, used for calculating recent activity
     */
    pub archive_timestamp: String,
    /**
     * Whether the thread is locked; when a thread is locked, only users with `MANAGE_THREADS` can unarchive it
     */
    pub locked: bool,
    /**
     * Whether non-moderators can add other non-moderators to the thread; only available on private threads
     */
    pub invitable: Option<bool>,
    /**
     * Timestamp when the thread was created; only populated for threads created after 2022-01-09
     */
    pub create_timestamp: Option<String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum ThreadAutoArchiveDuration {
    OneHour = 60,
    OneDay = 1_440,
    ThreeDays = 4_320,
    OneWeek = 10_080,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#thread-member-object-thread-member-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIThreadMember {
    /**
     * The id of the thread
     *
     * **This field is omitted on the member sent within each thread in the `GUILD_CREATE` event**
     */
    pub id: Option<String>,
    /**
     * The id of the member
     *
     * **This field is omitted on the member sent within each thread in the `GUILD_CREATE` event**
     */
    pub user_id: Option<String>,
    /**
     * An ISO8601 timestamp for when the member last joined
     */
    pub join_timestamp: String,
    /**
     * Member flags combined as a bitfield
     *
     * @see {@link https://en.wikipedia.org/wiki/Bit_field}
     */
    pub flags: ThreadMemberFlags,
    /**
     * Additional information about the user
     *
     * **This field is omitted on the member sent within each thread in the `GUILD_CREATE` event**
     *
     * **This field is only present when `with_member` is set to true when calling `List Thread Members` or `Get Thread Member`**
     */
    pub member: Option<APIGuildMember>,
}

bitflags! {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ThreadMemberFlags: u32 {
        /**
         * @unstable This thread member flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const HasInteracted = 1 << 0;
        /**
         * @unstable This thread member flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const AllMessages = 1 << 1;
        /**
         * @unstable This thread member flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const OnlyMentions = 1 << 2;
        /**
         * @unstable This thread member flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const NoMessages = 1 << 3;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIThreadList {
    /**
     * The threads that were fetched
     */
    pub threads: Vec<APIChannel>,
    /**
     * The members for the client user in each of the fetched threads
     */
    pub members: Vec<APIThreadMember>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-structure}
 *
 * Length limit: 6000 characters
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIEmbed {
    /**
     * Title of embed
     *
     * Length limit: 256 characters
     */
    pub title: Option<String>,
    /**
     * Type of embed (always "rich" for webhook embeds)
     */
    pub r#type: Option<EmbedType>,
    /**
     * Description of embed
     *
     * Length limit: 4096 characters
     */
    pub description: Option<String>,
    /**
     * URL of embed
     */
    pub url: Option<String>,
    /**
     * Timestamp of embed content
     */
    pub timestamp: Option<String>,
    /**
     * Color code of the embed
     */
    pub color: Option<i64>,
    /**
     * Footer information
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure}
     */
    pub footer: Option<APIEmbedFooter>,
    /**
     * Image information
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure}
     */
    pub image: Option<APIEmbedImage>,
    /**
     * Thumbnail information
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure}
     */
    pub thumbnail: Option<APIEmbedThumbnail>,
    /**
     * Video information
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure}
     */
    pub video: Option<APIEmbedVideo>,
    /**
     * Provider information
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure}
     */
    pub provider: Option<APIEmbedProvider>,
    /**
     * Author information
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure}
     */
    pub author: Option<APIEmbedAuthor>,
    /**
     * Fields information
     *
     * Length limit: 25 field objects
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure}
     */
    pub fields: Option<Vec<APIEmbedField>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-types}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbedType {
    /**
     * Generic embed rendered from embed attributes
     */
    #[serde(rename = "rich")]
    Rich,
    /**
     * Image embed
     */
    #[serde(rename = "image")]
    Image,
    /**
     * Video embed
     */
    #[serde(rename = "video")]
    Video,
    /**
     * Animated gif image embed rendered as a video embed
     */
    #[serde(rename = "gifv")]
    GIFV,
    /**
     * Article embed
     */
    #[serde(rename = "article")]
    Article,
    /**
     * Link embed
     */
    #[serde(rename = "link")]
    Link,
    /**
     * Auto moderation alert embed
     *
     * @unstable This embed type is currently not documented by Discord, but it is returned in the auto moderation system messages.
     */
    #[serde(rename = "auto_moderation_message")]
    AutoModerationMessage,
    /**
     * Poll result embed
     */
    #[serde(rename = "poll_result")]
    PollResult,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIEmbedThumbnail {
    /**
     * Source url of thumbnail (only supports http(s) and attachments)
     */
    pub url: String,
    /**
     * A proxied url of the thumbnail
     */
    pub proxy_url: Option<String>,
    /**
     * Height of thumbnail
     */
    pub height: Option<i64>,
    /**
     * Width of thumbnail
     */
    pub width: Option<i64>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIEmbedVideo {
    /**
     * Source url of video
     */
    pub url: Option<String>,
    /**
     * A proxied url of the video
     */
    pub proxy_url: Option<String>,
    /**
     * Height of video
     */
    pub height: Option<i64>,
    /**
     * Width of video
     */
    pub width: Option<i64>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIEmbedImage {
    /**
     * Source url of image (only supports http(s) and attachments)
     */
    pub url: String,
    /**
     * A proxied url of the image
     */
    pub proxy_url: Option<String>,
    /**
     * Height of image
     */
    pub height: Option<i64>,
    /**
     * Width of image
     */
    pub width: Option<i64>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIEmbedProvider {
    /**
     * Name of provider
     */
    pub name: Option<String>,
    /**
     * URL of provider
     */
    pub url: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIEmbedAuthor {
    /**
     * Name of author
     *
     * Length limit: 256 characters
     */
    pub name: String,
    /**
     * URL of author
     */
    pub url: Option<String>,
    /**
     * URL of author icon (only supports http(s) and attachments)
     */
    pub icon_url: Option<String>,
    /**
     * A proxied url of author icon
     */
    pub proxy_icon_url: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIEmbedFooter {
    /**
     * Footer text
     *
     * Length limit: 2048 characters
     */
    pub text: String,
    /**
     * URL of footer icon (only supports http(s) and attachments)
     */
    pub icon_url: Option<String>,
    /**
     * A proxied url of footer icon
     */
    pub proxy_icon_url: Option<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIEmbedField {
    /**
     * Name of the field
     *
     * Length limit: 256 characters
     */
    pub name: String,
    /**
     * Value of the field
     *
     * Length limit: 1024 characters
     */
    pub value: String,
    /**
     * Whether or not this field should display inline
     */
    pub inline: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/message#attachment-object-attachment-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAttachment {
    /**
     * Attachment id
     */
    pub id: String,
    /**
     * Name of file attached
     */
    pub filename: String,
    /**
     * The title of the file
     */
    pub title: Option<String>,
    /**
     * Description for the file
     */
    pub description: Option<String>,
    /**
     * The attachment's media type
     *
     * @see {@link https://en.wikipedia.org/wiki/Media_type}
     */
    pub content_type: Option<String>,
    /**
     * Size of file in bytes
     */
    pub size: i64,
    /**
     * Source url of file
     */
    pub url: String,
    /**
     * A proxied url of file
     */
    pub proxy_url: String,
    /**
     * Height of file (if image)
     */
    pub height: Option<i64>,
    /**
     * Width of file (if image)
     */
    pub width: Option<i64>,
    /**
     * Whether this attachment is ephemeral
     */
    pub ephemeral: Option<bool>,
    /**
     * The duration of the audio file (currently for voice messages)
     */
    pub duration_secs: Option<f64>,
    /**
     * Base64 encoded bytearray representing a sampled waveform (currently for voice messages)
     */
    pub waveform: Option<String>,
    /**
     * Attachment flags combined as a bitfield
     */
    pub flags: Option<AttachmentFlags>,
}

bitflags! {
    /**
    * @see {@link https://discord.com/developers/docs/resources/channel#attachment-object-attachment-structure-attachment-flags}
    */
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct AttachmentFlags: u32 {
        /**
         * This attachment has been edited using the remix feature on mobile
         */
        const IsRemix = 1 << 2;
    }
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#channel-mention-object-channel-mention-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIChannelMention {
    /**
     * ID of the channel
     */
    pub id: String,
    /**
     * ID of the guild containing the channel
     */
    pub guild_id: String,
    /**
     * The type of channel
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#channel-object-channel-types}
     */
    pub r#type: ChannelType,
    /**
     * The name of the channel
     */
    pub name: String,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#allowed-mentions-object-allowed-mention-types}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AllowedMentionsTypes {
    /**
     * Controls `@everyone` and `@here` mentions
     */
    Everyone,
    /**
     * Controls role mentions
     */
    Role,
    /**
     * Controls user mentions
     */
    User,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#allowed-mentions-object-allowed-mentions-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAllowedMentions {
    /**
     * An array of allowed mention types to parse from the content
     *
     * @see {@link https://discord.com/developers/docs/resources/channel#allowed-mentions-object-allowed-mention-types}
     */
    pub parse: Option<Vec<AllowedMentionsTypes>>,
    /**
     * Array of role_ids to mention (Max size of 100)
     */
    pub roles: Option<Vec<String>>,
    /**
     * Array of user_ids to mention (Max size of 100)
     */
    pub users: Option<Vec<String>>,
    /**
     * For replies, whether to mention the author of the message being replied to
     *
     * @defaultValue `false`
     */
    pub replied_user: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#anatomy-of-a-component}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBaseComponent<T> {
    /**
     * The type of the component
     */
    pub r#type: T,
    /**
     * 32 bit integer used as an optional identifier for component
     *
     * The id field is optional and is used to identify components in the response from an interaction that aren't interactive components. The id must be unique within the message and is generated sequentially if left empty. Generation of ids won't use another id that exists in the message if you have one defined for another component.
     */
    pub id: Option<i32>,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#component-object-component-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ComponentType {
    /**
     * Container to display a row of interactive components
     */
    ActionRow = 1,
    /**
     * Button component
     */
    Button = 2,
    /**
     * Select menu for picking from defined text options
     */
    StringSelect = 3,
    /**
     * Text Input component
     */
    TextInput = 4,
    /**
     * Select menu for users
     */
    UserSelect = 5,
    /**
     * Select menu for roles
     */
    RoleSelect = 6,
    /**
     * Select menu for users and roles
     */
    MentionableSelect = 7,
    /**
     * Select menu for channels
     */
    ChannelSelect = 8,
    /**
     * Container to display text alongside an accessory component
     */
    Section = 9,
    /**
     * Markdown text
     */
    TextDisplay = 10,
    /**
     * Small image that can be used as an accessory
     */
    Thumbnail = 11,
    /**
     * Display images and other media
     */
    MediaGallery = 12,
    /**
     * Displays an attached file
     */
    File = 13,
    /**
     * Component to add vertical padding between other components
     */
    Separator = 14,
    /**
     * @unstable This component type is currently not documented by Discord but has a known value which we will try to keep up to date.
     */
    ContentInventoryEntry = 16,
    /**
     * Container that visually groups a set of components
     */
    Container = 17,
}

/**
 * An Action Row is a top-level layout component used in messages and modals.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#action-row}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIActionRowComponent<T> {
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
    /**
     * The components in the ActionRow
     */
    pub components: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIButtonBase<Style> {
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
    /**
     * The style of the button
     */
    pub style: Style,
    /**
     * The status of the button
     */
    pub disabled: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#button}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIButtonComponentBase<Style> {
    #[serde(flatten)]
    pub base: APIButtonBase<Style>,
    /**
     * The label to be displayed on the button
     */
    pub label: Option<String>,
    /**
     * The emoji to display to the left of the text
     */
    pub emoji: Option<APIMessageComponentEmoji>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageComponentEmoji {
    /**
     * Emoji id
     */
    pub id: Option<String>,
    /**
     * Emoji name
     */
    pub name: Option<String>,
    /**
     * Whether this emoji is animated
     */
    pub animated: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#button}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIButtonComponentWithCustomId {
    #[serde(flatten)]
    pub base: APIButtonComponentBase<ButtonStyle>,
    /**
     * The custom_id to be sent in the interaction when clicked
     */
    pub custom_id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#button}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIButtonComponentWithURL {
    #[serde(flatten)]
    pub base: APIButtonComponentBase<ButtonStyle>,
    /**
     * The URL to direct users to when clicked for Link buttons
     */
    pub url: String,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#button}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIButtonComponentWithSKUId {
    #[serde(flatten)]
    pub base: APIButtonBase<ButtonStyle>,
    /**
     * The id for a purchasable SKU
     */
    pub sku_id: String,
}

/**
 * A Button is an interactive component that can only be used in messages. It creates clickable elements that users can interact with, sending an interaction to your app when clicked.
 *
 * Buttons must be placed inside an Action Row or a Section's accessory field.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#button}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIButtonComponent {
    APIButtonComponentWithCustomId(APIButtonComponentWithCustomId),
    APIButtonComponentWithSKUId(APIButtonComponentWithSKUId),
    APIButtonComponentWithURL(APIButtonComponentWithURL),
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#button-button-styles}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ButtonStyle {
    /**
     * The most important or recommended action in a group of options
     */
    Primary = 1,
    /**
     * Alternative or supporting actions
     */
    Secondary = 2,
    /**
     * Positive confirmation or completion actions
     */
    Success = 3,
    /**
     * An action with irreversible consequences
     */
    Danger = 4,
    /**
     * Navigates to a URL
     */
    Link = 5,
    /**
     * Purchase
     */
    Premium = 6,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#text-input-text-input-styles}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TextInputStyle {
    /**
     * Single-line input
     */
    Short = 1,
    /**
     * Multi-line input
     */
    Paragraph = 2,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBaseSelectMenuComponent<T> {
    /**
     * A developer-defined identifier for the select menu, max 100 characters
     */
    pub custom_id: String,
    /**
     * Custom placeholder text if nothing is selected, max 150 characters
     */
    pub placeholder: Option<String>,
    /**
     * The minimum number of items that must be chosen; min 0, max 25
     *
     * @defaultValue `1`
     */
    pub min_values: Option<i64>,
    /**
     * The maximum number of items that can be chosen; max 25
     *
     * @defaultValue `1`
     */
    pub max_values: Option<i64>,
    /**
     * Disable the select
     *
     * @defaultValue `false`
     */
    pub disabled: Option<bool>,
    #[serde(flatten)]
    pub base: APIBaseComponent<T>,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBaseAutoPopulatedSelectMenuComponent<T, D> {
    /**
     * List of default values for auto-populated select menu components
     */
    pub default_values: Option<Vec<APISelectMenuDefaultValue<D>>>,
    #[serde(flatten)]
    pub base: APIBaseSelectMenuComponent<T>,
}

/**
 * A String Select is an interactive component that allows users to select one or more provided options in a message.
 *
 * String Selects can be configured for both single-select and multi-select behavior. When a user finishes making their choice(s) your app receives an interaction.
 *
 * String Selects must be placed inside an Action Row and are only available in messages. An Action Row can contain only one select menu and cannot contain buttons if it has a select menu.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#string-select}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIStringSelectComponent {
    /**
     * Specified choices in a select menu; max 25
     */
    pub options: Vec<APISelectMenuOption>,
    #[serde(flatten)]
    pub base: APIBaseSelectMenuComponent<ComponentType>,
}

/**
 * A User Select is an interactive component that allows users to select one or more users in a message. Options are automatically populated based on the server's available users.
 *
 * User Selects can be configured for both single-select and multi-select behavior. When a user finishes making their choice(s) your app receives an interaction.
 *
 * User Selects must be placed inside an Action Row and are only available in messages. An Action Row can contain only one select menu and cannot contain buttons if it has a select menu.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#user-select}
 */
pub type APIUserSelectComponent =
    APIBaseAutoPopulatedSelectMenuComponent<ComponentType, SelectMenuDefaultValueType>;

/**
 * A Role Select is an interactive component that allows users to select one or more roles in a message. Options are automatically populated based on the server's available roles.
 *
 * Role Selects can be configured for both single-select and multi-select behavior. When a user finishes making their choice(s) your app receives an interaction.
 *
 * Role Selects must be placed inside an Action Row and are only available in messages. An Action Row can contain only one select menu and cannot contain buttons if it has a select menu.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#role-select}
 */
pub type APIRoleSelectComponent =
    APIBaseAutoPopulatedSelectMenuComponent<ComponentType, SelectMenuDefaultValueType>;

/**
 * A Mentionable Select is an interactive component that allows users to select one or more mentionables in a message. Options are automatically populated based on available mentionables in the server.
 *
 * Mentionable Selects can be configured for both single-select and multi-select behavior. When a user finishes making their choice(s), your app receives an interaction.
 *
 * Mentionable Selects must be placed inside an Action Row and are only available in messages. An Action Row can contain only one select menu and cannot contain buttons if it has a select menu.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#mentionable-select}
 */
pub type APIMentionableSelectComponent =
    APIBaseAutoPopulatedSelectMenuComponent<ComponentType, SelectMenuDefaultValueType>;

/**
 * A Channel Select is an interactive component that allows users to select one or more channels in a message. Options are automatically populated based on available channels in the server and can be filtered by channel types.
 *
 * Channel Selects can be configured for both single-select and multi-select behavior. When a user finishes making their choice(s) your app receives an interaction.
 *
 * Channel Selects must be placed inside an Action Row and are only available in messages. An Action Row can contain only one select menu and cannot contain buttons if it has a select menu.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#channel-select}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIChannelSelectComponent {
    /**
     * List of channel types to include in the ChannelSelect component
     */
    pub channel_types: Option<Vec<ChannelType>>,
    #[serde(flatten)]
    pub base: APIBaseAutoPopulatedSelectMenuComponent<ComponentType, SelectMenuDefaultValueType>,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#user-select-select-default-value-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SelectMenuDefaultValueType {
    Channel,
    Role,
    User,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#user-select-select-default-value-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APISelectMenuDefaultValue<T> {
    pub r#type: T,
    pub id: String,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIAutoPopulatedSelectMenuComponent {
    APIChannelSelectComponent(APIChannelSelectComponent),
    APIMentionableSelectComponent(APIMentionableSelectComponent),
    APIRoleSelectComponent(APIRoleSelectComponent),
    APIUserSelectComponent(APIUserSelectComponent),
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APISelectMenuComponent {
    APIChannelSelectComponent(APIChannelSelectComponent),
    APIMentionableSelectComponent(APIMentionableSelectComponent),
    APIRoleSelectComponent(APIRoleSelectComponent),
    APIStringSelectComponent(APIStringSelectComponent),
    APIUserSelectComponent(APIUserSelectComponent),
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#string-select-select-option-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APISelectMenuOption {
    /**
     * The user-facing name of the option (max 100 chars)
     */
    pub label: String,
    /**
     * The dev-defined value of the option (max 100 chars)
     */
    pub value: String,
    /**
     * An additional description of the option (max 100 chars)
     */
    pub description: Option<String>,
    /**
     * The emoji to display to the left of the option
     */
    pub emoji: Option<APIMessageComponentEmoji>,
    /**
     * Whether this option should be already-selected by default
     */
    pub default: Option<bool>,
}

/**
 * Text Input is an interactive component that allows users to enter free-form text responses in modals. It supports both short, single-line inputs and longer, multi-line paragraph inputs.
 *
 * Text Inputs can only be used within modals and must be placed inside an Action Row.
 *
 * When defining a text input component, you can set attributes to customize the behavior and appearance of it. However, not all attributes will be returned in the text input interaction payload.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#text-input}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APITextInputComponent {
    /**
     * One of text input styles
     */
    pub style: TextInputStyle,
    /**
     * The custom id for the text input
     */
    pub custom_id: String,
    /**
     * Text that appears on top of the text input field, max 45 characters
     */
    pub label: String,
    /**
     * Placeholder for the text input
     */
    pub placeholder: Option<String>,
    /**
     * The pre-filled text in the text input
     */
    pub value: Option<String>,
    /**
     * Minimal length of text input
     */
    pub min_length: Option<i64>,
    /**
     * Maximal length of text input
     */
    pub max_length: Option<i64>,
    /**
     * Whether or not this text input is required or not
     */
    pub required: Option<bool>,
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UnfurledMediaItemLoadingState {
    Unknown = 0,
    Loading = 1,
    LoadedSuccess = 2,
    LoadedNotFound = 3,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#unfurled-media-item-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIUnfurledMediaItem {
    /**
     * Supports arbitrary urls and attachment://<filename> references
     */
    pub url: String,
    /**
     * The proxied url of the media item. This field is ignored and provided by the API as part of the response
     */
    pub proxy_url: Option<String>,
    /**
     * The width of the media item. This field is ignored and provided by the API as part of the response
     */
    pub width: Option<i64>,
    /**
     * The height of the media item. This field is ignored and provided by the API as part of the response
     */
    pub height: Option<i64>,
    pub placeholder: Option<String>,
    pub placeholder_version: Option<i64>,
    /**
     * The media type of the content. This field is ignored and provided by the API as part of the response
     *
     * @see {@link https://en.wikipedia.org/wiki/Media_type}
     */
    pub content_type: Option<String>,
    pub loading_state: Option<UnfurledMediaItemLoadingState>,
    pub flags: Option<i64>,
    /**
     * The id of the uploaded attachment. This field is ignored and provided by the API as part of the response
     */
    pub attachment_id: Option<String>,
}

/**
 * A Section is a top-level layout component that allows you to join text contextually with an accessory.
 *
 * Sections are only available in messages.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#section}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APISectionComponent {
    /**
     * One to three text components
     */
    pub components: Vec<APITextDisplayComponent>,
    /**
     * A thumbnail or a button component, with a future possibility of adding more compatible components
     */
    pub accessory: APISectionAccessoryComponent,
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
}

/**
 * A Text Display is a top-level content component that allows you to add text to your message formatted with markdown and mention users and roles. This is similar to the content field of a message, but allows you to add multiple text components, controlling the layout of your message.
 *
 * Text Displays are only available in messages.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#text-display}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APITextDisplayComponent {
    /**
     * Text that will be displayed similar to a message
     */
    pub content: String,
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
}

/**
 * A Thumbnail is a content component that is a small image only usable as an accessory in a section. The preview comes from an url or attachment through the unfurled media item structure.
 *
 * Thumbnails are only available in messages as an accessory in a section.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#thumbnail}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIThumbnailComponent {
    /**
     * A url or attachment
     */
    pub media: APIUnfurledMediaItem,
    /**
     * Alt text for the media
     */
    pub description: Option<String>,
    /**
     * Whether the thumbnail should be a spoiler (or blurred out)
     *
     * @defaultValue `false`
     */
    pub spoiler: Option<bool>,
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#media-gallery-media-gallery-item-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMediaGalleryItem {
    /**
     * A url or attachment
     */
    pub media: APIUnfurledMediaItem,
    /**
     * Alt text for the media
     */
    pub description: Option<String>,
    /**
     * Whether the media should be a spoiler (or blurred out)
     *
     * @defaultValue `false`
     */
    pub spoiler: Option<bool>,
}

/**
 * A Media Gallery is a top-level content component that allows you to display 1-10 media attachments in an organized gallery format. Each item can have optional descriptions and can be marked as spoilers.
 *
 * Media Galleries are only available in messages.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#media-gallery}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMediaGalleryComponent {
    /**
     * 1 to 10 media gallery items
     */
    pub items: Vec<APIMediaGalleryItem>,
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
}

/**
 * A File is a top-level component that allows you to display an uploaded file as an attachment to the message and reference it in the component.
 *
 * Each file component can only display 1 attached file, but you can upload multiple files and add them to different file components within your payload.
 *
 * Files are only available in messages.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#file}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIFileComponent {
    /**
     * This unfurled media item is unique in that it **only** support attachment references using the `attachment://<filename>` syntax
     */
    pub file: APIUnfurledMediaItem,

    /**
     * Whether the media should be a spoiler (or blurred out)
     *
     * @defaultValue `false`
     */
    pub spoiler: Option<bool>,
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#separator}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SeparatorSpacingSize {
    Small = 1,
    Large = 2,
}

/**
 * A Separator is a top-level layout component that adds vertical padding and visual division between other components.
 *
 * Separators are only available in messages.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#separator}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APISeparatorComponent {
    /**
     * Whether a visual divider should be displayed in the component
     *
     * @defaultValue `true`
     */
    pub divider: Option<bool>,
    /**
     * Size of separator padding
     *
     * @defaultValue `SeparatorSpacingSize.Small`
     */
    pub spacing: Option<SeparatorSpacingSize>,
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
}

/**
 * A Container is a top-level layout component that holds up to 10 components. Containers are visually distinct from surrounding components and has an optional customizable color bar.
 *
 * Containers are only available in messages.
 *
 * @see {@link https://discord.com/developers/docs/components/reference#container}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIContainerComponent {
    /**
     * Color for the accent on the container as RGB from `0x000000` to `0xFFFFFF`
     */
    pub accent_color: Option<i64>,
    /**
     * Whether the container should be a spoiler (or blurred out)
     *
     * @defaultValue `false`
     */
    pub spoiler: Option<bool>,
    /**
     * Up to 10 components of the type action row, text display, section, media gallery, separator, or file
     */
    pub components: Vec<APIComponentInContainer>,
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/channel#message-snapshot-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageSnapshot {
    /**
     * Subset of the message object fields
     */
    pub message: APIMessageSnapshotFields,
    /**
     * Id of the origin message's guild
     *
     * @deprecated This field doesn't accurately reflect the Discord API as it doesn't exist nor is documented and will
     * be removed in the next major version.
     *
     * It was added in {@link https://github.com/discord/discord-api-docs/pull/6833/commits/d18f72d06d62e6b1d51ca2c1ef308ddc29ff3348 | d18f72d}
     * but was later removed before the PR ({@link https://github.com/discord/discord-api-docs/pull/6833 | discord-api-docs#6833}) was merged.
     * @see {@link https://github.com/discordjs/discord-api-types/pull/1084 | discord-api-types#1084} for more information.
     */
    pub guild_id: Option<String>,
}

bitflags! {
    /**
    * @see {@link https://discord.com/developers/docs/resources/channel#channel-object-channel-flags}
    */
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

    pub struct ChannelFlags: u32 {
        /**
         * @unstable This channel flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const GuildFeedRemoved = 1 << 0;
        /**
         * This thread is pinned to the top of its parent forum channel
         */
        const Pinned = 1 << 1;
        /**
         * @unstable This channel flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const ActiveChannelsRemoved = 1 << 2;
        /**
         * Whether a tag is required to be specified when creating a thread in a forum channel.
         * Tags are specified in the `applied_tags` field
         */
        const RequireTag = 1 << 4;
        /**
         * @unstable This channel flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const IsSpam = 1 << 5;
        /**
         * @unstable This channel flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const IsGuildResourceChannel = 1 << 7;
        /**
         * @unstable This channel flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const ClydeAI = 1 << 8;
        /**
         * @unstable This channel flag is currently not documented by Discord but has a known value which we will try to keep up to date.
         */
        const IsScheduledForDeletion = 1 << 9;
        /**
         * Whether media download options are hidden.
         */
        const HideMediaDownloadOptions = 1 << 15;
    }
}

/**
 * All components that can appear in messages.
 *
 * For more specific sets, see {@link APIMessageTopLevelComponent}, {@link APIComponentInMessageActionRow}, {@link APIComponentInContainer}, and {@link APISectionAccessoryComponent}
 *
 * @see {@link https://discord.com/developers/docs/components/reference}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIMessageComponent {
    APIActionRowComponent(APIActionRowComponent<APIComponentInMessageActionRow>),
    APIButtonComponent(APIButtonComponent),
    APIContainerComponent(APIContainerComponent),
    APIFileComponent(APIFileComponent),
    APIMediaGalleryComponent(APIMediaGalleryComponent),
    APISectionComponent(APISectionComponent),
    APISelectMenuComponent(APISelectMenuComponent),
    APISeparatorComponent(APISeparatorComponent),
    APITextDisplayComponent(APITextDisplayComponent),
    APIThumbnailComponent(APIThumbnailComponent),
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIMessageTopLevelComponent {
    APIActionRowComponent(APIActionRowComponent<APIComponentInMessageActionRow>),
    APIContainerComponent(APIContainerComponent),
    APIFileComponent(APIFileComponent),
    APIMediaGalleryComponent(APIMediaGalleryComponent),
    APISectionComponent(APISectionComponent),
    APISeparatorComponent(APISeparatorComponent),
    APITextDisplayComponent(APITextDisplayComponent),
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIModalComponent {
    APIActionRowComponent(APIActionRowComponent<APIComponentInModalActionRow>),
    APIComponentInModalActionRow(APIComponentInModalActionRow),
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#action-row}
 */
pub type APIComponentInActionRow = APIComponentInMessageActionRow;

/**
 * @see {@link https://discord.com/developers/docs/components/reference#action-row}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIComponentInMessageActionRow {
    APIButtonComponent(APIButtonComponent),
    APISelectMenuComponent(APISelectMenuComponent),
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#action-row}
 */
pub type APIComponentInModalActionRow = APITextInputComponent;

/**
 * @see {@link https://discord.com/developers/docs/components/reference#section}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APISectionAccessoryComponent {
    APIButtonComponent(APIButtonComponent),
    APIThumbnailComponent(APIThumbnailComponent),
}

/**
 * @see {@link https://discord.com/developers/docs/components/reference#container}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIComponentInContainer {
    APIActionRowComponent(APIActionRowComponent<APIComponentInMessageActionRow>),
    APIFileComponent(APIFileComponent),
    APIMediaGalleryComponent(APIMediaGalleryComponent),
    APISectionComponent(APISectionComponent),
    APISeparatorComponent(APISeparatorComponent),
    APITextDisplayComponent(APITextDisplayComponent),
}

/**
 * https://discord.com/developers/docs/resources/message#message-snapshot-object
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessageSnapshotFields {
    pub attachments: Vec<APIAttachment>,
    pub components: Option<Vec<APIMessageTopLevelComponent>>,
    pub content: String,
    pub edited_timestamp: Option<String>,
    pub embeds: Vec<APIEmbed>,
    pub flags: MessageFlags,
    pub mention_roles: Vec<APIRole>,
    pub mentions: Vec<APIUser>,
    pub sticker_items: Option<Vec<APIStickerItem>>,
    pub stickers: Option<Vec<APISticker>>,
    pub timestamp: String,
    pub r#type: MessageType,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/message#message-pin-object}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIMessagePin {
    /**
     * The time the message was pinned
     */
    pub pinned_at: String,
    /**
     * The pinned message
     */
    pub message: APIMessage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIRoleId {
    pub id: String,
}
