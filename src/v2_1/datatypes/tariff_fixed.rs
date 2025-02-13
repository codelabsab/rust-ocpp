use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tariff_fixed_price::TariffFixedPriceType};

/// Fixed tariff structure defining fixed costs.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffFixedType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Fixed price per charging session.
    pub fixed_price: TariffFixedPriceType,
}
