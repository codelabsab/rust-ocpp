use crate::v2_0_1::enumerations::publish_firmware_status_enum_type::PublishFirmwareStatusEnumType;
use crate::Vec;

/// This contains the field definition of the PublishFirmwareStatusNotificationRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationRequest<'a, const N_LOCATIONS: usize> {
    pub status: PublishFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub location: Option<Vec<&'a str, N_LOCATIONS>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
}

/// This contains the field definition of the PublishFirmwareStatusNotificationResponse PDU sent by the CSMS to the Charging station in response to a PublishFirmwareStatusNotificationRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationResponse {}
