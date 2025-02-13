use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::EnergyTransferModeEnumType;

/// Rule that describes the price of charging.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceRuleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Mode of energy transfer for which this price rule applies.
    pub energy_transfer_mode: EnergyTransferModeEnumType,

    /// Required. Price per energy unit, excluding taxes, in the specified currency.
    pub energy_fee: f64,

    /// Required. Price per time unit, excluding taxes, in the specified currency.
    pub time_fee: f64,

    /// Required. Price for parking time, excluding taxes, in the specified currency.
    /// Only applicable when the EV is not charging.
    pub parking_fee: f64,

    /// Required. Minimum duration in seconds that a charging session SHALL last to benefit from this price rule.
    #[validate(range(min = 0))]
    pub minimum_duration: i32,

    /// Required. Maximum duration in seconds that a charging session can last under this price rule.
    #[validate(range(min = 0))]
    pub maximum_duration: i32,

    /// Required. Maximum power in kW that can be delivered under this price rule.
    pub maximum_power: f64,

    /// Required. Minimum power in kW that must be delivered under this price rule.
    pub minimum_power: f64,
}
