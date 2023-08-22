use validator::Validate;

use crate::v1_6::types::DataTransferStatus;

#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest<'a> {
    #[serde(rename = "vendorId")]
    pub vendor_string: &'a str,
    #[validate(length(min = 1, max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<&'a str>,
    #[validate(length(min = 1, max = 255))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<&'a str>,
}

#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse<'a> {
    /// Required. This indicates the success or failure of the data transfer.
    pub status: DataTransferStatus,
    /// Optional. Data in response to request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<&'a str>,
}
