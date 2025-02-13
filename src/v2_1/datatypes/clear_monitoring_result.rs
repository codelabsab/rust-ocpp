use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use super::status_info::StatusInfoType;
use crate::v2_1::enumerations::ClearMonitoringStatusEnumType;

/// Result of the clear request for this monitor, identified by its Id.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearMonitoringResultType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Result of the clear request for this monitor, identified by its Id.
    pub status: ClearMonitoringStatusEnumType,

    /// Required. Id of the monitor of which a clear was requested.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
