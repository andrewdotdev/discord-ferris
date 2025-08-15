use crate::models::payloads::application::{APIApplication, APIApplicationRoleConnectionMetadata};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/**
 * @see {@link https://discord.com/developers/docs/resources/application-role-connection-metadata#get-application-role-connection-metadata-records}
 */
pub type RESTGetAPIApplicationRoleConnectionMetadataResult =
    Vec<APIApplicationRoleConnectionMetadata>;

/**
 * @see {@link https://discord.com/developers/docs/resources/application-role-connection-metadata#update-application-role-connection-metadata-records}
 */
pub type RESTPutAPIApplicationRoleConnectionMetadataJSONBody =
    Vec<APIApplicationRoleConnectionMetadata>;

/**
 * @see {@link https://discord.com/developers/docs/resources/application-role-connection-metadata#update-application-role-connection-metadata-records}
 */
pub type RESTPutAPIApplicationRoleConnectionMetadataResult =
    Vec<APIApplicationRoleConnectionMetadata>;

/**
 * @see {@link https://discord.com/developers/docs/resources/application#get-current-application}
 */
pub type RESTGetCurrentApplicationResult = APIApplication;

/**
 * @see {@link https://discord.com/developers/docs/resources/application#edit-current-application}
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RESTPatchCurrentApplicationJSONBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_install_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions_endpoint_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_connections_verification_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/**
 * @see {@link https://discord.com/developers/docs/resources/application#edit-current-application}
 */
pub type RESTPatchCurrentApplicationResult = APIApplication;
