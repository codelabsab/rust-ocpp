use crate::v2_0_1::enumerations::firmware_status_enum_type::FirmwareStatusEnumType;

/// This contains the field definition of the FirmwareStatusNotifitacionRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    pub status: FirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
}

/// This contains the field definition of the FirmwareStatusNotificationResponse PDU sent by the CSMS to the Charging Station in response to a FirmwareStatusNotificationRequest. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationResponse {}
