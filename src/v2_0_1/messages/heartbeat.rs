//! HeartBeat

use chrono::DateTime;
use chrono::Utc;

/// HeartbeatRequest, sent by the Charging Station to the CSMS. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatRequest {
    // No fields are defined
}

/// HeartbeatResponse, sent by the CSMS to the Charging Station in response to a HeartbeatRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatResponse {
    /// Contains the current time of the CSMS.
    pub current_time: DateTime<Utc>,
}
