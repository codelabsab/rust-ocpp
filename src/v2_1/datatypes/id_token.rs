use super::additional_info::AdditionalInfoType;
use super::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};

/// Contains a case insensitive identifier to use for the authorization and the type of authorization
/// to support multiple forms of identifiers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdTokenType {
    /// IdToken is case insensitive. Might hold the hidden id of an RFID tag,
    /// but can for example also contain a UUID.
    pub id_token: String,

    /// Enumeration of possible idToken types.
    /// Values defined in Appendix as IdTokenEnumStringType.
    #[serde(rename = "type")]
    pub type_: String,

    /// Optional list of additional information about the identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Vec<AdditionalInfoType>>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
