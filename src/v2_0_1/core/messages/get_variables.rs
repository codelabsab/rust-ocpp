use crate::v2_0_1::core::datatypes::{
    get_variable_data_type::GetVariableDataType, get_variable_result_type::GetVariableResultType,
};

/// This contains the field definition of the GetVariablesRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest {
    pub get_variable_data: GetVariableDataType,
}

/// This contains the field definition of the GetVariablesResponse PDU sent by the CSMS to the Charging Station in response to GetVariablesRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesResponse {
    pub get_variable_result: GetVariableResultType,
}
