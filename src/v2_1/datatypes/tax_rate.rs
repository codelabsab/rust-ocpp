use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Tax rate structure defining tax rate details.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaxRateType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Tax rate in percent.
    pub tax_rate: f64,

    /// Required. Tax rate name.
    #[validate(length(max = 20))]
    pub tax_rate_name: String,
}
