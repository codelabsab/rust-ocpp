use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, MessageInfoType};

/// Request to notify the CSMS about display messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesRequest {
    /// Required. Id of this request for identification in response.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Optional. "to be continued" indicator. Indicates whether another part of the report follows in an upcoming notifyDisplayMessagesRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Required. Array of message info objects.
    #[validate(length(min = 1))]
    pub message_info: Vec<MessageInfoType>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a NotifyDisplayMessagesRequest. This message has no fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
