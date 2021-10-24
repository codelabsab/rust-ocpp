use crate::v2_0_1::enumerations::upload_log_status_enum_type::UploadLogStatusEnumType;

/// This contains the field definition of the LogStatusNotificationRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationRequest {
    pub status: UploadLogStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
}

/// This contains the field definition of the LogStatusNotificationResponse PDU sent by the CSMS to the Charging Station in response to LogStatusNotificationRequest. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationResponse {}
