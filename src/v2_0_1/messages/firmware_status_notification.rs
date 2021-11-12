//! FirmwareStatusNotification
use crate::v2_0_1::enumerations::firmware_status_enum_type::FirmwareStatusEnumType;

/// FirmwareStatusNotifitacionRequest, sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    /// This contains the progress status of the firmware installation.
    pub status: FirmwareStatusEnumType,
    /// The request id that was provided in the UpdateFirmwareRequest that started this firmware update.
    /// This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no firmware update ongoing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
}

/// FirmwareStatusNotificationResponse, sent by the CSMS to the Charging Station in response to a FirmwareStatusNotificationRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationResponse {
    // No fields are defined.
}
