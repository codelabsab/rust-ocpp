use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tariff_energy_price::TariffEnergyPriceType};

/// Energy tariff structure defining energy costs.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffEnergyType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Energy price per kWh.
    pub energy_price: TariffEnergyPriceType,

    /// Optional. Maximum energy in kWh that can be charged under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_energy: Option<f64>,

    /// Optional. Minimum energy in kWh that must be charged under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_energy: Option<f64>,
}
