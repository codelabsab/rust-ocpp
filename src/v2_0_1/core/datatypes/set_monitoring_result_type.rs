use crate::v2_0_1::core::enumerations::{
    monitor_enum_type::MonitorEnumType,
    set_monitoring_status_enum_type::SetMonitoringStatusEnumType,
};

use super::{
    component_type::ComponentType, status_info_type::StatusInfoType, variable_type::VariableType,
};

/// Class to hold result of SetVariableMonitoring request.
/// SetMonitoringResultType is used by: SetVariableMonitoringResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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
