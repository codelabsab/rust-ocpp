use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::charging_schedule_type::ChargingScheduleType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::generic_status_enum_type::GenericStatusEnumType;
use crate::v2_0_1::helpers::serializer::datetime;

/// The Charging Station uses this message to communicate the charging needs as calculated by the EV to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleRequest {
    #[serde(with = "datetime")]
    pub time_base: DateTime<Utc>,
    pub evse_id: i32,
    pub charging_schedule: ChargingScheduleType,
}

/// Response to a NotifyEVChargingScheduleRequest message.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleResponse {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
