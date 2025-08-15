use crate::models::payloads::{
    APIAutoModerationAction, APIAutoModerationRule, APIAutoModerationRuleTriggerMetadata,
    AutoModerationRuleEventType, AutoModerationRuleTriggerType,
};
use serde::{Deserialize, Serialize};

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#list-auto-moderation-rules-for-guild}
 */
pub type RESTGetAPIAutoModerationRulesResult = Vec<APIAutoModerationRule>;

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#get-auto-moderation-rule}
 */
pub type RESTGetAPIAutoModerationRuleResult = APIAutoModerationRule;

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#create-auto-moderation-rule}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RESTPostAPIAutoModerationRuleJSONBody {
    /**
     * The rule name
     */
    pub name: String,
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
     *
     * Can be omitted if the trigger type is {@link AutoModerationRuleTriggerType.Spam}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_metadata: Option<APIAutoModerationRuleTriggerMetadata>,
    /**
     * The actions which will execute when this rule is triggered
     */
    pub actions: Vec<APIAutoModerationAction>,
    /**
     * Whether this rule is enabled
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /**
     * The role ids that shouldn't be affected by this rule (Maximum of 20)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_roles: Option<Vec<String>>,
    /**
     * The channel ids that shouldn't be affected by this rule (Maximum of 50)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_channels: Option<Vec<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#create-auto-moderation-rule}
 */
pub type RESTPostAPIAutoModerationRuleResult = APIAutoModerationRule;

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#modify-auto-moderation-rule}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchAPIAutoModerationRuleJSONBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<AutoModerationRuleEventType>,
    /**
     * The rule trigger metadata
     *
     * Can be omitted if the trigger type is {@link AutoModerationRuleTriggerType.Spam}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_metadata: Option<APIAutoModerationRuleTriggerMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<APIAutoModerationAction>>,
    /**
     * Whether this rule is enabled
     *
     * @defaultValue `false`
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /**
     * The role ids that shouldn't be affected by this rule (Maximum of 20)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_roles: Option<Vec<String>>,
    /**
     * The channel ids that shouldn't be affected by this rule (Maximum of 50)
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_channels: Option<Vec<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#modify-auto-moderation-rule}
 */
pub type RESTPatchAPIAutoModerationRuleResult = APIAutoModerationRule;

/**
 * @see {@link https://discord.com/developers/docs/resources/auto-moderation#delete-auto-moderation-rule}
 */
pub type RESTDeleteAPIAutoModerationRuleResult = ();
