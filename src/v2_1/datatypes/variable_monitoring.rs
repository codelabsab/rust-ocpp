use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::MonitorEnumType;

/// A monitoring setting for a variable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableMonitoringType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. ID of the monitor.
    pub id: i32,

    /// Required. Monitor type of the variable.
    pub r#type: MonitorEnumType,

    /// Required. Value for threshold or delta of the monitor.
    pub value: f64,

    /// Optional. The severity that will be assigned to an event that is triggered by this monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
}
