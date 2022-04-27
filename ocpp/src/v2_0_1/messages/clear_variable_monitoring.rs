//! ClearedChargingLimit
use crate::v2_0_1::datatypes::clear_monitoring_result_type::ClearMonitoringResultType;

/// ClearVariableMonitoringRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringRequest {
    /// List of the monitors to be cleared, identified by there Id.
    pub id: Vec<i64>,
}

/// ClearVariableMonitoringResponse, sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringResponse {
    /// List of result statuses per monitor.
    pub clear_monitoring_result: Vec<ClearMonitoringResultType>,
}
