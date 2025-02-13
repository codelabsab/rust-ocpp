use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Parameters for voltage-based control.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VoltageParamsType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Voltage at which to start charging, in Volts.
    pub voltage_start: f64,

    /// Required. Voltage at which to stop charging, in Volts.
    pub voltage_stop: f64,

    /// Required. Voltage at which to start discharging, in Volts.
    pub voltage_discharge_start: f64,

    /// Required. Voltage at which to stop discharging, in Volts.
    pub voltage_discharge_stop: f64,
}
