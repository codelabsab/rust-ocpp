use chrono::DateTime;
use chrono::Utc;

use super::charging_schedule_period_type::ChargingSchedulePeriodType;
use super::sales_tariff_type::SalesTariffType;
use crate::v2_0_1::enumerations::charging_rate_unit_enum_type::ChargingRateUnitEnumType;

/// Charging schedule structure defines a list of charging periods, as used in: GetCompositeSchedule.conf and ChargingProfile.
/// ChargingScheduleType is used by: Common:ChargingProfileType , NotifyChargingLimitRequest, NotifyEVChargingScheduleRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleType {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    pub charging_rate_unit: ChargingRateUnitEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_rate: Option<f64>,
    pub charging_schedule_period: ChargingSchedulePeriodType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff: Option<SalesTariffType>,
}
