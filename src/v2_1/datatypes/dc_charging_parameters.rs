use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// EV DC charging parameters for ISO 15118-2
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DCChargingParametersType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Maximum current (in A) supported by the electric vehicle. Includes cable capacity.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType:EVMaximumCurrentLimit
    pub ev_max_current: f64,

    /// Maximum voltage supported by the electric vehicle.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVMaximumVoltageLimit
    pub ev_max_voltage: f64,

    /// Maximum power (in W) supported by the electric vehicle. Required for DC charging.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVMaximumPowerLimit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_power: Option<f64>,

    /// Capacity of the electric vehicle battery (in Wh).
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVEnergyCapacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_energy_capacity: Option<f64>,

    /// Amount of energy requested (in Wh). This includes energy required for preconditioning.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVEnergyRequest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_amount: Option<f64>,
}
