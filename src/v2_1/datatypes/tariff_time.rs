use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tariff_time_price::TariffTimePriceType};

/// Time tariff structure defining time-based costs.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffTimeType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Time-based price per hour.
    pub time_price: TariffTimePriceType,

    /// Optional. Maximum duration in seconds that can be charged under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<i32>,

    /// Optional. Minimum duration in seconds that must be charged under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration: Option<i32>,
}
