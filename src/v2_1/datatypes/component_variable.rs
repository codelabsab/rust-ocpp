use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{component::ComponentType, custom_data::CustomDataType, variable::VariableType};

/// Class to report components, variables and variable attributes and characteristics.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct ComponentVariableType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Component for which a report of Variable is requested.
    pub component: ComponentType,

    /// Optional. Variable(s) for which the report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType>,
}
