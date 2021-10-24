use crate::v2_0_1::core::enumerations::publish_firmware_status_enum_type::PublishFirmwareStatusEnumType;

/// This contains the field definition of the PublishFirmwareStatusNotificationRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationRequest {
    pub status: PublishFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
}

/// This contains the field definition of the PublishFirmwareStatusNotificationResponse PDU sent by the CSMS to the Charging station in response to a PublishFirmwareStatusNotificationRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationResponse {}
