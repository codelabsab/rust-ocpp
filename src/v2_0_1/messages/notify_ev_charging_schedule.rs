use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::charging_schedule_type::ChargingScheduleType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::generic_status_enum_type::GenericStatusEnumType;

/// The Charging Station uses this message to communicate the charging needs as calculated by the EV to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleRequest<'a, const N_SALES_TARIFF_ENTRIES: usize, const N_TARIFF_CONSUMPTION_COSTS: usize, const N_COSTS_PER_TARIFF_CONS_COST: usize, const N_CHARGING_SCHEDULE_PERIODS: usize> {
    pub time_base: DateTime<Utc>,
    pub evse_id: i64,
    #[serde(borrow)]
    pub charging_schedule: ChargingScheduleType<'a, N_SALES_TARIFF_ENTRIES, N_TARIFF_CONSUMPTION_COSTS, N_COSTS_PER_TARIFF_CONS_COST, N_CHARGING_SCHEDULE_PERIODS>,
}

/// Response to a NotifyEVChargingScheduleRequest message.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleResponse<'a> {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub status_info: Option<StatusInfoType<'a>>,
}
