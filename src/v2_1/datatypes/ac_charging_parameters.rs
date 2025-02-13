use crate::v2_1::datatypes::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};

/// EV AC charging parameters for ISO 15118-2
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ACChargingParametersType {
    /// Amount of energy requested (in Wh). This includes energy required for preconditioning.
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EAmount
    /// *ISO 15118-20*: Dynamic/Scheduled_SEReqControlModeType: EVTargetEnergyRequest
    pub energy_amount: f64,

    /// Minimum current (amps) supported by the electric vehicle (per phase).
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EVMinCurrent
    pub ev_min_current: f64,

    /// Maximum current (amps) supported by the electric vehicle (per phase). Includes cable capacity.
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EVMaxCurrent
    pub ev_max_current: f64,

    /// Maximum voltage supported by the electric vehicle.
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EVMaxVoltage
    pub ev_max_voltage: f64,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
