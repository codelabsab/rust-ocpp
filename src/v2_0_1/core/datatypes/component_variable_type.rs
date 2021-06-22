use super::{component_type::ComponentType, variable_type::VariableType};

/// Class to report components, variables and variable attributes and characteristics.
/// ComponentVariableType is used by: GetMonitoringReportRequest , GetReportRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentVariableType {
    pub component: ComponentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType>,
}
