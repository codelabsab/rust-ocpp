use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    consumption_cost::ConsumptionCostType, custom_data::CustomDataType,
    relative_time_interval::RelativeTimeIntervalType,
};

/// Sales tariff entry details.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffEntryType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Time and date at which the tariff becomes valid.
    pub relative_time_interval: RelativeTimeIntervalType,

    /// Optional. Consumption cost per time interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_cost: Option<Vec<ConsumptionCostType>>,

    /// Optional. A human readable tariff description.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub e_price_level: Option<i32>,
}
