use super::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::clear_monitoring_status_enum_type::ClearMonitoringStatusEnumType;

/// ClearMonitoringResultType is used by: ClearVariableMonitoringResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearMonitoringResultType {
    /// Required. Result of the clear request for this monitor, identified by its Id.
    pub status: ClearMonitoringStatusEnumType,
    /// Required. Id of the monitor of which a clear was requested.
    pub id: i64,
    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
