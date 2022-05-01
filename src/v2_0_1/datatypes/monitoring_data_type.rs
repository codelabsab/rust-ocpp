use super::component_type::ComponentType;
use super::variable_monitoring_type::VariableMonitoringType;
use super::variable_type::VariableType;

/// Class to hold parameters of SetVariableMonitoring request.
/// MonitoringDataType is used by: NotifyMonitoringReportRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    pub variable_monitoring: Vec<VariableMonitoringType>,
}
