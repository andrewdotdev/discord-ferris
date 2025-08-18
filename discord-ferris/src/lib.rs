pub mod client;
pub mod framework;
pub mod gateway;
pub mod http;
pub mod models;
pub mod structs;
pub mod utils;

pub use crate::utils::log;
pub use discord_ferris_macros::event_handler;

pub mod prelude {
    pub use crate::framework::context::Ctx;
    pub use crate::models::gateway::*;
    pub use discord_ferris_macros::event_handler;
}

#[doc(hidden)]
pub use crate as discord_ferris;

#[doc(hidden)]
pub mod __private {
    pub use crate::framework::context::Ctx;
    pub use crate::framework::events::registry;
    pub use crate::models::gateway::GatewayDispatchEvents;
    pub use crate::models::gateway::*;
    pub use ::inventory;
}
