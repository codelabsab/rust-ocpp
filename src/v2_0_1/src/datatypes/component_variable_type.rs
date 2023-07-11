//! Struct to report components, variables and variable attributes and characteristics.
use super::component_type::ComponentType;
use super::variable_type::VariableType;

/// ComponentVariableType is used by:
/// [GetMonitoringReportRequest](`crate::messages::get_monitoring_report::GetMonitoringReportRequest`)
/// [GetReportRequest](`crate::messages::get_report::GetReportRequest`)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentVariableType {
    /// Required. Component for which a report of Variable is requested.
    pub component: ComponentType,
    /// Optional. Variable(s) for which the report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType>,
}
