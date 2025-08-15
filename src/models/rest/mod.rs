//! Public REST surface.
//!
//! This module re-exports everything from its submodules so people can do:
//! `use crate::rest::*;`
//! or reach items like: `crate::rest::RESTPostAPIWebhookWithTokenJSONBody`

// ---- Submodules (each should correspond to a `*.rs` file in this folder) ----

pub mod application;
pub mod audit_log;
pub mod auto_moderation;
pub mod channel;
pub mod common;
pub mod emoji;
pub mod gateway;
pub mod guild;
pub mod guild_scheduled_event;
pub mod interactions;
pub mod invite;
pub mod monetization;
pub mod oauth2;
pub mod poll;
pub mod soundboard;
pub mod stage_instance;
pub mod sticker;
pub mod template;
pub mod user;
pub mod voice;
pub mod webhook;

// ---- Re-exports (flatten the surface so users don’t care about file boundaries) ----

#[allow(ambiguous_glob_reexports, unused_imports)]
pub use application::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use audit_log::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use auto_moderation::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use channel::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use common::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use emoji::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use gateway::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use guild::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use guild_scheduled_event::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use interactions::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use invite::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use monetization::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use oauth2::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use poll::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use soundboard::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use stage_instance::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use sticker::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use template::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use user::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use voice::*;
#[allow(ambiguous_glob_reexports, unused_imports)]
pub use webhook::*;
