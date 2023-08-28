use crate::v2_0_1::datatypes::set_monitoring_data_type::SetMonitoringDataType;
use crate::v2_0_1::datatypes::set_monitoring_result_type::SetMonitoringResultType;
use crate::Vec;

/// This contains the field definition of the SetVariableMonitoringRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest<'a, const N_MONITORING_DATA: usize> {
    #[serde(borrow)]
    pub set_monitoring_data: Vec<SetMonitoringDataType<'a>, N_MONITORING_DATA>,
}

/// This contains the field definition of the SetVariableMonitoringResponse PDU sent by the Charging Station to the CSMS in response to a SetVariableMonitoringRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringResponse<'a, const N_MONITORING_RESULTS: usize> {
    #[serde(borrow)]
    pub set_monitoring_result: Vec<SetMonitoringResultType<'a>, N_MONITORING_RESULTS>,
}
