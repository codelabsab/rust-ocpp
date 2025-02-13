use super::super::datatypes::CustomDataType;
use super::super::enumerations::CertificateSigningUseEnumType;
use serde::{Deserialize, Serialize};

/// Request to inform the Charging Station about the signing of a certificate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateSignedRequest {
    /// The signed PEM encoded X.509 certificate. This SHALL also contain the necessary sub CA certificates,
    /// when applicable. The order of the bundle follows the certificate chain, starting from the leaf certificate.
    pub certificate_chain: String,

    /// Indicates the type of the signed certificate that is returned. When omitted the certificate is used
    /// for both the 15118 connection (if implemented) and the Charging Station to CSMS connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,

    /// RequestId to correlate this message with the SignCertificateRequest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
