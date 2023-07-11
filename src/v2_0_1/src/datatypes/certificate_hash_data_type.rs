use crate::enumerations::hash_algorithm_enum_type::HashAlgorithmEnumType;
use validator::Validate;

/// CertificateHashDataType is used by: Common:CertificateHashDataChainType , DeleteCertificateRequest , CustomerInformationRequest
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataType {
    /// Required. Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithmEnumType,
    /// Required. Hashed value of the Issuer DN (Distinguished Name).
    #[validate(length(min = 0, max = 128))]
    pub issuer_name_hash: String,
    /// Required. Hashed value of the issuers public key
    #[validate(length(min = 0, max = 128))]
    pub issuer_key_hash: String,
    /// Required. The serial number of the certificate.
    #[validate(length(min = 0, max = 40))]
    pub serial_number: String,
}
