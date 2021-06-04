use crate::v2_0_1::core::{
    datatypes::status_info_type::StatusInfoType,
    enumerations::generic_status_enum_type::GenericStatusEnumType,
};

/// This contains the field definition of the PublishFirmwareRequest PDU sent by the CSMS to the Local Controller.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareRequest {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    pub checksum: String,
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
}

/// This contains the field definition of the PublishFirmwareResponse PDU sent by the Local Controller to the CSMS in response to a PublishFirmwareRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareResponse {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
