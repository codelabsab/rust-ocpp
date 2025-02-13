use crate::v2_1::datatypes::{
    certificate_hash_data::CertificateHashDataType, custom_data::CustomDataType,
};
use crate::v2_1::enumerations::get_certificate_id_use::GetCertificateIdUseEnumType;
use serde::{Deserialize, Serialize};

/// Certificate hash data chain for validating certificates through OCSP.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateHashDataChainType {
    /// Information to identify a certificate
    pub certificate_hash_data: CertificateHashDataType,

    /// Indicates the type of the requested certificate(s).
    pub certificate_type: GetCertificateIdUseEnumType,

    /// Information to identify the child certificate(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_certificate_hash_data: Option<Vec<CertificateHashDataType>>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
