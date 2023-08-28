use chrono::{DateTime, Utc};

use crate::v1_6::types::{ChargingRateUnitType, ChargingSchedule, GetCompositeScheduleStatus};

/// This contains the field definition of the GetCompositeSchedule.req PDU sent by the Central System to theCharge Point. See also Get Composite Schedule
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    /// Required. The ID of the Connector for which the schedule is requested. When ConnectorId=0, the Charge Point will calculate the expected consumption for the grid connection.
    pub connector_id: i64,
    /// Required. Time in seconds. length of requested schedule
    pub duration: i64,
    /// Optional. Can be used to force a power or current profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_rate_unit: Option<ChargingRateUnitType>,
}

/// This contains the field definition of the GetCompositeSchedule.conf PDU sent by the Charge Point to the Central System in response to a GetCompositeSchedule.req PDU. See also Get Composite Schedule
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleResponse<const N_CHARGING_SCHEDULE_PERIODS: usize = { crate::N_CHARGING_SCHEDULE_PERIODS }> {
    /// Required. Status of the request. The Charge Point will indicate if it was able to process the request
    pub status: GetCompositeScheduleStatus,
    /// Optional. The charging schedule contained in this notification applies to a Connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i64>,
    /// Optional. Time. Periods contained in the charging profile are relative to this point in time. If status is "Rejected", this field may be absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_start: Option<DateTime<Utc>>,
    /// Optional. Planned Composite Charging Schedule, the energy consumption over time. Always relative to ScheduleStart. If status is "Rejected", this field may be absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_schedule: Option<ChargingSchedule<N_CHARGING_SCHEDULE_PERIODS>>,
}
