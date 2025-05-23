use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, ChargingNeedsType, StatusInfoType},
    enumerations::NotifyEVChargingNeedsStatusEnumType,
};

/// Request to notify the CSMS about EV charging needs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsRequest {
    /// Optional. Contains the maximum schedule tuples the car supports per schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_schedule_tuples: Option<i32>,

    /// Required. Charging needs of the EV.
    pub charging_needs: ChargingNeedsType,

    /// Required. Defines the EVSE and connector to which the EV is connected. EvseId may not be 0.
    #[validate(range(min = 1))]
    pub evse_id: i32,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a NotifyEVChargingNeedsRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsResponse {
    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: NotifyEVChargingNeedsStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
