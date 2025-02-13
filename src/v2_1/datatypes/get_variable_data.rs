use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{component::ComponentType, custom_data::CustomDataType, variable::VariableType};

/// Class to hold parameters for GetVariables request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableDataType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Component for which the Variable is requested.
    pub component: ComponentType,

    /// Required. Variable for which the attribute value is requested.
    pub variable: VariableType,

    /// Optional. If the variable is attribute-based, this field specifies the attribute type for which the value is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub attribute_type: Option<String>,
}
