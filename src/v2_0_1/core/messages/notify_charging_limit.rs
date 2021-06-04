use crate::v2_0_1::core::datatypes::{
    charging_limit_type::ChargingLimitType, charging_schedule_type::ChargingScheduleType,
};

/// The message NotifyChargingLimitRequest can be used to communicate a charging limit, set by an external system on the Charging Station (Not installed by the CSO via SetChargingProfileRequest), to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i64>,
    pub charging_limit: ChargingLimitType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_schedule: Option<ChargingScheduleType>,
}

/// The NotifyChargingLimitResponse message is sent by the CSMS to the Charging Station in response to a NotifyChargingLimitsRequest. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitResponse {}
