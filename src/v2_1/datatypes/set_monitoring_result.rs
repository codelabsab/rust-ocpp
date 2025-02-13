use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType, status_info::StatusInfoType,
    variable::VariableType,
};
use crate::v2_1::enumerations::SetMonitoringStatusEnumType;

/// Class to hold result of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringResultType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Status indicating whether the Charging Station accepts the monitoring request.
    pub status: SetMonitoringStatusEnumType,

    /// Required. Component for which the monitoring status is returned.
    pub component: ComponentType,

    /// Required. Variable for which the monitoring status is returned.
    pub variable: VariableType,

    /// Required. Id of the monitor that was set.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
