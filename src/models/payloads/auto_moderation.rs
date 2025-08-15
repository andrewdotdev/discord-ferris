use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 * Types extracted from https://discord.com/developers/docs/resources/auto-moderation
 */

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-auto-moderation-rule-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAutoModerationRule {
    /**
     * The id of this rule
     */
    pub id: String,
    /**
     * The guild which this rule belongs to
     */
    pub guild_id: String,
    /**
     * The rule name
     */
    pub name: String,
    /**
     * The user id who created this rule
     */
    pub creator_id: String,
    /**
     * The rule event type
     */
    pub event_type: AutoModerationRuleEventType,
    /**
     * The rule trigger type
     */
    pub trigger_type: AutoModerationRuleTriggerType,
    /**
     * The rule trigger metadata
     */
    pub trigger_metadata: APIAutoModerationRuleTriggerMetadata,
    /**
     * The actions which will execute when this rule is triggered
     */
    pub actions: Vec<APIAutoModerationAction>,
    /**
     * Whether this rule is enabled
     */
    pub enabled: bool,
    /**
     * The role ids that shouldn't be affected by this rule (Maximum of 20)
     */
    pub exempt_roles: Vec<String>,
    /**
     * The channel ids that shouldn't be affected by this rule (Maximum of 50)
     */
    pub exempt_channels: Vec<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutoModerationRuleTriggerType {
    /**
     * Check if content contains words from a user defined list of keywords (Maximum of 6 per guild)
     */
    Keyword = 1,
    /**
     * Check if content represents generic spam (Maximum of 1 per guild)
     */
    Spam = 3,
    /**
     * Check if content contains words from internal pre-defined wordsets (Maximum of 1 per guild)
     */
    KeywordPreset = 4,
    /**
     * Check if content contains more mentions than allowed (Maximum of 1 per guild)
     */
    MentionSpam = 5,
    /**
     * Check if member profile contains words from a user defined list of keywords (Maximum of 1 per guild)
     */
    MemberProfile = 6,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAutoModerationRuleTriggerMetadata {
    /**
     * Substrings which will be searched for in content (Maximum of 1000)
     *
     * A keyword can be a phrase which contains multiple words. Wildcard symbols can be used to customize how each string will be matched. Each keyword must be 60 characters or less
     *
     * @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-keyword-matching-strategies | Keyword matching strategies}
     *
     * Associated trigger types: {@link AutoModerationRuleTriggerType.Keyword}, {@link AutoModerationRuleTriggerType.MemberProfile}
     */
    pub keyword_filter: Option<Vec<String>>,
    /**
     * The internally pre-defined wordsets which will be searched for in content
     *
     * Associated trigger type: {@link AutoModerationRuleTriggerType.KeywordPreset}
     */
    pub presets: Option<Vec<AutoModerationRuleKeywordPresetType>>,
    /**
     * Substrings which will be exempt from triggering the preset trigger type (Maximum of 1000)
     *
     * A allowed-word can be a phrase which contains multiple words. Wildcard symbols can be used to customize how each string will be matched. Each keyword must be 60 characters or less
     *
     * @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-keyword-matching-strategies | Keyword matching strategies}
     *
     * Associated trigger types: {@link AutoModerationRuleTriggerType.Keyword}, {@link AutoModerationRuleTriggerType.KeywordPreset}, {@link AutoModerationRuleTriggerType.MemberProfile}
     */
    pub allow_list: Option<Vec<String>>,
    /**
     * Regular expression patterns which will be matched against content (Maximum of 10)
     *
     * Only Rust flavored regex is currently supported (Maximum of 260 characters)
     *
     * Associated trigger types: {@link AutoModerationRuleTriggerType.Keyword}, {@link AutoModerationRuleTriggerType.MemberProfile}
     */
    pub regex_patterns: Option<Vec<String>>,
    /**
     * Total number of mentions (role & user) allowed per message (Maximum of 50)
     *
     * Associated trigger type: {@link AutoModerationRuleTriggerType.MentionSpam}
     */
    pub mention_total_limit: Option<i64>,
    /**
     * Whether to automatically detect mention raids
     *
     * Associated trigger type: {@link AutoModerationRuleTriggerType.MentionSpam}
     */
    pub mention_raid_protection_enabled: Option<bool>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-keyword-preset-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutoModerationRuleKeywordPresetType {
    /**
     * Words that may be considered forms of swearing or cursing
     */
    Profanity = 1,
    /**
     * Words that refer to sexually explicit behavior or activity
     */
    SexualContent = 2,
    /**
     * Personal insults or words that may be considered hate speech
     */
    Slurs = 3,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-event-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutoModerationRuleEventType {
    /**
     * When a member sends or edits a message in the guild
     */
    MessageSend = 1,
    /**
     * When a member edits their profile
     */
    MemberUpdate = 2,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-auto-moderation-action-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAutoModerationAction {
    /**
     * The action type
     */
    #[serde(rename = "type")]
    pub r#type: AutoModerationActionType,
    /**
     * Additional metadata needed during execution for this specific action type
     *
     * Will only be omitted if the action type is {@link AutoModerationActionType.BlockMessage}
     */
    pub metadata: Option<APIAutoModerationActionMetadata>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-types}
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AutoModerationActionType {
    /**
     * Blocks a member's message and prevents it from being posted.
     * A custom explanation can be specified and shown to members whenever their message is blocked
     */
    BlockMessage = 1,
    /**
     * Logs user content to a specified channel
     */
    SendAlertMessage = 2,
    /**
     * Timeout user for specified duration, this action type can be set if the bot has `MODERATE_MEMBERS` permission
     */
    Timeout = 3,
    /**
     * Prevents a member from using text, voice, or other interactions
     */
    BlockMemberInteraction = 4,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-metadata}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIAutoModerationActionMetadata {
    /**
     * Channel to which user content should be logged
     *
     * Associated action type: {@link AutoModerationActionType.SendAlertMessage}
     */
    pub channel_id: Option<String>,
    /**
     * Timeout duration in seconds (Maximum of 4 weeks - 2419200 seconds)
     *
     * Only available if using {@link AutoModerationRuleTriggerType.Keyword}
     *
     * Associated action type: {@link AutoModerationActionType.Timeout}
     */
    pub duration_seconds: Option<i64>,
    /**
     * Additional explanation that will be shown to members whenever their message is blocked (Maximum 150 characters)
     *
     * Associated action type {@link AutoModerationActionType.BlockMessage}
     */
    pub custom_message: Option<String>,
}
