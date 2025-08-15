use crate::models::payloads::base::{
    APIDMInteractionWrapper, APIGuildInteractionWrapper,
};
use crate::models::payloads::channel::APIBaseComponent;
use crate::models::payloads::responses::InteractionType;
use crate::models::payloads::{ComponentType, base::APIBaseInteraction};

use serde::{Deserialize, Serialize};

/**
 * A simple component entry submitted within a modal.
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModalSubmitComponent {
    pub r#type: ComponentType,
    pub custom_id: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModalSubmitActionRowComponent {
    #[serde(flatten)]
    pub base: APIBaseComponent<ComponentType>,
    pub components: Vec<ModalSubmitComponent>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-modal-submit-data-structure}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APIModalSubmission {
    /**
     * A developer-defined identifier for the component, max 100 characters
     */
    pub custom_id: String,
    /**
     * A list of child components
     */
    pub components: Vec<ModalSubmitActionRowComponent>,
}

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIModalSubmitInteraction = APIBaseInteraction<InteractionType, APIModalSubmission>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIModalSubmitDMInteraction = APIDMInteractionWrapper<APIModalSubmitInteraction>;

/**
 * @see {@link https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object}
 */
pub type APIModalSubmitGuildInteraction = APIGuildInteractionWrapper<APIModalSubmitInteraction>;
