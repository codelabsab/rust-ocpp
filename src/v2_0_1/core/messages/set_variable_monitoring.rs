use crate::v2_0_1::core::datatypes::{
    set_monitoring_data_type::SetMonitoringDataType,
    set_monitoring_result_type::SetMonitoringResultType,
};

/// This contains the field definition of the SetVariableMonitoringRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    pub set_monitoring_data: SetMonitoringDataType,
}

/// This contains the field definition of the SetVariableMonitoringResponse PDU sent by the Charging Station to the CSMS in response to a SetVariableMonitoringRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringResponse {
    pub set_monitoring_result: SetMonitoringResultType,
}
