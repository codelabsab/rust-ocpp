use super::ac_charging_parameters_type::ACChargingParametersType;
use super::dc_charging_parameters_type::DCChargingParametersType;
use crate::v2_0_1::enumerations::charging_limit_source_enum_type::ChargingLimitSourceEnumType;

/// ChargingNeedsType is used by: NotifyEVChargingNeedsRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingNeedsType {
    pub requested_energy_transfer: ChargingLimitSourceEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_charging_parameters: Option<ACChargingParametersType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_charging_parameters: Option<DCChargingParametersType>,
}
