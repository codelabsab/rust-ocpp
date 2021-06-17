use crate::v2_0_1::core::{
    datatypes::{id_token_type::IdTokenType, ocsp_request_data_type::OCSPRequestDataType},
    enumerations::authorize_certificate_status_enum_type::AuthorizeCertificateStatusEnumType,
};

/// This contains the field definition of the AuthorizeRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    pub id_token: IdTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso1_5118_certificate_hash_data: Option<OCSPRequestDataType>,
}

/// This contains the field definition of the AuthorizeResponse PDU sent by the CSMS to the Charging Station in response to an AuthorizeRequest.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,
    pub id_token: IdTokenType,
}
