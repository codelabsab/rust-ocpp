use crate::v2_0_1::core::enumerations::hash_algorithm_enum_type::HashAlgorithmEnumType;

/// OCSPRequestDataType is used by: AuthorizeRequest , GetCertificateStatusRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    pub hash_algorithm: HashAlgorithmEnumType,
    pub issuer_name_hash: String,
    pub issuer_key_hash: String,
    pub serial_number: i64,
    pub responder_url: i64,
}
