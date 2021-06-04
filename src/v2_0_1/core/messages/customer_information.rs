use crate::v2_0_1::core::{
    datatypes::{
        certificate_hash_data_type::CertificateHashDataType, id_token_type::IdTokenType,
        status_info_type::StatusInfoType,
    },
    enumerations::customer_information_status_enum_type::CustomerInformationStatusEnumType,
};

/// This contains the field definition of the CostUpdatedRequest PDU sent by the CSMS to the Charging Station. With this request the CSMS can send the current cost of a transaction to a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationRequest {
    pub request_id: i64,
    pub report: bool,
    pub clear: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_certificate: Option<CertificateHashDataType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationResponse {
    pub status: CustomerInformationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
