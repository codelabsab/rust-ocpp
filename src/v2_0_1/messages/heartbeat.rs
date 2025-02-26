//! HeartBeat

use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::helpers::datetime_rfc3339;

/// HeartbeatRequest, sent by the Charging Station to the CSMS. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatRequest {
    // No fields are defined
}

/// HeartbeatResponse, sent by the CSMS to the Charging Station in response to a HeartbeatRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatResponse {
    /// Contains the current time of the CSMS.
    #[serde(with = "datetime_rfc3339 ")]
    pub current_time: DateTime<Utc>,
}
