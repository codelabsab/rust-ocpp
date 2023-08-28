use super::component_type::ComponentType;
use super::variable_attribute_type::VariableAttributeType;
use super::variable_characteristics_type::VariableCharacteristicsType;
use super::variable_type::VariableType;
use crate::Vec;

/// Class to report components, variables and variable attributes and characteristics.
/// ReportDataType is used by: NotifyReportRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataType<'a, const N_VARIABLE_ATTRIBUTES: usize = { crate::N_VARIABLE_ATTRIBUTES }> {
    #[serde(borrow)]
    pub component: ComponentType<'a>,
    pub variable: VariableType<'a>,
    pub variable_attribute: Vec<VariableAttributeType<'a>, N_VARIABLE_ATTRIBUTES>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_characteristics: Option<VariableCharacteristicsType<'a>>,
}
