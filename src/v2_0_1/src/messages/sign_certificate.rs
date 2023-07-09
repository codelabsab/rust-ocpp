use crate::datatypes::status_info_type::StatusInfoType;
use crate::enumerations::certificate_signing_use_enum_type::CertificateSigningUseEnumType;
use crate::enumerations::generic_status_enum_type::GenericStatusEnumType;

/// Sent by the Charging Station to the CSMS to request that the Certificate Authority signs the public key into a certificate.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateRequest {
    pub csr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
}

/// Sent by the CSMS to the Charging Station in response to the SignCertificateRequest message.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateResponse {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
