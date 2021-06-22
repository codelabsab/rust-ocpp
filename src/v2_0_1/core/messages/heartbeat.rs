use chrono::{DateTime, Utc};

/// This contains the field definition of the HeartbeatRequest PDU sent by the Charging Station to the CSMS. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatRequest {}

/// This contains the field definition of the HeartbeatRequest PDU sent by the Charging Station to the CSMS. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatResponse {
    pub current_time: DateTime<Utc>,
}
