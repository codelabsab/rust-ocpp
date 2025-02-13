use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Parameters for the EnterService DER control function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnterServiceType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Enter service voltage high
    pub high_voltage: f64,

    /// Enter service voltage low
    pub low_voltage: f64,

    /// Enter service frequency high
    pub high_freq: f64,

    /// Enter service frequency low
    pub low_freq: f64,

    /// Enter service delay
    pub delay: f64,

    /// Enter service randomized delay
    pub random_delay: f64,

    /// Enter service ramp rate in seconds
    pub ramp_rate: f64,
}
