use super::cost_type::CostType;

/// ConsumptionCostType is used by: Common:SalesTariffEntryType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionCostType {
    pub start_value: i64,
    pub cost: Vec<CostType>,
}
