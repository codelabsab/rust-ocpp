use crate::v2_0_1::core::enumerations::{
    attribute_enum_type::AttributeEnumType,
    get_variable_status_enum_type::GetVariableStatusEnumType,
};

use super::component_type::ComponentType;

/// Class to hold parameters for GetVariables request.
/// GetVariableDataType is used by: GetVariablesRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableResultType {
    pub attribute_status: GetVariableStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    pub component: ComponentType,
}
