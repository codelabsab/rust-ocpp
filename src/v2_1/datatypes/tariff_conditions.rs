use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, tariff_conditions_fixed::TariffConditionsFixedType};
use crate::v2_1::enumerations::EnergyTransferModeEnumType;

/// Conditions for a tariff.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffConditionsType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Mode of energy transfer for which this tariff applies.
    pub energy_transfer_mode: EnergyTransferModeEnumType,

    /// Optional. Fixed conditions for this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<TariffConditionsFixedType>,

    /// Optional. Maximum power in kW that can be delivered under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_power: Option<f64>,

    /// Optional. Minimum power in kW that must be delivered under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_power: Option<f64>,

    /// Optional. Maximum duration in seconds that a charging session can last under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<i32>,

    /// Optional. Minimum duration in seconds that a charging session must last under this tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration: Option<i32>,
}
