use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Entry in the PriceLevelSchedule.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceLevelScheduleEntryType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Duration of the schedule entry in seconds.
    pub duration: i32,

    /// Required. Relative price level of this schedule entry.
    /// Values between -9 and 9 are allowed. Higher values indicate a more expensive price level.
    #[validate(range(min = -9, max = 9))]
    pub price_level: i8,
}
