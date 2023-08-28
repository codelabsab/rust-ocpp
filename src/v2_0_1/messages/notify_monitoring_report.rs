use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::monitoring_data_type::MonitoringDataType;
use crate::Vec;

/// This contains the field definition of the NotifyMonitoringRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportRequest<'a, const N_VARIABLE_MONITORINGS: usize, const N_MONITORS: usize> {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i64,
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub monitor: Option<Vec<MonitoringDataType<'a, N_VARIABLE_MONITORINGS>, N_MONITORS>>,
}

/// Response to a NotifyMonitoringRequest message. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportResponse {}
