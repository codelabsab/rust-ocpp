//! ClearedChargingLimit
use crate::v2_0_1::datatypes::clear_monitoring_result_type::ClearMonitoringResultType;
use crate::Vec;

/// ClearVariableMonitoringRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringRequest<const N_IDS: usize> {
    /// List of the monitors to be cleared, identified by there Id.
    pub id: Vec<i64, N_IDS>,
}

/// ClearVariableMonitoringResponse, sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringResponse<'a, const N_CLEAR_MONITORING_RESULTS: usize> {
    /// List of result statuses per monitor.
    #[serde(borrow)]
    pub clear_monitoring_result: Vec<ClearMonitoringResultType<'a>, N_CLEAR_MONITORING_RESULTS>,
}
