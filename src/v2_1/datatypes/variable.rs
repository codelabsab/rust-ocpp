use super::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};

/// Reference key to a component-variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableType {
    /// Name of the variable. Name should be taken from the list of standardized variable names whenever possible.
    /// Case Insensitive. Strongly advised to use Camel Case.
    pub name: String,

    /// Name of instance in case the variable exists as multiple instances.
    /// Case Insensitive. Strongly advised to use Camel Case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
