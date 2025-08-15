use crate::models::payloads::{APIGatewayBotInfo, APIGatewayInfo};

/// @see {@link https://discord.com/developers/docs/topics/gateway#get-gateway}
pub type RESTGetAPIGatewayResult = APIGatewayInfo;

/// @see {@link https://discord.com/developers/docs/topics/gateway#get-gateway-bot}
pub type RESTGetAPIGatewayBotResult = APIGatewayBotInfo;
