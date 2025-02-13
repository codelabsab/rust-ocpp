use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Time-based price for a tariff.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffTimePriceType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Price per hour in the currency specified.
    pub price: f64,

    /// Optional. Price per hour in the currency specified, excluding taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_excl_tax: Option<f64>,

    /// Optional. Price per hour in the currency specified, including taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_incl_tax: Option<f64>,
}
