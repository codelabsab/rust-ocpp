use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{component::ComponentType, custom_data::CustomDataType, variable::VariableType};

/// Class to hold parameters of SetVariable request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableDataType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Component for which the variable is set.
    pub component: ComponentType,

    /// Required. Variable which holds the attribute value.
    pub variable: VariableType,

    /// Required. Value to be assigned to attribute of variable.
    #[validate(length(max = 1000))]
    pub attribute_value: String,

    /// Optional. Type of attribute that is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub attribute_type: Option<String>,
}
