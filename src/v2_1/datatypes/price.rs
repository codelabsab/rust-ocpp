use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tax_rate::TaxRateType};

/// Price with and without tax. At least one of exclTax, inclTax must be present.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceType {
    /// Price/cost excluding tax. Can be absent if inclTax is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excl_tax: Option<f64>,

    /// Price/cost including tax. Can be absent if exclTax is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incl_tax: Option<f64>,

    /// List of tax rates used to calculate tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRateType>>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
