use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, SetMonitoringDataType, SetMonitoringResultType};

/// Request to set monitoring parameters for a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    /// Custom data from the CSMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of monitoring settings to configure.
    #[validate(length(min = 1), nested)]
    pub set_monitoring_data: Vec<SetMonitoringDataType>,
}

/// Response to SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringResponse {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of result statuses per monitoring setting.
    #[validate(length(min = 1), nested)]
    pub set_monitoring_result: Vec<SetMonitoringResultType>,
}
