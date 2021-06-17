use crate::v2_0_1::core::datatypes::clear_monitoring_result_type::ClearMonitoringResultType;

/// This contains the field definition of the ClearVariableMonitoringRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringRequest {
    pub id: i64,
}

/// This contains the field definition of the ClearVariableMonitoringResponse PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringResponse {
    pub clear_monitoring_result: ClearMonitoringResultType,
}
