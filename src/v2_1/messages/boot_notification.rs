use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{ChargingStationType, CustomDataType, StatusInfoType},
    enumerations::{BootReasonEnumType, RegistrationStatusEnumType},
};

/// Request body for the BootNotification request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    /// Required. The physical system where an Electrical Vehicle (EV) can be charged.
    pub charging_station: ChargingStationType,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains the reason for sending this message to the CSMS.
    pub reason: BootReasonEnumType,
}

/// Response body for the BootNotification response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains the CSMS's current time.
    pub current_time: DateTime<Utc>,

    /// Required. When Status is Accepted, this contains the heartbeat interval in seconds.
    /// If the CSMS returns something other than Accepted, the value of the interval field
    /// indicates the minimum wait time before sending a next BootNotification request.
    #[validate(range(min = 0))]
    pub interval: i32,

    /// Required. This contains whether the Charging Station has been registered within the CSMS.
    pub status: RegistrationStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
