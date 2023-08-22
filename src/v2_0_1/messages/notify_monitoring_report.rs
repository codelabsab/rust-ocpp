use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::monitoring_data_type::MonitoringDataType;

/// This contains the field definition of the NotifyMonitoringRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportRequest<'a> {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i64,
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub monitor: Option<Vec<MonitoringDataType<'a>>>,
}

/// Response to a NotifyMonitoringRequest message. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportResponse {}
