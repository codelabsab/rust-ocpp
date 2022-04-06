use super::component_type::ComponentType;
use super::variable_type::VariableType;
use crate::v2_0_1::enumerations::attribute_enum_type::AttributeEnumType;

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
