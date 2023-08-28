//! GetVariables

use crate::v2_0_1::datatypes::get_variable_data_type::GetVariableDataType;
use crate::v2_0_1::datatypes::get_variable_result_type::GetVariableResultType;
use crate::Vec;

/// GetVariablesRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest<'a, const N_VARIABLES: usize = { crate::N_VARIABLES }> {
    /// List of requested variables.
    #[serde(borrow)]
    pub get_variable_data: Vec<GetVariableDataType<'a>, N_VARIABLES>,
}

/// GetVariablesResponse, sent by the CSMS to the Charging Station in response to GetVariablesRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesResponse<'a, const N_VARIABLES: usize> {
    /// List of requested variables and their values.
    #[serde(borrow)]
    pub get_variable_result: Vec<GetVariableResultType<'a>, N_VARIABLES>,
}
