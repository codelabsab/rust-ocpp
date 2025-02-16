use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{ComponentType, CustomDataType, StatusInfoType, VariableType};

/// The type of monitor, e.g. a threshold, delta or periodic monitor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MonitorEnumType {
    UpperThreshold,
    LowerThreshold,
    Delta,
    Periodic,
    PeriodicClockAligned,
    TargetDelta,
    TargetDeltaRelative,
}

/// Status returned in response to SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SetMonitoringStatusEnumType {
    Accepted,
    UnknownComponent,
    UnknownVariable,
    UnsupportedMonitorType,
    Rejected,
    Duplicate,
}

/// Class to hold parameters of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringDataType {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. An id SHALL only be given to replace an existing monitor.
    /// The Charging Station handles the generation of id's for new monitors.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub id: Option<i32>,

    /// Optional. Monitor only active when a transaction is ongoing on a component
    /// relevant to this transaction. Default = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,

    /// Required. Value for threshold or delta monitoring.
    /// For Periodic or PeriodicClockAligned this is the interval in seconds.
    pub value: f64,

    /// Required. The type of this monitor.
    #[serde(rename = "type")]
    pub monitor_type: MonitorEnumType,

    /// Required. The severity that will be assigned to an event that is triggered by this monitor.
    /// The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.
    #[validate(range(min = 0, max = 9))]
    pub severity: i32,

    /// Required. Component for which a variable is monitored.
    pub component: ComponentType,

    /// Required. Variable that is monitored.
    pub variable: VariableType,
}

/// Class to hold result of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringResultType {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. Id given to the VariableMonitor by the Charging Station.
    /// The Id is only returned when status is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub id: Option<i32>,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Required. The status of the monitoring setting.
    pub status: SetMonitoringStatusEnumType,

    /// Required. The type of this monitor.
    #[serde(rename = "type")]
    pub monitor_type: MonitorEnumType,

    /// Required. Component for which a variable is monitored.
    pub component: ComponentType,

    /// Required. Variable that is monitored.
    pub variable: VariableType,

    /// Required. The severity that will be assigned to an event that is triggered by this monitor.
    /// The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.
    #[validate(range(min = 0, max = 9))]
    pub severity: i32,
}

/// Request to set monitoring parameters for a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    /// Optional. Custom data specific to this class.
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
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of result statuses per monitoring setting.
    #[validate(length(min = 1), nested)]
    pub set_monitoring_result: Vec<SetMonitoringResultType>,
}
