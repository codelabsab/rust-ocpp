//! GetVariables

use crate::v2_0_1::datatypes::get_variable_data_type::GetVariableDataType;
use crate::v2_0_1::datatypes::get_variable_result_type::GetVariableResultType;

/// GetVariablesRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest<'a> {
    /// List of requested variables.
    #[serde(borrow)]
    pub get_variable_data: Vec<GetVariableDataType<'a>>,
}

/// GetVariablesResponse, sent by the CSMS to the Charging Station in response to GetVariablesRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesResponse<'a> {
    /// List of requested variables and their values.
    #[serde(borrow)]
    pub get_variable_result: Vec<GetVariableResultType<'a>>,
}
