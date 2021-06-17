use std::fmt;

use chrono::{DateTime, Utc};

use crate::v2_0_1::core::{
    datatypes::{charging_station_type::ChargingStationType, status_info_type::StatusInfoType},
    enumerations::{
        boot_reason_enum_type::BootReasonEnumType,
        registration_status_enum_type::RegistrationStatusEnumType,
    },
};

/// BootNotificationRequest PDU sent by the Charging Station to the CSMS
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub reason: BootReasonEnumType,
    pub charging_station: ChargingStationType,
}

/// PDU sent by the CSMS to the Charging Station in response to a BootNotificationRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    pub current_time: DateTime<Utc>,
    pub interval: u16,
    pub status: RegistrationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl fmt::Display for BootNotificationRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for BootNotificationResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
