use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Rule that describes the pricing of overstaying.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OverstayRuleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Duration in seconds after which the overstay rule becomes active.
    #[validate(range(min = 0, max = 86400))]
    pub overstay_time_threshold: i32,

    /// Required. Factor by which the price is multiplied when the overstay rule is active.
    #[validate(range(min = 0.0))]
    pub overstay_fee_threshold: f32,
}
