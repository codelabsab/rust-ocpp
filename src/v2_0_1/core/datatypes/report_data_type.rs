use super::{
    component_type::ComponentType, variable_attribute_type::VariableAttributeType,
    variable_characteristics_type::VariableCharacteristicsType, variable_type::VariableType,
};

/// Class to report components, variables and variable attributes and characteristics.
/// ReportDataType is used by: NotifyReportRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    pub variable_attribute: VariableAttributeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_characteristics: Option<VariableCharacteristicsType>,
}
