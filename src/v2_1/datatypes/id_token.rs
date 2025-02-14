use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{additional_info::AdditionalInfoType, custom_data::CustomDataType};

/// Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType {
    /// Optional. Additional information about the identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub additional_info: Option<Vec<AdditionalInfoType>>,

    /// Required. IdToken is case insensitive. Might hold the hidden id of an RFID tag, but can for example also contain a UUID.
    #[validate(length(max = 255))]
    pub id_token: String,

    /// Required. Type of identification used to authorize charging.
    /// Allowed values: "Central", "DirectPayment", "eMAID", "EVCCID", "ISO14443", "ISO15693",
    /// "KeyCode", "Local", "MacAddress", "NoAuthorization", "VIN"
    #[validate(length(max = 20))]
    pub r#type: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
