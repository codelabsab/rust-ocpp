use crate::v2_1::datatypes::{
    certificate_hash_data::CertificateHashDataType, custom_data::CustomDataType,
};
use crate::v2_1::enumerations::CertificateStatusSourceEnumType;
use serde::{Deserialize, Serialize};

/// Data necessary to request the revocation status of a certificate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateStatusRequestInfoType {
    /// Certificate hash data needed for validating certificates through OCSP.
    pub certificate_hash_data: CertificateHashDataType,

    /// Source of status: OCSP, CRL
    pub source: CertificateStatusSourceEnumType,

    /// URL(s) of _source_.
    pub urls: Vec<String>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
