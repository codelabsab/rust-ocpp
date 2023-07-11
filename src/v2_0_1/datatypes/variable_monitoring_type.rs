use crate::v2_0_1::enumerations::monitor_enum_type::MonitorEnumType;

/// A monitoring setting for a variable.
/// VariableMonitoringType is used by: NotifyMonitoringReportRequest.MonitoringDataType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VariableMonitoringType {
    pub id: i64,
    pub transaction: bool,
    pub value: f64,
    #[serde(rename = "type")]
    pub kind: MonitorEnumType,
    pub severity: u8,
}
