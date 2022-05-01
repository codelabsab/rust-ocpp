use chrono::DateTime;
use chrono::Utc;

use super::charging_schedule_period_type::ChargingSchedulePeriodType;
use crate::v2_0_1::enumerations::charging_rate_unit_enum_type::ChargingRateUnitEnumType;

/// CompositeScheduleType is used by: GetCompositeScheduleResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompositeScheduleType {
    pub evse_id: i64,
    pub duration: i64,
    pub schedule_start: DateTime<Utc>,
    pub charging_rate_unit: ChargingRateUnitEnumType,
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
}
