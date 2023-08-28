use crate::v2_0_1::datatypes::set_variable_data_type::SetVariableDataType;
use crate::v2_0_1::datatypes::set_variable_result_type::SetVariableResultType;
use crate::Vec;

/// This contains the field definition of the SetVariablesRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesRequest<'a, const N_VARIABLE_DATA: usize> {
    #[serde(borrow)]
    pub set_variable_data: Vec<SetVariableDataType<'a>, N_VARIABLE_DATA>,
}

/// This contains the field definition of the SetVariablesResponse PDU sent by the Charging Station to the CSMS in response to a SetVariablesRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesResponse<'a, const N_VARIABLE_RESULTS: usize> {
    #[serde(borrow)]
    pub set_variable_result: Vec<SetVariableResultType<'a>, N_VARIABLE_RESULTS>,
}
