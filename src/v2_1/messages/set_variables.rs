use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    custom_data::CustomDataType, set_variable_data::SetVariableDataType,
    set_variable_result::SetVariableResultType,
};

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

impl SetVariablesRequest {
    /// Creates a new `SetVariablesRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `set_variable_data` - List of settings to set in components
    ///
    /// # Returns
    ///
    /// A new instance of `SetVariablesRequest` with optional fields set to `None`
    pub fn new(set_variable_data: Vec<SetVariableDataType>) -> Self {
        Self {
            custom_data: None,
            set_variable_data,
        }
    }
}

impl SetVariablesResponse {
    /// Creates a new `SetVariablesResponse` with required fields.
    ///
    /// # Arguments
    ///
    /// * `set_variable_result` - List of result statuses per settings
    ///
    /// # Returns
    ///
    /// A new instance of `SetVariablesResponse` with optional fields set to `None`
    pub fn new(set_variable_result: Vec<SetVariableResultType>) -> Self {
        Self {
            custom_data: None,
            set_variable_result,
        }
    }
}
