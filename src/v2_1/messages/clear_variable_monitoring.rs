use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, ClearMonitoringResultType};

/// Request to clear variable monitoring settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringRequest {
    /// Required. List of the monitors to be cleared, identified by their Id.
    #[validate(length(min = 1))]
    pub id: Vec<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a ClearVariableMonitoringRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringResponse {
    /// Required. List of results for each monitor that was cleared or attempted to be cleared.
    #[validate(length(min = 1))]
    pub clear_monitoring_result: Vec<ClearMonitoringResultType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
