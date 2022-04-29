use crate::v2_0_1::enumerations::hash_algorithm_enum_type::HashAlgorithmEnumType;
use validator::Validate;

/// OCSPRequestDataType is used by: AuthorizeRequest , GetCertificateStatusRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    pub hash_algorithm: HashAlgorithmEnumType,
    #[validate(length(min = 0, max = 128))]
    pub issuer_name_hash: String,
    #[validate(length(min = 0, max = 128))]
    pub issuer_key_hash: String,
    #[validate(length(min = 0, max = 40))]
    pub serial_number: String,
    #[validate(length(min = 0, max = 512))]
    #[serde(rename = "responderURL")]
    pub responder_url: String,
}
