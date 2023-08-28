use super::consumption_cost_type::ConsumptionCostType;
use super::relative_time_interval_type::RelativeTimeIntervalType;
use crate::Vec;

/// SalesTariffEntryType is used by: Common:SalesTariffType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffEntryType<const N_CONSUMPTION_COSTS: usize = { crate::N_CONSUMPTION_COSTS }, const N_COSTS_PER_CONS_COST: usize = { crate::N_COSTS_PER_CONS_COST }> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_price_level: Option<u64>,
    pub relative_time_interval: RelativeTimeIntervalType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_cost: Option<Vec<ConsumptionCostType<N_COSTS_PER_CONS_COST>, N_CONSUMPTION_COSTS>>,
}
