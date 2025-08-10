use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /**
     * Gateway intents define which events your bot will receive from Discord's Gateway.
     * Selecting only the necessary intents helps improve performance and reduces event noise.
     *
     * Some intents are **privileged**, meaning they must be explicitly enabled
     * in the [Discord Developer Portal](https://discord.com/developers/applications).
     *
     * For more details, see
     * [Discord's documentation](https://discord.com/developers/docs/topics/gateway#gateway-intents).
     */
    pub struct GatewayIntents: u64 {
        /// Guild lifecycle and structure (guilds, roles, channels, threads, etc.).
        const GUILDS = 1 << 0;

        /// Member join/leave/update events (privileged).
        const GUILD_MEMBERS = 1 << 1;

        /// Moderation and audit log events.
        const GUILD_MODERATION = 1 << 2;

        /// Emoji and sticker updates.
        const GUILD_EMOJIS_AND_STICKERS = 1 << 3;

        /// Integrations updates.
        const GUILD_INTEGRATIONS = 1 << 4;

        /// Webhook updates.
        const GUILD_WEBHOOKS = 1 << 5;

        /// Invite create/delete.
        const GUILD_INVITES = 1 << 6;

        /// Voice state updates.
        const GUILD_VOICE_STATES = 1 << 7;

        /// Presence updates (privileged).
        const GUILD_PRESENCES = 1 << 8;

        /// Guild messages (dispatches without content unless MESSAGE_CONTENT is enabled).
        const GUILD_MESSAGES = 1 << 9;

        /// Reactions on guild messages.
        const GUILD_MESSAGE_REACTIONS = 1 << 10;

        /// Typing indicators in guilds.
        const GUILD_MESSAGE_TYPING = 1 << 11;

        /// Direct messages to the bot.
        const DIRECT_MESSAGES = 1 << 12;

        /// Reactions in DMs.
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;

        /// Typing indicators in DMs.
        const DIRECT_MESSAGE_TYPING = 1 << 14;

        /// Access to message content (privileged).
        const MESSAGE_CONTENT = 1 << 15;

        /// Scheduled events in guilds.
        const GUILD_SCHEDULED_EVENTS = 1 << 16;

        /// Auto moderation rule changes.
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;

        /// Auto moderation actions.
        const AUTO_MODERATION_EXECUTION = 1 << 21;

        /// Poll votes in guild messages.
        const GUILD_MESSAGE_POLLS = 1 << 24;

        /// Poll votes in direct messages.
        const DIRECT_MESSAGE_POLLS = 1 << 25;
    }
}

impl GatewayIntents {
    /// Minimal intents for basic bots: receive guild lifecycle and message events.
    /// Note: without MESSAGE_CONTENT, you may not get the actual message text.
    pub fn minimal() -> Self {
        Self::GUILDS | Self::GUILD_MESSAGES
    }

    /// Recommended intents for bots reacting to messages and reactions.
    pub fn recommended() -> Self {
        Self::minimal() | Self::GUILD_MESSAGE_REACTIONS
    }

    /// All available intents, including privileged ones.
    pub fn full() -> Self {
        Self::all()
    }

    /// Intents that require manual activation in the Developer Portal.
    pub fn privileged() -> Self {
        Self::GUILD_MEMBERS | Self::GUILD_PRESENCES | Self::MESSAGE_CONTENT
    }

    /// Intents that do not require manual activation.
    pub fn non_privileged() -> Self {
        Self::all() - Self::privileged()
    }
}
