use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{charging_schedule_period::ChargingSchedulePeriodType, CustomDataType},
    enumerations::ChargingRateUnitEnumType,
};

/// Charging schedule structure defines a list of charging periods, as used in: NotifyEVChargingScheduleRequest and ChargingProfileType.
/// When used in a NotifyEVChargingScheduleRequest only duration and chargingSchedulePeriod are relevant and chargingRateUnit must be 'W'.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Identifies the ChargingSchedule.
    pub id: i32,

    /// Starting point of an absolute schedule or recurring schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<DateTime<Utc>>,

    /// Duration of the charging schedule in seconds.
    /// If the duration is left empty, the last period will continue indefinitely or until end of the transaction
    /// in case startSchedule is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,

    /// The unit of measure in which limits and setpoints are expressed.
    pub charging_rate_unit: ChargingRateUnitEnumType,

    /// List of charging periods describing the amount of power or current that can be delivered per time interval.
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
}
