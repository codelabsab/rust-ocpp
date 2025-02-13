use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    charging_period::ChargingPeriodType, custom_data::CustomDataType, total_cost::TotalCostType,
    total_usage::TotalUsageType,
};

/// CostDetailsType contains the cost as calculated by Charging Station based on provided TariffType.
/// NOTE: Reservation is not shown as a chargingPeriod, because it took place outside of the transaction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CostDetailsType {
    /// List of charging periods that make up this transaction.
    pub charging_periods: Vec<ChargingPeriodType>,

    /// Total cost of this transaction, including taxes.
    pub total_cost: TotalCostType,

    /// Total usage of energy and time during this transaction.
    pub total_usage: TotalUsageType,

    /// If set to true, then Charging Station has failed to calculate the cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_to_calculate: Option<bool>,

    /// Optional human-readable reason text in case of failure to calculate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 500))]
    pub failure_reason: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
