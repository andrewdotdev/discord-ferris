// SPDX-License-Identifier: MIT
// Adapted from discord-api-types (c) vladfrangu
// Rust port and modifications (c) 2025 andrewdotdev

use crate::models::payloads::base::APIBaseInteraction;
use crate::models::payloads::responses::InteractionType;

pub type APIPingInteraction = APIBaseInteraction<InteractionType, ()>;
