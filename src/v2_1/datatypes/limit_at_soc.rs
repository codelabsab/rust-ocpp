use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Limit at State of Charge settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LimitAtSoCType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// State of Charge at which power limit becomes active.
    pub soc: f64,

    /// Maximum power level when power limit is active.
    pub power_limit: f64,
}
