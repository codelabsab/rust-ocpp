use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, tariff_conditions::TariffConditionsType,
    tariff_energy::TariffEnergyType, tariff_fixed::TariffFixedType, tariff_time::TariffTimeType,
};

/// A tariff defines price and price rules for charging sessions.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Unique identifier used to identify one or more tariffs.
    #[validate(length(max = 36))]
    pub id: String,

    /// Optional. Currency of the price in ISO 4217 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 3))]
    pub currency: Option<String>,

    /// Optional. Language identifier of the language used in the description fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 8))]
    pub language: Option<String>,

    /// Optional. Display text of the tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 100))]
    pub display_text: Option<String>,

    /// Required. Conditions under which this tariff applies.
    pub conditions: TariffConditionsType,

    /// Optional. Fixed costs of the tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<TariffFixedType>,

    /// Optional. Energy costs of the tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<TariffEnergyType>,

    /// Optional. Time costs of the tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<TariffTimeType>,
}
