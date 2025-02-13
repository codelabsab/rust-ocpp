use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{charging_schedule_period::ChargingSchedulePeriodType, custom_data::CustomDataType};
use crate::v2_1::enumerations::ChargingRateUnitEnumType;

/// Composite Schedule structure defines a list of charging periods.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct CompositeScheduleType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The ID of the EVSE for which the schedule is requested.
    /// When evseid=0, the Charging Station calculated the expected consumption for the grid connection.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Duration of the schedule in seconds.
    pub duration: i32,

    /// Date and time at which the schedule becomes active.
    /// All time measurements within the schedule are relative to this timestamp.
    pub schedule_start: DateTime<Utc>,

    /// The unit of measure in which limits and setpoints are expressed.
    pub charging_rate_unit: ChargingRateUnitEnumType,

    /// List of charging periods describing the amount of power or current that can be delivered per time interval.
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
}
