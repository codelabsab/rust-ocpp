use super::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};

/// Contains additional information about an identifier. The format of the additionalIdToken is pending standardization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalInfoType {
    /// This field specifies the type of the additionalIdToken. The format of the additionalIdToken is pending standardization.
    pub additional_id_token: String,

    /// This defines the type of the additionalIdToken. This is a custom type, so the implementation needs to be agreed upon by all involved parties.
    #[serde(rename = "type")]
    pub type_: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
