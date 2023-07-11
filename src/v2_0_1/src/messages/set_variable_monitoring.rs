use crate::datatypes::set_monitoring_data_type::SetMonitoringDataType;
use crate::datatypes::set_monitoring_result_type::SetMonitoringResultType;

/// This contains the field definition of the SetVariableMonitoringRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    pub set_monitoring_data: Vec<SetMonitoringDataType>,
}

/// This contains the field definition of the SetVariableMonitoringResponse PDU sent by the Charging Station to the CSMS in response to a SetVariableMonitoringRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringResponse {
    pub set_monitoring_result: Vec<SetMonitoringResultType>,
}
