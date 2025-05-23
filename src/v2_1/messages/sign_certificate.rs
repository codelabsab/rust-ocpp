use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    certificate_hash_data::CertificateHashDataType, custom_data::CustomDataType,
    status_info::StatusInfoType,
};
use crate::v2_1::enumerations::{
    certificate_signing_use::CertificateSigningUseEnumType, generic_status::GenericStatusEnumType,
};

/// Request to sign a certificate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateRequest {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The Charging Station SHALL send the public key in form of a Certificate
    /// Signing Request (CSR) as described in RFC 2986 and then PEM encoded.
    #[validate(length(max = 5500))]
    pub csr: String,

    /// Optional. Indicates the type of certificate that is to be signed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,

    /// Optional. Hash of the root certificate to be installed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_root_certificate: Option<CertificateHashDataType>,

    /// Optional. RequestId to match this message with the CertificateSignedRequest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

/// Response to a SignCertificateRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Returns whether the CSMS can process the request.
    pub status: GenericStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl SignCertificateRequest {
    /// Creates a new `SignCertificateRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `csr` - The Certificate Signing Request (CSR) as described in RFC 2986 and then PEM encoded
    ///
    /// # Returns
    ///
    /// A new instance of `SignCertificateRequest` with optional fields set to `None`
    pub fn new(csr: String) -> Self {
        Self {
            custom_data: None,
            csr,
            certificate_type: None,
            hash_root_certificate: None,
            request_id: None,
        }
    }
}

impl SignCertificateResponse {
    /// Creates a new `SignCertificateResponse` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Returns whether the CSMS can process the request
    ///
    /// # Returns
    ///
    /// A new instance of `SignCertificateResponse` with optional fields set to `None`
    pub fn new(status: GenericStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
