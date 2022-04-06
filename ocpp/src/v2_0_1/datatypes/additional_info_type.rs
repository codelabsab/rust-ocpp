//! Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
use validator::Validate;

/// Contains a case insensitive identifier to use for the authorization and the
/// type of authorization to support multiple forms of identifiers.
///
/// AdditionalInfoType is used by: [IdTokenType](`crate::v2_0_1::datatypes::id_token_type::IdTokenType`)
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfoType {
    /// This field specifies the additional IdToken
    #[validate(length(min = 0, max = 36))]
    pub additional_id_token: String,
    /// This defines the type of the additionalIdToken. This is a custom type, so the implementation needs to be agreed upon by all involved parties.
    #[validate(length(min = 0, max = 50))]
    #[serde(rename = "type")]
    pub kind: String,
}
