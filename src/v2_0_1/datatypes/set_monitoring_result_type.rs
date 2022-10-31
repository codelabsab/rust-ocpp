use super::component_type::ComponentType;
use super::status_info_type::StatusInfoType;
use super::variable_type::VariableType;
use crate::v2_0_1::enumerations::monitor_enum_type::MonitorEnumType;
use crate::v2_0_1::enumerations::set_monitoring_status_enum_type::SetMonitoringStatusEnumType;

/// Class to hold result of SetVariableMonitoring request.
/// SetMonitoringResultType is used by: SetVariableMonitoringResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringResultType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub status: SetMonitoringStatusEnumType,
    #[serde(rename = "type")]
    pub kind: MonitorEnumType,
    pub severity: u8,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
