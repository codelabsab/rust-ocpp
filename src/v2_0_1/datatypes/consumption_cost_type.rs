use super::cost_type::CostType;

/// ConsumptionCostType is used by: Common:SalesTariffEntryType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionCostType {
    pub start_value: i32,
    pub cost: Vec<CostType>,
}
