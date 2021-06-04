use crate::v2_0_1::core::enumerations::{
    attribute_enum_type::AttributeEnumType,
    set_variable_status_enum_type::SetVariableStatusEnumType,
};

use super::{
    component_type::ComponentType, status_info_type::StatusInfoType, variable_type::VariableType,
};

/// SetVariableResultType is used by: SetVariablesResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResultType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    pub attribute_value: SetVariableStatusEnumType,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
}
