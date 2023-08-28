use super::component_type::ComponentType;
use super::variable_monitoring_type::VariableMonitoringType;
use super::variable_type::VariableType;
use crate::Vec;

/// Class to hold parameters of SetVariableMonitoring request.
/// MonitoringDataType is used by: NotifyMonitoringReportRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType<'a, const N_VARIABLE_MONITORINGS: usize> {
    #[serde(borrow)]
    pub component: ComponentType<'a>,
    pub variable: VariableType<'a>,
    pub variable_monitoring: Vec<VariableMonitoringType, N_VARIABLE_MONITORINGS>,
}
