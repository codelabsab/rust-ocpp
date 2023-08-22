use super::component_type::ComponentType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::datatypes::variable_type::VariableType;
use crate::v2_0_1::enumerations::attribute_enum_type::AttributeEnumType;
use crate::v2_0_1::enumerations::get_variable_status_enum_type::GetVariableStatusEnumType;
use validator::Validate;

/// Class to hold parameters for GetVariables request.
/// GetVariableDataType is used by: GetVariablesRequest
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableResultType<'a> {
    pub attribute_status: GetVariableStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 2500))]
    pub attribute_value: Option<&'a str>,
    pub component: ComponentType<'a>,
    pub variable: VariableType<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType<'a>>,
}
