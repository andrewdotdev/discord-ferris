// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use serde::{Deserialize, Serialize};

use super::emoji::APIPartialEmoji;

/**
 * Types extracted from https://discord.com/developers/docs/resources/poll
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBasePoll {
    /**
     * The question of the poll
     */
    pub question: APIPollMedia,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPollDefaults {
    /**
     * Whether a user can select multiple answers
     *
     * @defaultValue `false`
     */
    pub allow_multiselect: bool,
    /**
     * The layout type of the poll
     *
     * @defaultValue `PollLayoutType.Default`
     */
    pub layout_type: PollLayoutType,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/poll#poll-object-poll-object-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPoll {
    /**
     * Each of the answers available in the poll, up to 10
     */
    pub answers: Vec<APIPollAnswer>,
    /**
     * The time when the poll ends (IS08601 timestamp)
     */
    pub expiry: String,
    /**
     * The results of the poll
     */
    pub results: Option<APIPollResults>,

    // Inlined from APIBasePoll
    /**
     * The question of the poll
     */
    pub question: APIPollMedia,
    // Inlined from APIPollDefaults
    /**
     * Whether a user can select multiple answers
     *
     * @defaultValue `false`
     */
    pub allow_multiselect: bool,
    /**
     * The layout type of the poll
     *
     * @defaultValue `PollLayoutType.Default`
     */
    pub layout_type: PollLayoutType,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/poll#layout-type}
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PollLayoutType {
    /**
     * The, uhm, default layout type
     */
    Default = 1,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/poll#poll-media-object-poll-media-object-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPollMedia {
    /**
     * The text of the field
     *
     * The maximum length is `300` for the question, and `55` for any answer
     */
    pub text: Option<String>,
    /**
     * The emoji of the field
     */
    pub emoji: Option<APIPartialEmoji>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBasePollAnswer {
    /**
     * The data of the answer
     */
    pub poll_media: APIPollMedia,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/poll#poll-answer-object-poll-answer-object-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPollAnswer {
    /**
     * The ID of the answer. Starts at `1` for the first answer and goes up sequentially
     */
    pub answer_id: i32,
    // Inlined from APIBasePollAnswer
    /**
     * The data of the answer
     */
    pub poll_media: APIPollMedia,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/poll#poll-results-object-poll-results-object-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPollResults {
    /**
     * Whether the votes have been precisely counted
     */
    pub is_finalized: bool,
    /**
     * The counts for each answer
     */
    pub answer_counts: Vec<APIPollAnswerCount>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/poll#poll-results-object-poll-answer-count-object-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIPollAnswerCount {
    /**
     * The `answer_id`
     */
    pub id: i32,
    /**
     * The number of votes for this answer
     */
    pub count: i32,
    /**
     * Whether the current user voted for this answer
     */
    pub me_voted: bool,
}
