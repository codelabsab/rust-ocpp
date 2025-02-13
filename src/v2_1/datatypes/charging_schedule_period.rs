use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// Charging schedule period structure defines a time period in a charging schedule.
/// It is used in: CompositeScheduleType and in ChargingScheduleType.
/// When used in a NotifyEVChargingScheduleRequest only startPeriod, limit, limit_L2, limit_L3 are relevant.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriodType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Start of the period, in seconds from the start of schedule.
    /// The value of StartPeriod also defines the stop time of the previous period.
    pub start_period: i32,

    /// Optional only when not required by the operationMode, as in CentralSetpoint, ExternalSetpoint,
    /// ExternalLimits, LocalFrequency, LocalLoadBalancing.
    /// Charging rate limit during the schedule period, in the applicable chargingRateUnit.
    /// This SHOULD be a non-negative value; a negative value is only supported for backwards compatibility
    /// with older systems that use a negative value to specify a discharging limit.
    /// When using chargingRateUnit = 'W', this field represents the sum of the power of all phases,
    /// unless values are provided for L2 and L3, in which case this field represents phase L1.
    pub limit: f64,

    /// Charging rate limit on phase L2 in the applicable chargingRateUnit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l2: Option<f64>,

    /// Charging rate limit on phase L3 in the applicable chargingRateUnit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l3: Option<f64>,

    /// The number of phases that can be used for charging.
    /// If a number of phases is needed, numberPhases=3 will be assumed unless another number is given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i32>,

    /// Values: 1..3, Used if numberPhases=1 and if the EVSE is capable of switching the phase connected to the EV,
    /// i.e. ACPhaseSwitchingSupported is defined and true. It's not allowed unless both conditions above are true.
    /// If both conditions are true, and phaseToUse is omitted, the Charging Station / EVSE will make the selection on its own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_to_use: Option<i32>,
}
