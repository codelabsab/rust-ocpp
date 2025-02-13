use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::MessageFormatEnumType;

/// Contains message details, for a message to be displayed on a Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageContentType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Message contents.
    #[validate(length(max = 512))]
    pub content: String,

    /// Required. Format of the message.
    pub format: MessageFormatEnumType,

    /// Required. Language identifier of the message content.
    #[validate(length(max = 8))]
    pub language: String,
}
