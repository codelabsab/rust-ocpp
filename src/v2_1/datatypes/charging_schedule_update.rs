use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Updates to a ChargingSchedulePeriodType for dynamic charging profiles.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleUpdateType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Optional only when not required by the _operationMode_, as in CentralSetpoint, ExternalSetpoint, ExternalLimits, LocalFrequency,  LocalLoadBalancing.
    /// Charging rate limit during the schedule period, in the applicable _chargingRateUnit_.
    /// This SHOULD be a non-negative value; a negative value is only supported for backwards compatibility with older systems that use a negative value to specify a discharging limit.
    /// For AC this field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f32>,

    /// *(2.1)* Charging rate limit on phase L2 in the applicable _chargingRateUnit_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l2: Option<f32>,

    /// *(2.1)* Charging rate limit on phase L3 in the applicable _chargingRateUnit_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l3: Option<f32>,
}
