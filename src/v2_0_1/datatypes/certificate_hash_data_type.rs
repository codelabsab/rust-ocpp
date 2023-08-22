use crate::v2_0_1::enumerations::hash_algorithm_enum_type::HashAlgorithmEnumType;
use validator::Validate;

/// CertificateHashDataType is used by: Common:CertificateHashDataChainType , DeleteCertificateRequest , CustomerInformationRequest
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataType<'a> {
    /// Required. Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithmEnumType,
    /// Required. Hashed value of the Issuer DN (Distinguished Name).
    #[validate(length(min = 0, max = 128))]
    pub issuer_name_hash: &'a str,
    /// Required. Hashed value of the issuers public key
    #[validate(length(min = 0, max = 128))]
    pub issuer_key_hash: &'a str,
    /// Required. The serial number of the certificate.
    #[validate(length(min = 0, max = 40))]
    pub serial_number: &'a str,
}
