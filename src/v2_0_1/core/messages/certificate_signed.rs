use crate::v2_0_1::core::{
    datatypes::status_info_type::StatusInfoType,
    enumerations::{
        certificate_signed_status_enum_type::CertificateSignedStatusEnumType,
        certificate_signing_use_enum_type::CertificateSigningUseEnumType,
    },
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedRequest {
    pub certificate_chain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedResponse {
    pub status: CertificateSignedStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
