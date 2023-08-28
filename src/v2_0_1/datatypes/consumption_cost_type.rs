use super::cost_type::CostType;
use crate::Vec;

/// ConsumptionCostType is used by: Common:SalesTariffEntryType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionCostType<const N_COSTS: usize> {
    pub start_value: i64,
    pub cost: Vec<CostType, N_COSTS>,
}
