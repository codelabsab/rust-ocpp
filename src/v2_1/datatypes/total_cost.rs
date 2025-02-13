use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, price::PriceType, total_price::TotalPriceType};
use crate::v2_1::enumerations::TariffCostEnumType;

/// Structure to report costs.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TotalCostType {
    /// Currency of the costs in ISO 4217 Code.
    #[validate(length(max = 3))]
    pub currency: String,

    /// Type of cost: normal or the minimum or maximum cost.
    pub type_of_cost: TariffCostEnumType,

    /// Fixed costs per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<PriceType>,

    /// Energy costs per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<PriceType>,

    /// Time cost per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_time: Option<PriceType>,

    /// Idle time cost per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_time: Option<PriceType>,

    /// Reservation time cost per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_time: Option<PriceType>,

    /// Fixed reservation costs per transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_fixed: Option<PriceType>,

    /// Total cost including and/or excluding tax.
    pub total: TotalPriceType,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The total cost (including taxes) of the transaction in the specified currency.
    pub cost: f64,

    /// Optional. The total cost (excluding taxes) of the transaction in the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_excl_tax: Option<f64>,

    /// Optional. The total cost (including taxes) of the transaction in the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_incl_tax: Option<f64>,
}
