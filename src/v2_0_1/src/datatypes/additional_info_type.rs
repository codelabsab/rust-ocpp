//! Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
use crate::helpers::validator::validate_identifier_string;
use validator::Validate;

/// Contains a case insensitive identifier to use for the authorization and the
/// type of authorization to support multiple forms of identifiers.
///
/// AdditionalInfoType is used by: [IdTokenType](`crate::datatypes::id_token_type::IdTokenType`)
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfoType {
    /// This field specifies the additional IdToken
    #[validate(length(min = 0, max = 36), custom = "validate_identifier_string")]
    pub additional_id_token: String,
    /// This defines the type of the additionalIdToken. This is a custom type, so the implementation needs to be agreed upon by all involved parties.
    #[validate(length(min = 0, max = 50))]
    #[serde(rename = "type")]
    pub kind: String,
}
