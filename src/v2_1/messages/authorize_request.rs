use super::super::datatypes::{CustomDataType, IdTokenType};
use serde::{Deserialize, Serialize};

/// Request to start a transaction with the given idToken.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorizeRequest {
    /// The X.509 certificate chain presented by EV and encoded in PEM format.
    /// Order of certificates in chain is from leaf up to (but excluding) root certificate.
    /// Only needed in case of central contract validation when Charging Station cannot validate the contract certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    /// Contains the identifier that needs to be authorized.
    pub id_token: IdTokenType,

    /// Optional list of OCSP request data for certificates that need to be validated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso15118_certificate_hash_data: Option<Vec<OCSPRequestDataType>>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Information about a certificate for an OCSP check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OCSPRequestDataType {
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

    /// This contains the responder URL (Case insensitive).
    pub responder_url: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Used algorithms for the hashes provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HashAlgorithmEnumType {
    #[serde(rename = "SHA256")]
    SHA256,
    #[serde(rename = "SHA384")]
    SHA384,
    #[serde(rename = "SHA512")]
    SHA512,
}
