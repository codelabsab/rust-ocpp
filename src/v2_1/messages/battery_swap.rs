use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    custom_data::CustomDataType,
    id_token::IdTokenType,
    battery_data::BatteryDataType,
};
use crate::v2_1::enumerations::battery_swap_event::BatterySwapEventEnumType;

/// Request body for the BatterySwap request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapRequest {
    /// Required. Array of battery data.
    #[validate(length(min = 1))]
    pub battery_data: Vec<BatteryDataType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Type of battery swap event.
    pub event_type: BatterySwapEventEnumType,

    /// Required. Contains the identifier that needs to be authorized.
    pub id_token: IdTokenType,

    /// Required. RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    pub request_id: i32,
}

/// Response body for the BatterySwap response.
/// This is an empty response that just acknowledges receipt of the request. (The request cannot be rejected).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
