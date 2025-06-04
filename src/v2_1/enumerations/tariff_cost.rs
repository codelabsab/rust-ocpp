use serde::{Deserialize, Serialize};

/// Type of cost: normal or the minimum or maximum cost.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TariffCostEnumType {
    /// Normal cost based on the tariff.
    #[serde(rename = "NormalCost")]
    NormalCost,

    /// Minimum cost that will be billed.
    #[serde(rename = "MinCost")]
    MinCost,

    /// Maximum cost that will be billed.
    #[serde(rename = "MaxCost")]
    MaxCost,
}
