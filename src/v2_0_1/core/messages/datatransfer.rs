use crate::v2_0_1::core::{
    datatypes::status_info_type::StatusInfoType,
    enumerations::data_transfer_status_enum_type::DataTransferStatusEnumType,
};

/// This contains the field definition of the DataTransferRequest PDU sent either by the CSMS to the Charging Station or vice versa.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    pub data: String,
    pub vendor_id: String,
}

/// This contains the field definition of the DataTransferResponse PDU sent by the Charging Station to the CSMS or vice versa in response to a DataTransferRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse {
    pub status: DataTransferStatusEnumType,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
