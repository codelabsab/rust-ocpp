use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Reference key to a component-variable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Name of the variable. Name should be taken from the list of standardized variable names whenever possible.
    #[validate(length(max = 50))]
    pub name: String,

    /// Required. Name of instance in case the variable exists as multiple instances.
    #[validate(length(max = 50))]
    pub instance: String,
}
