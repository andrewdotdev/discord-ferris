use serde::{Deserialize, Serialize};

use crate::models::payloads::application_commands::ApplicationCommandType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIBaseApplicationCommandInteractionData<T = ApplicationCommandType> {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: T,
    pub name: String,
    pub guild_id: Option<String>,
}
