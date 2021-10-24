use super::consumption_cost_type::ConsumptionCostType;
use super::relative_time_interval_type::RelativeTimeIntervalType;

/// SalesTariffEntryType is used by: Common:SalesTariffType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffEntryType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_price_level: Option<u64>,
    pub relative_time_interval: RelativeTimeIntervalType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_cost: Option<ConsumptionCostType>,
}
