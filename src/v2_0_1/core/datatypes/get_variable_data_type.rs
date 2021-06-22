use crate::v2_0_1::core::enumerations::attribute_enum_type::AttributeEnumType;

use super::{component_type::ComponentType, variable_type::VariableType};

/// Class to hold parameters for GetVariables request.
/// GetVariableDataType is used by: GetVariablesRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableDataType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    pub component: ComponentType,
    pub variable: VariableType,
}
