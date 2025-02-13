use crate::v2_1::{
    datatypes::{
        ac_charging_parameters::ACChargingParametersType, custom_data::CustomDataType,
        dc_charging_parameters::DCChargingParametersType,
    },
    enumerations::EnergyTransferModeEnumType,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the charging needs of an EV.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingNeedsType {
    /// Mode of energy transfer requested by the EV.
    pub requested_energy_transfer: EnergyTransferModeEnumType,

    /// Estimated departure time of the EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<DateTime<Utc>>,

    /// EV AC charging parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_charging_parameters: Option<ACChargingParametersType>,

    /// EV DC charging parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_charging_parameters: Option<DCChargingParametersType>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
