use crate::v2_1::datatypes::{
    certificate_hash_data::CertificateHashDataType, custom_data::CustomDataType,
};
use crate::v2_1::enumerations::{CertificateStatusEnumType, CertificateStatusSourceEnumType};
use serde::{Deserialize, Serialize};

/// Revocation status of certificate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateStatusType {
    /// Certificate hash data needed for validating certificates through OCSP.
    pub certificate_hash_data: CertificateHashDataType,

    /// Source of status: OCSP, CRL
    pub source: CertificateStatusSourceEnumType,

    /// Status of certificate: good, revoked or unknown.
    pub status: CertificateStatusEnumType,

    /// The date and time at which the next update of the certificate status MAY be expected.
    pub next_update: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
