//! LogStatusNotification
use crate::v2_0_1::enumerations::upload_log_status_enum_type::UploadLogStatusEnumType;

/// LogStatusNotificationRequest, sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationRequest {
    /// This contains the status of the log upload.
    pub status: UploadLogStatusEnumType,
    /// The request id that was provided in GetLogRequest that started this log upload. This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no log upload ongoing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
}

/// LogStatusNotificationResponse, sent by the CSMS to the Charging Station in response to LogStatusNotificationRequest. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationResponse {
    // No fields are defined.
}
