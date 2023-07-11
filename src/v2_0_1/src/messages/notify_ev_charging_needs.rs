//! The Charging Station uses this message to communicate the charging needs as calculated by the EV to the CSMS.
use crate::datatypes::charging_needs_type::ChargingNeedsType;
use crate::datatypes::status_info_type::StatusInfoType;
use crate::enumerations::notify_ev_charging_needs_status_enum_type::NotifyEVChargingNeedsStatusEnumType;

/// The Charging Station uses this message to communicate the charging needs as calculated by the EV to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_schedule_tuples: Option<i64>,
    pub evse_id: i64,
    pub charging_needs: ChargingNeedsType,
}

/// Response to a [`NotifyEVChargingNeedsRequest`]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsResponse {
    pub status: NotifyEVChargingNeedsStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
