use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::generic_status_enum_type::GenericStatusEnumType;

/// This contains the field definition of the PublishFirmwareRequest PDU sent by the CSMS to the Local Controller.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareRequest<'a> {
    pub location: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    pub checksum: &'a str,
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
}

/// This contains the field definition of the PublishFirmwareResponse PDU sent by the Local Controller to the CSMS in response to a PublishFirmwareRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareResponse<'a> {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub status_info: Option<StatusInfoType<'a>>,
}
