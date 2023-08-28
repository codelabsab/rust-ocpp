use super::component_type::ComponentType;
use super::variable_type::VariableType;
use crate::v2_0_1::enumerations::monitor_enum_type::MonitorEnumType;

/// Class to hold parameters of SetVariableMonitoring request.
/// SetMonitoringDataType is used by: SetVariableMonitoringRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringDataType<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,
    pub value: f64,
    #[serde(rename = "type")]
    pub kind: MonitorEnumType,
    pub severity: u8,
    #[serde(borrow)]
    pub component: ComponentType<'a>,
    pub variable: VariableType<'a>,
}
