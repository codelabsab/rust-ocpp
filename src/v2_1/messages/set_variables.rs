use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, SetVariableDataType, SetVariableResultType};

/// Request to set variables in a charging station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesRequest {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of settings to set in components.
    #[validate(length(min = 1), nested)]
    pub set_variable_data: Vec<SetVariableDataType>,
}

/// Response to SetVariables request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of result statuses per settings.
    #[validate(length(min = 1), nested)]
    pub set_variable_result: Vec<SetVariableResultType>,
}
