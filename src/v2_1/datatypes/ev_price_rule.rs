use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Price rule for a power range.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVPriceRuleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Energy fee in the currency specified in EVAbsolutePriceSchedule.
    pub energy_fee: f64,

    /// Start of the power range in Watts (W).
    pub power_range_start: f64,
}
