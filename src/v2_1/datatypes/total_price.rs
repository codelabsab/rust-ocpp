use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Structure to report total price.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TotalPriceType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The total price (including taxes) in the specified currency.
    pub price: f64,

    /// Optional. The total price (excluding taxes) in the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_excl_tax: Option<f64>,

    /// Optional. The total price (including taxes) in the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_incl_tax: Option<f64>,
}
