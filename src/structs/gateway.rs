#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GatewayIntents(pub u64);

macro_rules! define_intents {
    (
        $( $name:ident = $bit:expr ),* $(,)?
    ) => {
        impl GatewayIntents {
            $(
                pub const $name: Self = Self(1 << $bit);
            )*

            pub const fn empty() -> Self {
                Self(0)
            }

            pub const fn all() -> Self {
                Self(
                    0 $( | (1 << $bit) )*
                )
            }

            pub const fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }

            pub const fn union(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }

            pub const fn remove(self, other: Self) -> Self {
                Self(self.0 & !other.0)
            }
        }
    }
}

define_intents! {
    GUILDS = 0,
    GUILD_MEMBERS = 1,
    GUILD_MODERATION = 2,
    GUILD_EMOJIS_AND_STICKERS = 3,
    GUILD_INTEGRATIONS = 4,
    GUILD_WEBHOOKS = 5,
    GUILD_INVITES = 6,
    GUILD_VOICE_STATES = 7,
    GUILD_PRESENCES = 8,
    GUILD_MESSAGES = 9,
    GUILD_MESSAGE_REACTIONS = 10,
    GUILD_MESSAGE_TYPING = 11,
    DIRECT_MESSAGES = 12,
    DIRECT_MESSAGE_REACTIONS = 13,
    DIRECT_MESSAGE_TYPING = 14,
    MESSAGE_CONTENT = 15,
    GUILD_SCHEDULED_EVENTS = 16,
    AUTO_MODERATION_CONFIGURATION = 20,
    AUTO_MODERATION_EXECUTION = 21,
    GUILD_MESSAGE_POLLS = 24,
    DIRECT_MESSAGE_POLLS = 25,
}

impl GatewayIntents {
    pub const fn minimal() -> Self {
        Self::GUILDS.union(Self::GUILD_MESSAGES)
    }

    pub const fn recommended() -> Self {
        Self::minimal().union(Self::GUILD_MESSAGE_REACTIONS)
    }

    pub const fn privileged() -> Self {
        Self::GUILD_MEMBERS
            .union(Self::GUILD_PRESENCES)
            .union(Self::MESSAGE_CONTENT)
    }

    pub const fn non_privileged() -> Self {
        Self::all().remove(Self::privileged())
    }
}
