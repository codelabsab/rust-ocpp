use crate::v2_0_1::core::enumerations::hash_algorithm_enum_type::HashAlgorithmEnumType;

/// CertificateHashDataType is used by: Common:CertificateHashDataChainType , DeleteCertificateRequest , CustomerInformationRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataType {
    pub hash_algorithm: HashAlgorithmEnumType,
    pub issuer_name_hash: String,
    pub issuer_key_hash: String,
    pub serial_number: String,
}
