use crate::v2_1::datatypes::custom_data::CustomDataType;
use crate::v2_1::enumerations::HashAlgorithmEnumType;
use serde::{Deserialize, Serialize};

/// Certificate hash data for validating certificates through OCSP.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateHashDataType {
    /// Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithmEnumType,

    /// The hash of the issuer's distinguished name (DN), that must be calculated over the DER
    /// encoding of the issuer's name field in the certificate being checked.
    pub issuer_name_hash: String,

    /// The hash of the DER encoded public key: the value (excluding tag and length) of the subject
    /// public key field in the issuer's certificate.
    pub issuer_key_hash: String,

    /// The string representation of the hexadecimal value of the serial number without the
    /// prefix "0x" and without leading zeroes.
    pub serial_number: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
