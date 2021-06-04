use super::{
    component_type::ComponentType, variable_monitoring_type::VariableMonitoringType,
    variable_type::VariableType,
};

/// Class to hold parameters of SetVariableMonitoring request.
/// MonitoringDataType is used by: NotifyMonitoringReportRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    pub variable_monitoring: VariableMonitoringType,
}
