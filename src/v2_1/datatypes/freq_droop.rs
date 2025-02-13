use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Frequency droop settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FreqDroopType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Frequency droop slope in percent active power per Hz.
    pub droop: f64,

    /// Frequency offset in Hz.
    pub offset: f64,

    /// Frequency deadband in Hz.
    pub deadband: f64,

    /// Response time in seconds.
    pub response_time: f64,
}
