use crate::v2_0_1::core::enumerations::unpublish_firmware_status_enum_type::UnpublishFirmwareStatusEnumType;

/// This contains the field definition of the UnpublishFirmwareRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareRequest {
    pub checksum: String,
}

/// This contains the field definition of the UnpublishFirmwareResponse PDU sent by the Charging Station to the CSMS in response to a UnpublishFirmwareRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareResponse {
    pub status: UnpublishFirmwareStatusEnumType,
}
