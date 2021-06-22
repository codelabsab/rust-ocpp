use chrono::{DateTime, Utc};

use crate::v2_0_1::core::{
    datatypes::{charging_schedule_type::ChargingScheduleType, status_info_type::StatusInfoType},
    enumerations::generic_status_enum_type::GenericStatusEnumType,
};

/// The Charging Station uses this message to communicate the charging needs as calculated by the EV to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleRequest {
    pub time_base: DateTime<Utc>,
    pub evse_id: i64,
    pub charging_schedule: ChargingScheduleType,
}

/// Response to a NotifyEVChargingScheduleRequest message.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleResponse {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
