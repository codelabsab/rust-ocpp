use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::event_data_type::EventDataType;

/// This contains the field definition of the NotifyEventRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEventRequest {
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i64,
    pub event_data: Vec<EventDataType>,
}

/// Response to NotifyEventRequest. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEventResponse {}
