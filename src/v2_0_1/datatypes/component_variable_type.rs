//! Struct to report components, variables and variable attributes and characteristics.
use super::component_type::ComponentType;
use super::variable_type::VariableType;

/// ComponentVariableType is used by:
/// [GetMonitoringReportRequest](`crate::v2_0_1::messages::get_monitoring_report::GetMonitoringReportRequest`)
/// [GetReportRequest](`crate::v2_0_1::messages::get_report::GetReportRequest`)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentVariableType<'a> {
    /// Required. Component for which a report of Variable is requested.
    #[serde(borrow)]
    pub component: ComponentType<'a>,
    /// Optional. Variable(s) for which the report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType<'a>>,
}
