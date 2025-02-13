use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Energy price for a tariff.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffEnergyPriceType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Price per kWh.
    pub price: f64,

    /// Optional. Price per kWh, excluding taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_excl_tax: Option<f64>,

    /// Optional. Price per kWh, including taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_incl_tax: Option<f64>,
}
