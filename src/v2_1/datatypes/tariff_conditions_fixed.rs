use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Fixed conditions for a tariff.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffConditionsFixedType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Fixed price per charging session.
    pub price: f64,

    /// Optional. Fixed price per charging session, excluding taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_excl_tax: Option<f64>,

    /// Optional. Fixed price per charging session, including taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_incl_tax: Option<f64>,
}
