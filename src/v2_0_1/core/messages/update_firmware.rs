use crate::v2_0_1::core::{
    datatypes::{firmware_type::FirmwareType, status_info_type::StatusInfoType},
    enumerations::update_firmware_status_enum_type::UpdateFirmwareStatusEnumType,
};

/// This contains the field definition of the UpdateFirmwareRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
    pub request_id: i64,
    pub firmware: FirmwareType,
}

/// This contains the field definition of the UpdateFirmwareResponse PDU sent by the Charging Station to the CSMS in response to an UpdateFirmwareRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareResponse {
    pub status: UpdateFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
