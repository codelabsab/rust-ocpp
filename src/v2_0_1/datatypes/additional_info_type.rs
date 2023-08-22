//! Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
use crate::v2_0_1::helpers::validator::validate_identifier_string;
use validator::Validate;

/// Contains a case insensitive identifier to use for the authorization and the
/// type of authorization to support multiple forms of identifiers.
///
/// AdditionalInfoType is used by: [IdTokenType](`crate::v2_0_1::datatypes::id_token_type::IdTokenType`)
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfoType<'a> {
    /// This field specifies the additional IdToken
    #[validate(length(min = 0, max = 36), custom = "validate_identifier_string")]
    pub additional_id_token: &'a str,
    /// This defines the type of the additionalIdToken. This is a custom type, so the implementation needs to be agreed upon by all involved parties.
    #[validate(length(min = 0, max = 50))]
    #[serde(rename = "type")]
    pub kind: &'a str,
}
