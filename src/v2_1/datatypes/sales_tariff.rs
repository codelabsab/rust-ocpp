use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, sales_tariff_entry::SalesTariffEntryType};

/// Sales tariff structure defining pricing information.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Unique identifier used to identify one or more tariffs.
    #[validate(length(max = 36))]
    pub id: String,

    /// Optional. A human readable title/description of the sales tariff e.g. for HMI display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 32))]
    pub sales_tariff_description: Option<String>,

    /// Optional. The number of time intervals supported for this sales tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_e_price_levels: Option<i32>,

    /// Required. List of sales tariff entries.
    #[validate(length(min = 1, max = 1024))]
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,
}
