use crate::models::payloads::audit_log::{APIAuditLog, AuditLogEvent};
use serde::{Deserialize, Serialize};

/// @see {@link https://discord.com/developers/docs/resources/audit-log#get-guild-audit-log}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct RESTGetAPIAuditLogQuery {
    /// Filter the log for actions made by a user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The type of audit log events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<AuditLogEvent>,

    /// Filter the log before a certain entry ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Filter the log after a certain entry ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// How many entries are returned (default 50, minimum 1, maximum 100)
    ///
    /// @defaultValue `50`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

pub type RESTGetAPIAuditLogResult = APIAuditLog;
