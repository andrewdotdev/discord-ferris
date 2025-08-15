use crate::models::payloads::base::APIBaseInteraction;
use crate::models::payloads::responses::InteractionType;

pub type APIPingInteraction = APIBaseInteraction<InteractionType, ()>;
