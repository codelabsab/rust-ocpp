use crate::v2_0_1::enumerations::hash_algorithm_enum_type::HashAlgorithmEnumType;
#[cfg(feature = "std")]
use validator::Validate;

/// OCSPRequestDataType is used by: AuthorizeRequest , GetCertificateStatusRequest
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType<'a> {
    pub hash_algorithm: HashAlgorithmEnumType,
    #[cfg_attr(feature="std", validate(length(min = 0, max = 128)))]
    pub issuer_name_hash: &'a str,
    #[cfg_attr(feature="std", validate(length(min = 0, max = 128)))]
    pub issuer_key_hash: &'a str,
    #[cfg_attr(feature="std", validate(length(min = 0, max = 40)))]
    pub serial_number: &'a str,
    #[cfg_attr(feature="std", validate(length(min = 0, max = 512)))]
    #[serde(rename = "responderURL")]
    pub responder_url: &'a str,
}
