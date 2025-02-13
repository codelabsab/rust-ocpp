use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::HashAlgorithmEnumType;

/// OCSP request data for a certificate.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The hash algorithm used to calculate HashValue.
    pub hash_algorithm: HashAlgorithmEnumType,

    /// Required. The hash value of the Issuer DN (Distinguished Name).
    #[validate(length(max = 128))]
    pub issuer_name_hash: String,

    /// Required. The hash value of the Issuer Public Key.
    #[validate(length(max = 128))]
    pub issuer_key_hash: String,

    /// Required. The serial number of the certificate.
    #[validate(length(max = 40))]
    pub serial_number: String,

    /// Required. This contains the responder URL (Case insensitive).
    #[validate(length(max = 512))]
    pub responder_url: String,
}
